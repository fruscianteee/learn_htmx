use actix_web::{web, App, HttpServer};
use anyhow::Result;
use dotenv::dotenv;
use learn_htmx::{config, Counter};
// use libsql_client::Client;
use std::{net::SocketAddr, sync::Mutex};

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    // let db = Client::from_env().await?;
    // let repository = web::Data::new(db);
    let counter = web::Data::new(Counter {
        count: Mutex::new(0),
    });
    let _ = HttpServer::new(move || {
        // App::new().app_data(repository.clone()).configure(config) // 各routerの定義
        App::new().app_data(counter.clone()).configure(config)
    })
    .bind(addr)?
    .run()
    .await;

    Ok(())
}
