use actix_web::{get, App, HttpResponse, HttpServer};
use actix_web::http::header::{LOCATION, AUTHORIZATION};

static CALLBACK_URI: &'static str = "http://127.0.0.1:8080/callback";

#[get("/auth")]
async fn auth() -> HttpResponse {
    // generate auth url
    todo!()
}

#[get("/callback")]
async fn callback() -> HttpResponse {
    HttpResponse::Ok().body("All good!") 
}

#[get("/healthz")]
async fn healthz() -> HttpResponse {
    HttpResponse::Ok().body("All good!") 
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(auth)
            .service(callback)
            .service(healthz)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}