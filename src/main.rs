use std::net::SocketAddr;

// use axum::routing::post;
use axum::{routing::{get,post}, Router};

pub mod database;
pub mod routes;
pub mod types;

use database::dbconnection;
use routes::login::{login,auth};
use routes::signup::{signup,signup_done};


#[tokio::main]
async fn main() {
	let client = dbconnection().await.unwrap();
    let addrs = SocketAddr::from(([0, 0, 0, 0], 8000));
    let tcp = tokio::net::TcpListener::bind(&addrs).await.unwrap();
    let app = Router::new()
        .route("/", get(login))
		.route("/auth", post(auth))
        .route("/signup", get(signup))
        .route("/signup_done", post(signup_done))
        .with_state(client);
    axum::serve(tcp, app.into_make_service()).await.unwrap();
}
