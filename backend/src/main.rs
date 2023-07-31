use std::path::PathBuf;

use axum::{Router, routing::get};
use axum_extra::routing::SpaRouter;
mod user;
mod common;
mod msg;
mod server;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    let router =
        Router::new().merge(SpaRouter::new("/", static_folder).index_file("index.html"));

    Ok(router.into())
}
