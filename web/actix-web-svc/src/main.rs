use actix_web::{web, guard, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;

struct Data {
    app_name : String,
}

#[rustfmt::skip]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(||
        App::new()
            .data(Data {
                app_name: "LandeApi".to_string()
            })
            .wrap(Logger::default())
            .service(
                web::scope("/app")
                    .guard(guard::Header("key", "secret"))
                    .route("/hello", web::get().to(index))
            )
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


async fn index(data: web::Data<Data>) -> String {
    format!("Hello world from {}", data.app_name)
}
