use crate::reactive::NamedProperties;
use indradb::{Identifier, NamedProperty};
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum TrayProperties {
    #[strum(serialize = "menu")]
    MENU,
    #[strum(serialize = "trigger")]
    TRIGGER,
}

impl TrayProperties {
    pub fn default_value(&self) -> Value {
        match self {
            TrayProperties::MENU => json!({}),
            TrayProperties::TRIGGER => json!({}),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![NamedProperty::from(TrayProperties::MENU), NamedProperty::from(TrayProperties::TRIGGER)]
    }
}

impl From<TrayProperties> for NamedProperty {
    fn from(p: TrayProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<TrayProperties> for String {
    fn from(p: TrayProperties) -> Self {
        p.to_string()
    }
}
