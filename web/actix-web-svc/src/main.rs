use actix_web::{web, get, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(||
        App::new()
            .wrap(Logger::default())
            .service(index)
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[get("/hello")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
