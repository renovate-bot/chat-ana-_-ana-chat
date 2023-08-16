use std::{env::set_var, path::PathBuf};

use axum::{Router, routing::{get, post}, http::Method};
use shuttle_secrets::SecretStore;
use tower_http::{services::{ServeFile, ServeDir}, cors::{CorsLayer, Any}};
mod user;
mod common;
mod msg;
mod server;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let uri = secret_store.get("MONGO_URI").expect("MONGO_URI not found");
    set_var("MONGO_URI", uri);
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any);
    let router =
        Router::new().layer(cors).nest_service("/", ServeDir::new(static_folder).not_found_service(ServeFile::new("index.html")))
        .route("/user/create", post(user::create_user))
        .route("/user/info", get(user::get_user_info))
        .route("/server/create", post(server::create_server))
        .route("/server/join", post(server::join_server))
        .route("/server/info", get(server::info_server))
        .route("/chat/send", post(msg::send_chat))
        .route("/chat/info", get(msg::info_chat));

    Ok(router.into())
}
