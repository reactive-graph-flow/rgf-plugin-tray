use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone)]
pub struct TrayMenu {
    pub title: String,
    pub icon: String,
    pub items: Vec<TrayMenuItem>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TrayMenuItem {
    pub label: String,
    pub value: Value,
}
