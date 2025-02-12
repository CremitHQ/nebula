use chrono::{DateTime, Utc};
use sea_orm::prelude::*;

use super::UlidId;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "machine_identity")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: UlidId,
    pub owner_gid: String,
    pub label: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::machine_identity_attribute::Entity")]
    MachineIdentityAttribute,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::machine_identity_attribute::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MachineIdentityAttribute.def()
    }
}
