pub mod product {
    use std::convert::TryFrom;

    use common::Product;
    use sea_orm::{entity::prelude::*, ActiveValue};

    #[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "product")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        #[sea_orm(unique)]
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

    macro_rules! unwrap_or_err {
        ($struct:ident.$field:ident) => {
            let $field = if !$struct.$field.is_unset() {
                $struct.$field.unwrap()
            } else {
                return Err(::eyre::eyre!("field not set: {}", stringify!()));
            };
        };
    }

    impl TryFrom<ActiveModel> for Product {
        type Error = eyre::Error;

        fn try_from(value: ActiveModel) -> Result<Self, Self::Error> {
            unwrap_or_err!(value.id);
            unwrap_or_err!(value.name);
            unwrap_or_err!(value.caffeine);
            unwrap_or_err!(value.alcohol);
            unwrap_or_err!(value.energy);
            unwrap_or_err!(value.sugar);
            unwrap_or_err!(value.price);
            unwrap_or_err!(value.created_at);
            unwrap_or_err!(value.updated_at);
            unwrap_or_err!(value.active);
            unwrap_or_err!(value.image);

            Ok(Product {
                id,
                name,
                caffeine,
                alcohol,
                energy,
                sugar,
                price,
                created_at: chrono::DateTime::from_utc(created_at, chrono::Utc),
                updated_at: chrono::DateTime::from_utc(updated_at, chrono::Utc),
                active,
                image,
            })
        }
    }
}

pub mod user {
    use std::convert::TryFrom;

    use common::User;
    use sea_orm::{entity::prelude::*, ActiveValue};

    #[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "user")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        #[sea_orm(unique)]
        pub name: String,
        pub email: Option<String>,
        pub created_at: DateTime,
        pub updated_at: DateTime,
        pub balance: i32,
        pub active: bool,
        pub audit: bool,
        pub redirect: bool,
        pub avatar: Option<i32>,
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

    impl From<Model> for User {
        fn from(model: Model) -> Self {
            User {
                id: model.id,
                name: model.name,
                email: model.email,
                created_at: chrono::DateTime::from_utc(model.created_at, chrono::Utc),
                updated_at: chrono::DateTime::from_utc(model.updated_at, chrono::Utc),
                balance: model.balance,
                barcode: None,
                active: model.active,
                audit: model.audit,
                redirect: model.redirect,
                avatar: model.avatar,
            }
        }
    }

    macro_rules! unwrap_or_err {
        ($struct:ident.$field:ident) => {
            let $field = if !$struct.$field.is_unset() {
                $struct.$field.unwrap()
            } else {
                return Err(::eyre::eyre!("field not set: {}", stringify!()));
            };
        };
    }

    impl TryFrom<ActiveModel> for User {
        type Error = eyre::Error;

        fn try_from(value: ActiveModel) -> Result<Self, Self::Error> {
            unwrap_or_err!(value.id);
            unwrap_or_err!(value.name);
            unwrap_or_err!(value.email);
            unwrap_or_err!(value.created_at);
            unwrap_or_err!(value.updated_at);
            unwrap_or_err!(value.balance);
            unwrap_or_err!(value.active);
            unwrap_or_err!(value.audit);
            unwrap_or_err!(value.redirect);
            unwrap_or_err!(value.avatar);

            Ok(User {
                id,
                name,
                email,
                created_at: chrono::DateTime::from_utc(created_at, chrono::Utc),
                updated_at: chrono::DateTime::from_utc(updated_at, chrono::Utc),
                balance,
                barcode: None,
                active,
                audit,
                redirect,
                avatar,
            })
        }
    }
}
