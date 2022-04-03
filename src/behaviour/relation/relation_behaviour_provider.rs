use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use crate::behaviour::entity::entity_behaviour_provider::{TrayEntityBehaviourProvider, TrayEntityBehaviourProviderImpl};
use crate::di::*;
use async_trait::async_trait;
use indradb::EdgeKey;
use log::debug;

use crate::behaviour::relation::tray_has_menu_item::TrayHasMenuItem;
use crate::model::ReactiveRelationInstance;
use crate::plugins::RelationBehaviourProvider;

const TRAY_HAS_MENU_ITEM: &str = "tray_has_menu_item";

#[wrapper]
pub struct TrayHasMenuItemRelationBehaviourStorage(RwLock<HashMap<EdgeKey, Arc<TrayHasMenuItem>>>);

#[provides]
fn create_tray_has_menu_item_relation_behaviour_storage() -> TrayHasMenuItemRelationBehaviourStorage {
    TrayHasMenuItemRelationBehaviourStorage(RwLock::new(HashMap::new()))
}

#[async_trait]
pub trait TrayRelationBehaviourProvider: RelationBehaviourProvider + Send + Sync {
    fn create_tray_has_menu_item_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_tray_has_menu_item_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_by_key(&self, edge_key: EdgeKey);
}

#[component]
pub struct TrayRelationBehaviourProviderImpl {
    entity_behaviour_provider: Wrc<TrayEntityBehaviourProviderImpl>,

    tray_has_menu_item_relation_behaviour: TrayHasMenuItemRelationBehaviourStorage,
}

interfaces!(TrayRelationBehaviourProviderImpl: dyn RelationBehaviourProvider);

impl TrayRelationBehaviourProviderImpl {}

#[async_trait]
#[provides]
impl TrayRelationBehaviourProvider for TrayRelationBehaviourProviderImpl {
    fn create_tray_has_menu_item_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        let tray_has_menu_item = Arc::new(TrayHasMenuItem::new(relation_instance.clone()));
        self.tray_has_menu_item_relation_behaviour
            .0
            .write()
            .unwrap()
            .insert(edge_key.clone(), tray_has_menu_item);
        relation_instance.add_behaviour(TRAY_HAS_MENU_ITEM);
        debug!("Added behaviour {} to relation instance {:?}", TRAY_HAS_MENU_ITEM, edge_key);
        self.entity_behaviour_provider.add_menu_item(relation_instance);
    }

    fn remove_tray_has_menu_item_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        self.entity_behaviour_provider.remove_menu_item(relation_instance.clone());
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        if let Some(_) = self.tray_has_menu_item_relation_behaviour.0.write().unwrap().remove(&edge_key) {
            relation_instance.remove_behaviour(TRAY_HAS_MENU_ITEM);
            debug!("Removed behaviour {} from relation instance {:?}", TRAY_HAS_MENU_ITEM, edge_key);
        }
    }

    fn remove_by_key(&self, edge_key: EdgeKey) {
        if self.tray_has_menu_item_relation_behaviour.0.write().unwrap().contains_key(&edge_key) {
            if let Some(tray_has_menu_item) = self.tray_has_menu_item_relation_behaviour.0.write().unwrap().remove(&edge_key) {
                debug!("Removed behaviour {} from relation instance {:?}", TRAY_HAS_MENU_ITEM, edge_key);
                self.entity_behaviour_provider.remove_menu_item(tray_has_menu_item.relation.clone());
            }
        }
    }
}

impl RelationBehaviourProvider for TrayRelationBehaviourProviderImpl {
    fn add_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        match relation_instance.clone().type_name.as_str() {
            TRAY_HAS_MENU_ITEM => self.create_tray_has_menu_item_behaviour(relation_instance),
            _ => {}
        }
    }

    fn remove_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        match relation_instance.clone().type_name.as_str() {
            TRAY_HAS_MENU_ITEM => self.remove_tray_has_menu_item_behaviour(relation_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_key(&self, edge_key: EdgeKey) {
        self.remove_by_key(edge_key);
    }
}
