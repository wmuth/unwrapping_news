use std::sync::atomic::{AtomicU32, Ordering};

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

struct State {
    count: AtomicU32,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/count")]
async fn count(data: web::Data<State>) -> impl Responder {
    data.count.fetch_add(1, Ordering::Relaxed);
    HttpResponse::Ok().body(format!("Count is {}", data.count.load(Ordering::Relaxed)))
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let data = web::Data::new(State {
        count: AtomicU32::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(hello)
            .service(count)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
