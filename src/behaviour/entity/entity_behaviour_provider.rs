use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use uuid::Uuid;
use waiter_di::*;

use crate::behaviour::entity::tray::Tray;
use crate::behaviour::entity::tray::TRAY;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

#[wrapper]
pub struct TrayStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<Tray>>>);

#[waiter_di::provides]
fn create_tray_storage() -> TrayStorage {
    TrayStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait TrayEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_tray(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_tray(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

pub struct TrayEntityBehaviourProviderImpl {
    tray: TrayStorage,
}

interfaces!(TrayEntityBehaviourProviderImpl: dyn EntityBehaviourProvider);

#[component]
impl TrayEntityBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self { tray: create_tray_storage() }
    }
}

#[async_trait]
#[provides]
impl TrayEntityBehaviourProvider for TrayEntityBehaviourProviderImpl {
    fn create_tray(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let device_key = Tray::new(entity_instance);
        if device_key.is_ok() {
            let tray = Arc::new(device_key.unwrap());
            self.tray.0.write().unwrap().insert(id, tray);
            debug!("Added behaviour {} to entity instance {}", TRAY, id);
        }
    }

    fn remove_tray(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.tray.0.write().unwrap().remove(&entity_instance.id);
        debug!("Removed behaviour {} from entity instance {}", TRAY, entity_instance.id);
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.tray.0.write().unwrap().contains_key(&id) {
            self.tray.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", TRAY, id);
        }
    }
}

impl EntityBehaviourProvider for TrayEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            TRAY => self.create_tray(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            TRAY => self.remove_tray(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
