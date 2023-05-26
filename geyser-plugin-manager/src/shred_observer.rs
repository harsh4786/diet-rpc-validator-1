use log::info;

use {
    solana_rpc::shred_tracker::{
        ShredNotification,
        ShredNotificationReceiver,
        ShredNotificationSender,
    },
    crossbeam_channel::Receiver,
    std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        thread::{self, Builder, JoinHandle},
    },
    crate::shred_notifier::{
        ShredNotifierLock,
    },
};


#[derive(Debug)]
pub struct ShredObserver{
    shred_receiver_service: Option<JoinHandle<()>>,
    exit_server: Arc<AtomicBool>,
}

impl ShredObserver {
    pub fn new(shred_notification: ShredNotification, ) -> Self{
        let exit_server = Arc::new(AtomicBool::new(false));

    }

    fn run_shred_notification_receiver(
        shred_notification_receiver: ShredNotificationReceiver,
        exit: Arc<AtomicBool>,
        shred_notifier: ShredNotifierLock,
    ) -> JoinHandle<()>{
       Builder::new()
                .name("solShredObserver".to_string())
                .spawn(move|| {
                    while !exit.load(Ordering::Relaxed) {
                        if let Ok(shred_notification) = shred_notification_receiver.recv(){
                            match shred_notification{
                                ShredNotification::DataShred(s) => {
                                    info!("Received data shred for slot {}", s.slot())
                                    //....
                                },
                                ShredNotification::CodingShred(s) =>{
                                    info!("Received code shred for slot {}", s.slot())
                                    //....
                                }
                            }
                        }
                    }
                }).unwrap()
    }
}