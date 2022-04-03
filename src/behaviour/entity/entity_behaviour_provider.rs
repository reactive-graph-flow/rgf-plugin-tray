use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::debug;
use uuid::Uuid;

use crate::behaviour::entity::tray::Tray;
use crate::behaviour::entity::tray::TRAY;
use crate::di::*;
use crate::model::ReactiveEntityInstance;
use crate::model::ReactiveRelationInstance;
use crate::plugins::EntityBehaviourProvider;

#[wrapper]
pub struct TrayStorage(RwLock<HashMap<Uuid, Arc<Tray>>>);

#[provides]
fn create_tray_storage() -> TrayStorage {
    TrayStorage(RwLock::new(HashMap::new()))
}

#[async_trait]
pub trait TrayEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_tray(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_tray(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);

    fn add_menu_item(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_menu_item(&self, relation_instance: Arc<ReactiveRelationInstance>);
}

#[component]
pub struct TrayEntityBehaviourProviderImpl {
    tray: TrayStorage,
}

interfaces!(TrayEntityBehaviourProviderImpl: dyn EntityBehaviourProvider);

impl TrayEntityBehaviourProviderImpl {}

#[async_trait]
#[provides]
impl TrayEntityBehaviourProvider for TrayEntityBehaviourProviderImpl {
    fn create_tray(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let device_key = Tray::new(entity_instance.clone());
        if device_key.is_ok() {
            let tray = Arc::new(device_key.unwrap());
            self.tray.0.write().unwrap().insert(id, tray);
            entity_instance.add_behaviour(TRAY);
            debug!("Added behaviour {} to entity instance {}", TRAY, id);
        }
    }

    fn remove_tray(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if let Some(_) = self.tray.0.write().unwrap().remove(&entity_instance.id) {
            entity_instance.remove_behaviour(TRAY);
            debug!("Removed behaviour {} from entity instance {}", TRAY, entity_instance.id);
        }
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.tray.0.write().unwrap().contains_key(&id) {
            if let Some(_) = self.tray.0.write().unwrap().remove(&id) {
                debug!("Removed behaviour {} from entity instance {}", TRAY, id);
            }
        }
    }

    fn add_menu_item(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        if let Some(tray) = self.tray.0.write().unwrap().get(&relation_instance.outbound.id) {
            tray.add_menu_item(relation_instance.clone());
        }
    }

    fn remove_menu_item(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        if let Some(tray) = self.tray.0.write().unwrap().get(&relation_instance.outbound.id) {
            tray.remove_menu_item(relation_instance.clone());
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
