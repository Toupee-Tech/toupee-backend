//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "aprs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub plugin_address: String,
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub token_address: String,
    #[sea_orm(column_type = "Double", nullable)]
    pub apr: Option<f64>,
    #[sea_orm(column_type = "Double", nullable)]
    pub min_apr: Option<f64>,
    #[sea_orm(column_type = "Double", nullable)]
    pub max_apr: Option<f64>,
    #[sea_orm(column_type = "Text")]
    pub symbol: String,
    #[sea_orm(column_type = "Text")]
    pub logo_url: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::assets::Entity",
        from = "Column::TokenAddress",
        to = "super::assets::Column::Address",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Assets,
    #[sea_orm(
        belongs_to = "super::plugins::Entity",
        from = "Column::PluginAddress",
        to = "super::plugins::Column::Address",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Plugins,
}

impl Related<super::assets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Assets.def()
    }
}

impl Related<super::plugins::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Plugins.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
