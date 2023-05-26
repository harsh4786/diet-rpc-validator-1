use solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaShredInfo;
use solana_ledger::blockstore::Blockstore;

use {
    crate::geyser_plugin_manager::GeyserPluginManager,
    log::*,
    solana_geyser_plugin_interface::geyser_plugin_interface::SlotStatus,
    solana_measure::measure::Measure,
    solana_metrics::*,
    solana_sdk::clock::Slot,
    std::sync::{Arc, RwLock},
};


pub trait ShredNotifierInterface {
     fn notify_shreds(&self);
     fn notify_shreds_for_slot(&self, slot: Slot);
}


pub type ShredNotifierLock = Arc<RwLock<dyn ShredNotifierInterface + Send + Sync>>;

pub struct ShredNotifierImpl{
    plugin_manager: Arc<RwLock<GeyserPluginManager>>,
}


impl ShredNotifierImpl{
    pub fn new(plugin_manager: Arc<RwLock<GeyserPluginManager>>) -> Self {
        Self { plugin_manager }
    }
    pub fn notify_shreds(&self ){
        let mut plugin_manager = self.plugin_manager.write().unwrap();
        if plugin_manager.plugins.is_empty() {
            return;
        }

        for plugin in  plugin_manager.plugins.iter_mut(){
            match plugin.
        }
    }

}

impl ShredNotifierInterface for ShredNotifierImpl{
    fn notify_shreds(&self) {
         //self.
    }
    fn notify_shreds_for_slot(&self, slot: Slot) {
        todo!()
    }
}

fn build_replica_shred_info(
    slot: Slot,
    
) -> ReplicaShredInfo{
    ReplicaShredInfo { slot: (), shred: () }
}