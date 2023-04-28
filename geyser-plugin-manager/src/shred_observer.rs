use {
    solana_rpc::shred_tracker::*,
    crossbeam_channel::Receiver,
    std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        thread::{self, Builder, JoinHandle},
    },
};


#[derive(Debug)]
pub struct ShredObserver{
    shred_receiver_service: Option<JoinHandle<()>>,
    exit_updated_slot_server: Arc<AtomicBool>,
}