use crate::reactive::NamedProperties;
use indradb::{Identifier, NamedProperty};
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum TrayProperties {
    #[strum(serialize = "title")]
    TITLE,
    #[strum(serialize = "icon")]
    ICON,
}

impl TrayProperties {
    pub fn default_value(&self) -> Value {
        match self {
            TrayProperties::TITLE => json!("Inexor"),
            TrayProperties::ICON => json!("accessories-calculator"),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![NamedProperty::from(TrayProperties::TITLE), NamedProperty::from(TrayProperties::ICON)]
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
