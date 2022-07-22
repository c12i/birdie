use actix_web::{get, App, HttpResponse, HttpServer};
use actix_web::http::header::{LOCATION, AUTHORIZATION};

static CALLBACK_URI: &'static str = "http://127.0.0.1:8080/callback";

#[get("/auth")]
async fn auth() -> HttpResponse {
    // generate auth url
    let client_id = std::env::var("TWITTER_OAUTH_CLIENT_ID").unwrap();
    let client_secret = std::env::var("TWITTER_OAUTH_CLIENT_SECRET").unwrap();
    let encoded_credentials = base64::encode(format!("{client_id}:{client_secret}").as_bytes());
    let auth_url = format!("https://twitter.com/i/oauth2/authorize?response_type=code&client_id={client_id}&redirect_uri={CALLBACK_URI}&scope=tweet.read%20users.read");
    HttpResponse::SeeOther()
        .insert_header((LOCATION, auth_url))
        .insert_header((AUTHORIZATION, format!("Basic: {encoded_credentials}")))
        .finish()
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