use actix_web::{middleware, web, App, HttpServer};
use app::{initialize_app_state, initialize_env};
use graphql::{graphiql, graphql};
use std::io;

mod app;
mod graphql;

fn main() -> io::Result<()> {
    initialize_env();

    let app_state = initialize_app_state();
    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(app_state.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
