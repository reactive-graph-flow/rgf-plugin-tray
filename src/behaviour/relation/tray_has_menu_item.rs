use std::sync::Arc;

use crate::model::ReactiveRelationInstance;
use crate::reactive::entity::Disconnectable;

pub struct TrayHasMenuItem {
    pub relation: Arc<ReactiveRelationInstance>,
}

impl TrayHasMenuItem {
    pub fn new<'a>(r: Arc<ReactiveRelationInstance>) -> TrayHasMenuItem {
        TrayHasMenuItem { relation: r.clone() }
    }
}

impl Disconnectable for TrayHasMenuItem {
    fn disconnect(&self) {}
}

impl Drop for TrayHasMenuItem {
    fn drop(&mut self) {
        self.disconnect();
    }
}
