use crate::reactive::NamedProperties;
use indradb::Identifier;
use indradb::NamedProperty;
use serde_json::json;
use serde_json::Value;
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum TrayMenuItemProperties {
    #[strum(serialize = "title")]
    TITLE,
    #[strum(serialize = "trigger")]
    TRIGGER,
}

impl TrayMenuItemProperties {
    pub fn default_value(&self) -> Value {
        match self {
            TrayMenuItemProperties::TITLE => json!(""),
            TrayMenuItemProperties::TRIGGER => json!(false),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(TrayMenuItemProperties::TITLE),
            NamedProperty::from(TrayMenuItemProperties::TRIGGER),
        ]
    }
}

impl From<TrayMenuItemProperties> for NamedProperty {
    fn from(p: TrayMenuItemProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<TrayMenuItemProperties> for String {
    fn from(p: TrayMenuItemProperties) -> Self {
        p.to_string()
    }
}
