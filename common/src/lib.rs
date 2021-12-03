use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    /// mg of caffeine per 100 ml/mg/unit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caffeine: Option<i32>,
    /// mg of caffeine per 100 ml/mg/unit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alcohol: Option<i32>,
    /// energy per 100ml / 100g with no decimal place
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy: Option<i32>,
    /// g sugar per 100g / 100ml with one decimal place
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sugar: Option<i32>,
    pub price: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<i32>,
}

impl Default for Product {
    fn default() -> Self {
        Self {
            created_at: Utc::now(),
            updated_at: Utc::now(),

            id: Default::default(),
            name: Default::default(),
            caffeine: Default::default(),
            alcohol: Default::default(),
            energy: Default::default(),
            sugar: Default::default(),
            price: 150,
            active: true,
            image: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub struct ProductCreateRequest {
    pub name: String,
    /// mg of caffeine per 100 ml/mg/unit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caffeine: Option<i32>,
    /// mg of caffeine per 100 ml/mg/unit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alcohol: Option<i32>,
    /// energy per 100ml / 100g with no decimal place
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy: Option<i32>,
    /// g sugar per 100g / 100ml with one decimal place
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sugar: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<i32>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub struct ProductEditRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// mg of caffeine per 100 ml/mg/unit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caffeine: Option<i32>,
    /// mg of caffeine per 100 ml/mg/unit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alcohol: Option<i32>,
    /// energy per 100ml / 100g with no decimal place
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy: Option<i32>,
    /// g sugar per 100g / 100ml with one decimal place
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sugar: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerInfo {
    pub version: String,

    #[serde(with = "i32_or_false")]
    /// global credit limit in cent (subdevision of currency); to disable set to [`Option::None`]
    pub global_credit_limit: Option<i32>,
    pub currency: String,
    /// If true the currency symbol is show before the sum
    pub currency_before: bool,
    /// if None, the currency uses no subdevision and therefore no decimal seperator is used. E.g. in Sweden
    pub decimal_seperator: Option<String>,
    /// unit of energy e.g. kcal or kj
    pub energy: String,
    #[serde(rename = "defaults")]
    pub default_product: DefaultProduct,
}

impl Default for ServerInfo {
    fn default() -> Self {
        Self {
            version: "3.0.0".to_string(),
            global_credit_limit: Default::default(),
            currency: "€".to_string(),
            currency_before: false,
            decimal_seperator: Some(",".to_string()),
            energy: "kcal".to_string(),
            default_product: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultProduct {
    /// default price in cent
    pub price: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// default package size
    pub package_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// default caffeine contents in mg per 100ml/g
    pub caffine: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// default volume percent of alcohol (with two decimal places)
    pub alcohol: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// default energy per 100g / 100ml without decimal places
    pub energy: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sugar: Option<i32>,
    pub active: bool,
}

mod i32_or_false {
    use serde::{de::Visitor, Deserializer, Serializer};

    pub fn serialize<S>(data: &Option<i32>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(value) = data {
            serializer.serialize_i32(*value)
        } else {
            serializer.serialize_bool(false)
        }
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct I32OrFalse;

        impl<'de> Visitor<'de> for I32OrFalse {
            type Value = Option<i32>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("i32 or false")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if !v {
                    Ok(None)
                } else {
                    Err(serde::de::Error::custom("expect false got true"))
                }
            }

            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Some(v))
            }
        }

        deserializer.deserialize_any(I32OrFalse)
    }

    #[cfg(test)]
    mod tests {
        use serde::{Deserialize, Serialize};
        use serde_test::{assert_tokens, Token};

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Test {
            #[serde(with = "super")]
            value: Option<i32>,
        }

        #[test]
        fn false_to_none() {
            let value = Test { value: None };

            assert_tokens(
                &value,
                &[
                    Token::Struct {
                        name: "Test",
                        len: 1,
                    },
                    Token::Str("value"),
                    Token::Bool(false),
                    Token::StructEnd,
                ],
            )
        }

        #[test]
        fn u64_to_some() {
            let value = Test { value: Some(42) };

            assert_tokens(
                &value,
                &[
                    Token::Struct {
                        name: "Test",
                        len: 1,
                    },
                    Token::Str("value"),
                    Token::I32(42),
                    Token::StructEnd,
                ],
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub balance: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<String>,
    #[serde(default = "default_true")]
    pub active: bool,
    #[serde(default)]
    pub audit: bool,
    #[serde(default = "default_true")]
    pub redirect: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<i32>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            created_at: Utc::now(),
            updated_at: Utc::now(),
            active: default_true(),
            redirect: default_true(),

            id: Default::default(),
            name: Default::default(),
            email: Default::default(),
            balance: Default::default(),
            barcode: Default::default(),
            audit: Default::default(),
            avatar: Default::default(),
        }
    }
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserCreateRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserEditRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FundsTransferRequest {
    pub amount: i32,
    pub receiver: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsersStatsResponce {
    pub user_count: i32,
    pub active_count: i32,
    pub balance_sum: i32,
}
