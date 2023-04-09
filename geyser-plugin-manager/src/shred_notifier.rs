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
     fn notify_shreds_for_slot(&self, slot: Slot);
}


pub type ShredNotifier = Arc<RwLock<dyn ShredNotifierInterface + Send + Sync>>;

pub struct ShredNotifierImpl{
    plugin_manager: Arc<RwLock<GeyserPluginManager>>,
}


impl ShredNotifierImpl{
    pub fn new(plugin_manager: Arc<RwLock<GeyserPluginManager>>) -> Self {
        Self { plugin_manager }
    }
    pub fn notify_shreds_for_slot(&self, slot: Slot){
        
    }

}