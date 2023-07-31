use std::path::PathBuf;

use axum::{Router, routing::{get, post}};
use axum_extra::routing::SpaRouter;
use mongodb::{options::{ClientOptions, ServerApi, ServerApiVersion}, Client};
mod user;
mod common;
mod msg;
mod server;
use shuttle_secrets::SecretStore;
use user::UserEndpoint;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let uri = secret_store.get("MONGO_URI").expect("MONGO_URI not found");
    let mut client_options =
        ClientOptions::parse(uri)
            .await.expect("option error");
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options).expect("option error");
    let db = client.database("anachat");
    let userendpoint = UserEndpoint::new(db);
    let router =
        Router::new().merge(SpaRouter::new("/", static_folder).index_file("index.html"))
        .route("/user/login", post(userendpoint.login_end)); // TODO we need to fix it

    Ok(router.into())
}
