mod models;

use models::{
    NewLog,
    Log,
    NewDay,
    Day,
    NewRecipe,
    Recipe,
    Ingredient,
    NewIngredient
};

fn main() {
    let example_log = NewLog {
        name: String::from("example"),
        calories: 122.4,
        protein: 13.8
    };
    println!("{:?}", example_log);
}
