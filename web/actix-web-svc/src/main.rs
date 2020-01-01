use actix_web::middleware::Logger;
use actix_web::{guard, web, error, get, post, App, Result, HttpResponse, HttpServer, Responder, HttpRequest, Error, Either, ResponseError};
use serde::{Serialize, Deserialize};
use actix_web::body::Body;
use futures::future::{Future, ok, ready, Ready};
use actix_web::web::post;
use failure::Fail;

struct Data {
    app_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
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
                    .service(post_user)
                    .service(post_user_form)
                    .service(path_one)
                    .service(custom_error)
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
        name: "Carlos Landeras".to_string(),
        age: 34
    }
}

#[post("/user/{userid}/{dept}")]
async fn post_user(path: web::Path<(String,String)>,json: web::Json<User>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Received json user {} with age {} with id {} and dept {}", json.name, json.age, path.0, path.1))
}

#[post("/user-form")]
async fn post_user_form(form: web::Form<User>) -> Result<String> {
    Ok(format!("User {} - Age {}", form.name, form.age))
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

#[derive(Fail, Debug)]
#[fail(display = "Error processing the request: {}", name)]
struct MyError {
    name: &'static str
}

impl error::ResponseError for MyError {}


#[derive(Deserialize)]
struct QueryOptions {
    option: String,
}

#[get("/custom-error")]
async fn custom_error(query: web::Query<QueryOptions>) -> Result<&'static str, MyError> {
    if query.option == "1" {
        Err(MyError{ name : "Invalid Request"})
    }
    else {
        Ok("Done!")
    }
}