use std::path::PathBuf;
use std::env::set_var;

use axum::{Router, routing::{get, post}};
use axum_extra::routing::SpaRouter;
mod user;
mod common;
mod msg;
mod server;
use shuttle_secrets::SecretStore;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let uri = secret_store.get("MONGO_URI").expect("MONGO_URI not found");
    set_var("MONGO_URI", uri);
    let router =
        Router::new().merge(SpaRouter::new("/", static_folder).index_file("index.html"))
        .route("/user/create", post(user::create_user))
        .route("/user/info", get(user::get_user_info))
        .route("/server/create", post(server::create_server))
        .route("/server/join", post(server::join_server))
        .route("/server/info", get(server::info_server))
        .route("/chat/send", post(msg::send_chat))
        .route("/chat/info", get(msg::info_chat));

    Ok(router.into())
}
