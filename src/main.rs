mod operators;
mod routes;
mod utils;
use actix_web::{
    http::header::ContentType, middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer,
};
use dotenvy::dotenv;
use routes::{get_all_indexes, get_similar_strings, index};
use std::env;
use utils::new_pg_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    dotenv().ok();
    let SERVER_ADDRESS = env::var("SERVER_ADDRESS").expect("cannot read server addr");
    let SERVER_PORT = env::var("SERVER_PORT").expect("cannot read server port");
    let bind_addr = format!("{}:{}", SERVER_ADDRESS, SERVER_PORT);
    let pool = web::Data::new(new_pg_pool().await.unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(index)
            .service(get_all_indexes)
            .service(get_similar_strings)
            .wrap(Logger::default())
    })
    .bind(&bind_addr)?
    .workers(4)
    .run()
    .await
}
