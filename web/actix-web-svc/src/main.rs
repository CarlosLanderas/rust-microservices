use actix_web::middleware::Logger;
use actix_web::{guard, web, get, App, HttpResponse, HttpServer, Responder, HttpRequest, Error, Either};
use serde_derive::Serialize;
use actix_web::body::Body;
use futures::future::{Future, ok, ready, Ready};

struct Data {
    app_name: String,
}

#[derive(Serialize, Debug)]
struct User {
    name: &'static str,
    age: u8,
}

impl Responder for User {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
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
                    .service(user)
                    .service(path_one)
            )
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn index(data: web::Data<Data>) -> String {
    format!("Hello world from {}", data.app_name)
}

#[get("/user")]
async fn user() -> impl Responder {
    User {
        name: "Carlos Landeras",
        age: 34
    }
}


#[get("/path1")]
async fn path_one(req: HttpRequest) -> impl Responder {

    if req.query_string().contains("param1") {
        let data = vec!["Some", "hidden", "data"];
        let body = serde_json::to_string(&data).unwrap();
        HttpResponse::Ok().content_type("application/json").body(body)
    } else {
        HttpResponse::BadRequest().body("")
    }
}