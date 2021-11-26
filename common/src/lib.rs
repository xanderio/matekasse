use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub struct Product {
    pub id: u64,
    pub name: String,
    /// mg of caffeine per 100 ml/mg/unit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caffeine: Option<u64>,
    /// mg of caffeine per 100 ml/mg/unit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alcohol: Option<u64>,
    /// energy per 100ml / 100g with no decimal place
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy: Option<u64>,
    /// g sugar per 100g / 100ml with one decimal place
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sugar: Option<u64>,
    pub price: u64,
    pub create_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
    pub active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<u64>,
}

impl Default for Product {
    fn default() -> Self {
        Self {
            create_at: Utc::now(),
            update_at: Utc::now(),

            id: Default::default(),
            name: Default::default(),
            caffeine: Default::default(),
            alcohol: Default::default(),
            energy: Default::default(),
            sugar: Default::default(),
            price: Default::default(),
            active: Default::default(),
            image: Default::default(),
        }
    }
}
