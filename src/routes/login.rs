use std::fs::read_to_string;

use axum::{extract::State, response::Html, Form};
use mongodb::{bson::doc, Client};

use crate::types::User;

pub async fn login() -> Html<String> {
    Html(read_to_string("pages/login.html").unwrap())
}

pub async fn auth(State(client): State<Client>, Form(usr): Form<User>) -> Html<String> {
    let doc = doc! {"username": usr.username.clone()};
    let maybe_user = client
        .database("mydb")
        .collection::<User>("users")
        .find_one(doc, None)
        .await;

    match maybe_user {
        Ok(Some(user)) => {
            if user.password == usr.password {
                Html("You are logged in".to_string())
            } else {
                Html("Invalid username or password".to_string())
            }
        }
        Ok(None) => Html("Invalid username or password".to_string()),
        Err(err) => Html(format!("Error: {}", err)),
    }
}
