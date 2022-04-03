use async_trait::async_trait;
use log::debug;
use log::error;
use rust_embed::RustEmbed;

use crate::di::*;
use crate::model::relation_type::RelationType;
use crate::plugins::RelationTypeProvider;

#[derive(RustEmbed)]
#[folder = "./assets/types/relations"]
struct TrayRelationTypeAsset;

#[async_trait]
pub trait TrayRelationTypeProvider: RelationTypeProvider + Send + Sync {}

#[derive(Clone)]
pub struct TrayRelationTypeProviderImpl {}

interfaces!(TrayRelationTypeProviderImpl: dyn RelationTypeProvider);

#[component]
impl TrayRelationTypeProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl TrayRelationTypeProvider for TrayRelationTypeProviderImpl {}

impl RelationTypeProvider for TrayRelationTypeProviderImpl {
    fn get_relation_types(&self) -> Vec<RelationType> {
        let mut relation_types = Vec::new();
        for file in TrayRelationTypeAsset::iter() {
            let filename = file.as_ref();
            debug!("Loading relation_type from resource {}", filename);
            let asset = TrayRelationTypeAsset::get(filename).unwrap();
            let json_str = std::str::from_utf8(asset.data.as_ref());
            if json_str.is_err() {
                error!("Could not decode UTF-8 {}", filename);
                continue;
            }
            let relation_type: RelationType = match serde_json::from_str(json_str.unwrap()) {
                Result::Ok(relation_type) => relation_type,
                Result::Err(err) => {
                    error!("Error in parsing JSON file {}: {}", filename, err);
                    continue;
                }
            };
            relation_types.push(relation_type);
        }
        relation_types
    }
}
