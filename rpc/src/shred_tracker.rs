///A shred tracking service to track shreds from the sigverify_stage
use std::{
    // collections::HashSet,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    },
    thread::{self, Builder, JoinHandle},
    time::Duration,
};
use crossbeam_channel::{Receiver, Sender, RecvTimeoutError};
use solana_rayon_threadlimit::get_thread_count;
use rayon::{ThreadPool, ThreadPoolBuilder ,prelude::{IntoParallelRefIterator, ParallelIterator}};
use solana_perf::packet::{PacketBatch, Packet};
use solana_ledger::{
    // blockstore::Blockstore, 
    shred::{Slot, self}, 
    // blockstore_db::columns::{ShredData, ShredCode}
};
use solana_ledger::shred::Shred;
use solana_ledger::shred::*;


// use crate::rpc_subscriptions::RpcSubscriptions;
// use crate::{
//     repair_response,
//     repair_service::{OutstandingShredRepairs, RepairInfo, RepairService},
//     result::{Error, Result},
// };

#[derive(Clone, Debug)]
pub enum ShredNotification{
    DataShred(Shred),
    CodingShred(Shred),
}

pub type ShredNotificationReceiver = Receiver<ShredNotification>;
pub type ShredNotificationSender = Sender<ShredNotification>;

pub struct ShredTracker{
    thread_hdl: JoinHandle<()>,
}

impl ShredTracker {
    pub fn new(
        shred_packet_receiver: Receiver<Vec<PacketBatch>>,
        exit: &Arc<AtomicBool>,
       // subscriptions: Arc<RpcSubscriptions>,
        shred_filter_slot: Option<Slot>,
        shred_notification_subscribers: Option<Arc<RwLock<Vec<ShredNotificationSender>>>>,
    ) -> Self {
        let exit_ = exit.clone();
        let num_threads = get_thread_count().min(8);
        let thread_pool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .thread_name(|i| format!("shredTracket101"))
        .build()
        .unwrap();
        let thread_hdl =  Builder::new()
                .name("solShredTracker".to_string())
                .spawn(move || loop{
                    if exit_.load(Ordering::Relaxed) {
                        break;
                    }
                    if let Err(RecvTimeoutError::Disconnected) = Self::receive_and_process_shreds(
                        &shred_packet_receiver,
                        &thread_pool, 
                        shred_filter_slot,
                         &shred_notification_subscribers,
                        ){
                            break;
                        } 
                }).unwrap();
        Self { thread_hdl }
    }

    fn receive_and_process_shreds(
        shred_notification_receiver: &Receiver<Vec<PacketBatch>>,
       // subscriptions: &Arc<RpcSubscriptions>,
       thread_pool: &ThreadPool,
       shred_filter_slot: Option<Slot>,
       shred_notification_subscribers: &Option<Arc<RwLock<Vec<ShredNotificationSender>>>>,
    ) ->Result<(), RecvTimeoutError>{
        let mut processed_packets = shred_notification_receiver.recv_timeout(Duration::from_millis(100))?;
        processed_packets.extend(shred_notification_receiver.try_iter().flatten());
        let handle_packet = |packet: &Packet| {
            // ignore packets marked with discard or repair.
            if packet.meta().discard() | packet.meta().repair() {
                return None;
            }
            let shred = shred::layout::get_shred(packet)?;
            let shred = Shred::new_from_serialized_shred(shred.to_vec()).ok()?;
            Some(shred)
        };
        let shreds: Vec<_> = thread_pool.install(|| {
            processed_packets
            .par_iter()
            .flat_map_iter(|packets| packets.iter().filter_map(handle_packet))
            .collect()
        });

        if let Some(filter_slot) = shred_filter_slot{
            let filtered_shreds = filter_shreds_by_slot(filter_slot, shreds);
            let _ = filtered_shreds.iter().map(|shred|{
                match shred{
                    Shred::ShredData(_) => {
                      let data_shred_notif =  ShredNotification::DataShred(shred.clone());
                      if let Some(shred_notification_subscribers) = shred_notification_subscribers{
                        for sender in shred_notification_subscribers.read().unwrap().iter(){
                            match sender.send(data_shred_notif.clone()){
                                Ok(_) => { 
                                    info!("Received Shreds for Slot {:?}", filter_slot)
                                },
                                Err(err) => {
                                    info!(
                                        "Failed to send notification {:?}, error: {:?}",
                                        data_shred_notif, err
                                    )
                                }
                            }
                        }
                      }
                    },
                    Shred::ShredCode(_) => { 
                      let code_shred_notif =  ShredNotification::CodingShred(shred.clone());
                      if let Some(shred_notification_subscribers) = shred_notification_subscribers{
                        for sender in shred_notification_subscribers.read().unwrap().iter(){
                            match sender.send(code_shred_notif.clone()){
                                Ok(_) => {},
                                Err(err) => {
                                    info!(
                                        "Failed to send notification {:?}, error: {:?}",
                                        code_shred_notif, err
                                    )
                                }
                            }
                        }
                      }
                    },
                }
            });
        } else {
            let _ =  shreds.iter().map(|shred|{
                match shred{
                    Shred::ShredData(_) => {
                      let data_shred_notif =  ShredNotification::DataShred(shred.clone());
                      if let Some(shred_notification_subscribers) = shred_notification_subscribers{
                        for sender in shred_notification_subscribers.read().unwrap().iter(){
                            match sender.send(data_shred_notif.clone()){
                                Ok(_) => {},
                                Err(err) => {
                                    info!(
                                        "Failed to send notification {:?}, error: {:?}",
                                        data_shred_notif, err
                                    )
                                }
                            }
                        }
                      }
                    },
                    Shred::ShredCode(_) => { 
                      let code_shred_notif =  ShredNotification::CodingShred(shred.clone());
                      if let Some(shred_notification_subscribers) = shred_notification_subscribers{
                        for sender in shred_notification_subscribers.read().unwrap().iter(){
                            match sender.send(code_shred_notif.clone()){
                                Ok(_) => {},
                                Err(err) => {
                                    info!(
                                        "Failed to send notification {:?}, error: {:?}",
                                        code_shred_notif, err
                                    )
                                }
                            }
                        }
                      }
                    },
                }
            });
        }
        
        
        Ok(())
    }
    pub fn close(self) -> thread::Result<()> {
        self.join()
    }
    pub fn join(self) -> thread::Result<()> {
        self.thread_hdl.join()
    }

}


fn filter_shreds_by_slot(
    required_slot: Slot,
    incoming_shreds: Vec<Shred>,
) -> Vec<Shred>{
    let required_shreds: Vec<Shred> = incoming_shreds.into_iter().filter(|shred| shred.slot() == required_slot).collect();
    required_shreds
}
