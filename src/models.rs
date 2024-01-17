use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FormData {
    pub brew_method: String,
    pub coffee: Coffee,
    pub weight: i32,
    pub water: i32,
    pub grind_size: i32,
    pub temperature: i32,
    pub rating: i32,
    pub funkiness: i32,
    pub acidity_bitterness: i32,
    pub strength: i32,
    pub notes: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Coffee {
    pub name: String,
    pub roastery: String,
    pub roast_date: String,
    pub roast_level: String,
    pub origin: String,
    pub region: String,
    pub farm: String,
    pub altitude: f32,
    pub variety: String,
    pub processing_method: String,
    pub tasting_notes: String,
    pub nickname: String,
}
