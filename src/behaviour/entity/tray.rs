use inexor_rgf_core_model::{PropertyInstanceSetter, ReactiveRelationInstance};
use log::debug;
use serde_json::json;
use std::sync::Arc;
use std::sync::RwLock;

use tray_item::IconSource;
use tray_item::TrayItem;

use crate::behaviour::entity::{TrayMenuItemProperties, TrayProperties};
use crate::model::PropertyInstanceGetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub const TRAY: &str = "tray";

pub struct Tray {
    pub entity: Arc<ReactiveEntityInstance>,
    pub tray_item: RwLock<TrayItem>,
}

impl Tray {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<Tray, BehaviourCreationError> {
        let title = e
            .get(TrayProperties::TITLE)
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or(TrayProperties::TITLE.default_value().to_string());
        let icon = e
            .get(TrayProperties::ICON)
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or(TrayProperties::ICON.default_value().to_string());
        // There's no other option than leaking the memory
        let icon = Box::leak(icon.into_boxed_str());
        let icon = IconSource::Resource(icon);
        match TrayItem::new(title.as_str(), icon) {
            Ok(tray_item) => Ok(Tray {
                entity: e,
                tray_item: RwLock::new(tray_item),
            }),
            Err(_) => Err(BehaviourCreationError),
        }
    }

    pub fn add_menu_item(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let mut tray_item = self.tray_item.write().unwrap();
        let tray_menu_item = relation_instance.inbound.clone();
        if let Some(title) = tray_menu_item
            .get(TrayMenuItemProperties::TITLE)
            .and_then(|v| v.as_str().map(|s| s.to_string()))
        {
            let id = tray_menu_item.id;
            if let Ok(_) = tray_item.add_menu_item(id, title.as_str(), move || {
                tray_menu_item.set(TrayMenuItemProperties::TRIGGER, json!(true));
            }) {
                debug!("Added tray menu item");
            }
        }
    }

    pub fn remove_menu_item(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let mut tray_item = self.tray_item.write().unwrap();
        let id = relation_instance.inbound.id;
        if let Ok(_) = tray_item.remove(id) {
            debug!("Removed tray menu item");
        }
    }
}

impl Disconnectable for Tray {
    fn disconnect(&self) {}
}

impl Drop for Tray {
    fn drop(&mut self) {
        self.disconnect();
    }
}
