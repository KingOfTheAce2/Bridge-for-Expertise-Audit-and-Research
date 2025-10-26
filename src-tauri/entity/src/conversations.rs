use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "conversations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub case_id: i32,
    pub title: String,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::cases::Entity",
        from = "Column::CaseId",
        to = "super::cases::Column::Id",
        on_delete = "Cascade"
    )]
    Cases,
    #[sea_orm(has_many = "super::messages::Entity")]
    Messages,
}

impl Related<super::cases::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Cases.def()
    }
}

impl Related<super::messages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Messages.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
