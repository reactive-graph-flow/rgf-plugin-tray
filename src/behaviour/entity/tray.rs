use std::sync::Arc;

use tray_item::{IconSource, TrayItem};

use crate::behaviour::entity::tray_menu::TrayMenu;
use crate::behaviour::entity::TrayProperties;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub const TRAY: &'static str = "tray";

pub struct Tray {
    pub entity: Arc<ReactiveEntityInstance>,
    pub tray_item: Option<Arc<TrayItem>>,
}

impl Tray {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<Tray, BehaviourCreationError> {
        let tray_menu = e.get(TrayProperties::MENU).unwrap_or(TrayProperties::MENU.default_value());
        let tray_item = match serde_json::from_value(tray_menu) {
            Ok(tray_menu) => {
                let tray_menu: TrayMenu = tray_menu;
                // There's no other option than leaking the memory
                let icon = Box::leak(tray_menu.icon.into_boxed_str());
                let icon = IconSource::Resource(icon);
                let mut tray_item = TrayItem::new(tray_menu.title.as_str(), icon).unwrap();
                for tray_menu_item in tray_menu.items {
                    let entity = e.clone();
                    let _ = tray_item.add_menu_item(tray_menu_item.label.as_str(), move || {
                        entity.set(TrayProperties::TRIGGER.as_ref(), tray_menu_item.value.clone());
                    });
                }
                Some(Arc::new(tray_item))
            }
            Err(_) => None,
        };

        Ok(Tray { entity: e.clone(), tray_item })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
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
