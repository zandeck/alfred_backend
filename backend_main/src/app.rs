use diesel::{r2d2, r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use storage::{create_schema, Context, Schema};

pub struct AppState {
    pub graphql_schema: Schema,
    pub r2d2_pool: Context,
}

pub fn initialize_env() {
    if cfg!(debug_assertions) {
        dotenv().ok();
    }
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
}

pub fn initialize_app_state() -> Arc<AppState> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to build the pool");
    // Create Juniper schema
    let schema = create_schema();

    Arc::new(AppState {
        graphql_schema: schema,
        r2d2_pool: Context { pool },
    })
}
