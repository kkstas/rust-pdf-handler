#![allow(unused)]

use axum::Router;
use std::net::SocketAddr;

use self::error::{Error, Result};

mod error;
pub mod pdf;

#[tokio::main]
async fn main() -> Result<()> {
    let main_routes = Router::new().merge(pdf::generate_pdf::routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(main_routes.into_make_service())
        .await
        .unwrap();
    Ok(())
}
