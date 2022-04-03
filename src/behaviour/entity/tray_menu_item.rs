// use std::sync::Arc;
//
// use crate::model::ReactiveEntityInstance;
// use crate::reactive::entity::Disconnectable;
// use crate::reactive::BehaviourCreationError;
//
// pub const TRAY_MENU_ITEM: &str = "tray_menu_item";
//
// pub struct TrayMenuItem {
//     pub entity: Arc<ReactiveEntityInstance>,
// }
//
// impl TrayMenuItem {
//     pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<TrayMenuItem, BehaviourCreationError> {
//         Ok(TrayMenuItem { entity: e.clone() })
//     }
// }
//
// impl Disconnectable for TrayMenuItem {
//     fn disconnect(&self) {}
// }
//
// impl Drop for TrayMenuItem {
//     fn drop(&mut self) {
//         self.disconnect();
//     }
// }
