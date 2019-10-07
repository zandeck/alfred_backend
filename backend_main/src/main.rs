use actix_web::{http, middleware, web, App, HttpServer};
use app::{initialize_app_state, initialize_env};
use graphql::{graphiql, graphql};
use std::io;
use actix_cors::Cors;
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
            .wrap(middleware::Logger::new("%a %{authorization}i"))
            .wrap(Cors::new() // <- Construct CORS middleware builder
                //.allowed_origin("http://localhost:8081")
                //.allowed_origin("http://localhost:8080")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600))
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
