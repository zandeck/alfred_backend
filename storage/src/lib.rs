#[macro_use]
extern crate diesel;

#[macro_use]
extern crate juniper;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

use models::Ingredient;
use models::NewIngredient;
pub use models::{create_schema, Context, Schema};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn get_ingredients() {
    use crate::schema::ingredients::dsl::*;

    let connection = establish_connection();
    let result = ingredients
        .limit(5)
        .load::<Ingredient>(&connection)
        .expect("error loading ingredients");

    for ingredient in result {
        println!("{} -> {}", ingredient.id, ingredient.name);
    }
}

pub fn create_ingredient(name: String) -> Ingredient {
    use schema::ingredients;
    let connection = establish_connection();

    let new_ingredient = NewIngredient { name };
    diesel::insert_into(ingredients::table)
        .values(&new_ingredient)
        .get_result(&connection)
        .expect("error while inserting")
}
