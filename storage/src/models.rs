use crate::schema::ingredients;
use diesel::prelude::*;
use diesel::{r2d2, r2d2::ConnectionManager, r2d2::PooledConnection, PgConnection};
use juniper;
use juniper::FieldResult;
use juniper::RootNode;

type Connection = ConnectionManager<PgConnection>;

pub struct Context {
    pub pool: r2d2::Pool<Connection>,
}

impl juniper::Context for Context {}

#[derive(Queryable, GraphQLObject)]
#[graphql(description = "This is an ingredient that can be input in a recipe")]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, GraphQLInputObject)]
#[graphql(description = "New Ingredient to be inserted")]
#[table_name = "ingredients"]
pub struct NewIngredient {
    pub name: String,
}

#[derive(Queryable)]
pub struct Recipe {
    pub name: String,
    pub creator: String,
}

pub struct ReceipesIngredientList {}

pub struct QueryRoot;

fn connection_from_pool(
    ctx: &Context,
) -> Result<PooledConnection<Connection>, juniper::FieldError> {
    ctx.pool.get().map_err(|e| {
        juniper::FieldError::new(
            e,
            juniper::graphql_value!({ "internal_error": "Connection to database" }),
        )
    })
}

graphql_object!( QueryRoot: Context |&self| {
    field apiVersion() -> &str {
        "1.0"
    }

    field ingredient(&executor, id: i32) -> FieldResult<Ingredient> {
        let connection = connection_from_pool(&executor.context())?;

        ingredients::table
            .filter(ingredients::id.eq(id))
            .first::<Ingredient>(&connection)
            .map_err(|e| {
                juniper::FieldError::new(
                    e,
                    juniper::graphql_value!({ "internal_error": "Could not find ingredient in DB" })
                )
            })
    }
});

pub struct MutationRoot;
pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

graphql_object!(MutationRoot: Context |&self| {
    field createIngredient(&executor, new_ingredient: NewIngredient) -> FieldResult<Ingredient> {
        let connection = connection_from_pool(&executor.context())?;

        diesel::insert_into(ingredients::table)
        .values(&new_ingredient)
        .get_result(&connection)
        .map_err(|e| {
                juniper::FieldError::new(
                    e,
                    juniper::graphql_value!({ "internal_error": "Problem while inserting ingredient in DB" })
                )
            })
    }



});

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
