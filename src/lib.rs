use std::sync::Mutex;

use actix_files::NamedFile;
use actix_web::{get, web, HttpResponse, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_todo);
    cfg.service(clicked);
    cfg.service(count);
}

#[get("/")]
pub async fn hello_todo() -> impl Responder {
    NamedFile::open_async("./static/index.html").await
}

#[get("/clicked")]
pub async fn clicked() -> impl Responder {
    HttpResponse::Ok().body("clicked!!!")
}

pub struct Counter {
    pub count: Mutex<i32>,
}

#[get("/count")]
pub async fn count(data: web::Data<Counter>) -> impl Responder {
    let mut counter = data.count.lock().unwrap();
    *counter += 1;
    HttpResponse::Ok().body(format!("count :{}", counter))
}
