use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NewLog {
    pub name: String,
    pub datetime: DateTime<Utc>,
    pub calories: f64,
    pub protein: f64,
    pub grams: Option<f64>
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Log {
    pub id: Uuid,
    pub name: String,
    pub datetime: DateTime<Utc>,
    pub calories: f64,
    pub protein: f64,
    pub grams: Option<f64>
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NewDay {
    pub datetime: DateTime<Utc>,
    pub logs: Vec<Log>
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Day {
    pub id: Uuid,
    pub datetime: DateTime<Utc>,
    pub logs: Vec<Log>,
    pub calories: f64,
    pub protein: f64
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NewIngredient {
    pub name: String,
    pub calories_p100g: f64,
    pub protein_p100g: f64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Ingredient {
    pub id: Uuid,
    pub name: String,
    pub calories_p100g: f64,
    pub protein_p100g: f64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NewRecipe {
    pub datetime: DateTime<Utc>,
    pub ingredients: Vec<Ingredient>,
    pub calories_p100g: f64,
    pub protein_p100g: f64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Recipe {
    pub id: Uuid,
    pub datetime: DateTime<Utc>,
    pub ingredients: Vec<Ingredient>,
    pub calories_p100g: f64,
    pub protein_p100g: f64,
}
