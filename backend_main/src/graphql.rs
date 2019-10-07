use crate::app::AppState;
use actix_web::{web, Error, HttpResponse, HttpRequest};
use actix_web::http::header;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use std::sync::Arc;
use actix_web_httpauth::headers::authorization::{Authorization, Basic};
use actix_web::http::header::Header;
use env_logger::info;
use futures::future::Future;

pub fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub fn graphql(
    st: web::Data<Arc<AppState>>,
    data: web::Json<GraphQLRequest>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let authorization = Authorization::<Basic>::parse(&req).ok();
    info!("{:?}", authorization);
    web::block(move || {
        let ctx = st.clone();
        let res = data.execute(&st.graphql_schema, &st.r2d2_pool);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
}
