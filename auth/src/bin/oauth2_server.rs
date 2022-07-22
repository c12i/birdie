use std::env;

use actix_web::http::header::LOCATION;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use twitter_v2::authorization::{Oauth2Client, Scope};
use twitter_v2::oauth2::{AuthorizationCode, PkceCodeChallenge, PkceCodeVerifier};
use url::Url;

static CALLBACK_URI: &'static str = "http://127.0.0.1:8080/callback";
static PKCE_CODE_VERIFIER: &'static str =
    "this-is-the-verifier-string-that-is-absolutely-long-and-super-secure";

#[get("/auth")]
async fn auth(twitter_oauth2_client: web::Data<Oauth2Client>) -> HttpResponse {
    // set scopes
    let scopes = [
        Scope::TweetRead,
        Scope::TweetWrite,
        Scope::ListRead,
        Scope::ListWrite,
        Scope::SpaceRead,
        Scope::OfflineAccess,
    ];
    // generate auth url
    let pkce_code_verifier = PkceCodeVerifier::new(PKCE_CODE_VERIFIER.to_string());
    let pkce_code_challenge = PkceCodeChallenge::from_code_verifier_sha256(&pkce_code_verifier);
    let (auth_url, _) = twitter_oauth2_client.auth_url(pkce_code_challenge, scopes);
    let auth_url = auth_url.as_str();
    log::info!("Twitter OAuth2 URL: {auth_url}");
    // redirect user to auth url
    HttpResponse::SeeOther()
        .insert_header((LOCATION, auth_url))
        .finish()
}

#[get("/callback")]
async fn callback(
    query_params: web::Query<QueryParams>,
    twitter_oauth2_client: web::Data<Oauth2Client>,
) -> HttpResponse {
    let authorization_code = AuthorizationCode::new(query_params.0.code);
    let code_verifier = PkceCodeVerifier::new(PKCE_CODE_VERIFIER.to_string());
    let oauth2_token = twitter_oauth2_client
        .request_token(authorization_code, code_verifier)
        .await
        .unwrap();
    // do something with the token i.e securely store them in a database for future reqests
    let _access_token = oauth2_token.access_token().secret();
    let _refresh_token = oauth2_token.refresh_token().unwrap().secret();
    HttpResponse::Ok().body("Success!")
}

#[derive(Deserialize)]
struct QueryParams {
    pub code: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    // get client_id and client_secret from env variables
    let client_id = env::var("TWITTER_OAUTH_CLIENT_ID").unwrap();
    let client_secret = env::var("TWITTER_OAUTH_CLIENT_SECRET").unwrap();
    // parse `CALLBACK_URL` to a `Url` type expected by the outh2 client
    let callback_url = Url::parse(CALLBACK_URI).unwrap();
    // create twitter oauth2 client
    let twitter_oauth2_client = Oauth2Client::new(client_id, client_secret, callback_url);
    HttpServer::new(move || {
        App::new()
            .service(auth)
            .service(callback)
            .app_data(web::Data::new(twitter_oauth2_client.clone()))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
