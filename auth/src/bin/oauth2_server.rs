use std::env;
use std::sync::Mutex;

use actix_web::http::header::LOCATION;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use twitter_v2::authorization::{Oauth2Client, Oauth2Token, Scope};
use twitter_v2::oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier};
use twitter_v2::query::{SpaceField, UserField};
use twitter_v2::TwitterApi;
use url::Url;

static CALLBACK_URI: &'static str = "http://127.0.0.1:8080/callback";

// global app context
struct AppContext {
    verifier: Mutex<String>,
    state: Mutex<String>,
    token: Mutex<Option<Oauth2Token>>,
}

#[get("/auth")]
async fn auth(
    twitter_oauth2_client: web::Data<Oauth2Client>,
    app_context: web::Data<AppContext>,
) -> HttpResponse {
    // set scopes
    let scopes = [
        Scope::TweetRead,
        Scope::TweetWrite,
        Scope::ListRead,
        Scope::ListWrite,
        Scope::SpaceRead,
        Scope::OfflineAccess,
        Scope::UsersRead,
    ];
    // generate auth url
    let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();
    let mut ctx_verifier = app_context.verifier.lock().unwrap();
    // store verifier in app context
    *ctx_verifier = verifier.secret().to_owned();
    let (auth_url, state) = twitter_oauth2_client.auth_url(challenge, scopes);
    let mut ctx_state = app_context.state.lock().unwrap();
    // store state in app context
    *ctx_state = state.secret().to_owned();
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
    app_context: web::Data<AppContext>,
) -> HttpResponse {
    // get stored state
    let ctx_state = app_context.state.lock().unwrap().to_string();
    let query_params_state = query_params.0.state;
    // compare stored state to query params state
    if ctx_state != query_params_state {
        return HttpResponse::InternalServerError().body("Invalid state");
    }
    let authorization_code = AuthorizationCode::new(query_params.0.code);
    // get verifier from app context
    let ctx_verifier = app_context.verifier.lock().unwrap();
    let code_verifier = PkceCodeVerifier::new(ctx_verifier.to_string());
    let oauth2_token = twitter_oauth2_client
        .request_token(authorization_code, code_verifier)
        .await
        .unwrap();
    let mut ctx_token = app_context.token.lock().unwrap();
    // store token in our application context
    *ctx_token = Some(oauth2_token);
    HttpResponse::Ok().body("Success!")
}

#[get("/action")]
async fn action(
    app_context: web::Data<AppContext>,
) -> HttpResponse {
    let ouath_token = app_context
        .token
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .to_owned();
    let api = TwitterApi::new(ouath_token);
    api.post_tweet().text("foo".to_string()).send().await.unwrap();
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct QueryParams {
    pub code: String,
    pub state: String,
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
    // create app context
    let app_context = web::Data::new(AppContext {
        verifier: Mutex::new(String::new()),
        state: Mutex::new(String::new()),
        token: Mutex::new(None),
    });
    HttpServer::new(move || {
        App::new()
            .service(auth)
            .service(callback)
            .service(action)
            .app_data(web::Data::new(twitter_oauth2_client.clone()))
            .app_data(app_context.clone())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
