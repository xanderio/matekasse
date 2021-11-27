pub mod product {
    use common::Product;
    use sea_orm::{entity::prelude::*, ActiveValue};

    #[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "product")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub name: String,
        pub caffeine: Option<i32>,
        pub alcohol: Option<i32>,
        pub energy: Option<i32>,
        pub sugar: Option<i32>,
        pub price: i32,
        pub created_at: DateTime,
        pub updated_at: DateTime,
        pub active: bool,
        pub image: Option<i32>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {
        fn before_save(self, _: bool) -> Result<Self, DbErr> {
            Ok(Self {
                updated_at: ActiveValue::set(chrono::Utc::now().naive_utc()),
                ..self
            })
        }
    }

    #[allow(clippy::from_over_into)]
    impl From<Model> for Product {
        fn from(model: Model) -> Self {
            Product {
                id: model.id,
                name: model.name,
                caffeine: model.caffeine,
                alcohol: model.alcohol,
                energy: model.energy,
                sugar: model.sugar,
                price: model.price,
                created_at: chrono::DateTime::from_utc(model.created_at, chrono::Utc),
                updated_at: chrono::DateTime::from_utc(model.updated_at, chrono::Utc),
                active: model.active,
                image: model.image,
            }
        }
    }
}
