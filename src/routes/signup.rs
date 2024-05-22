use std::fs::read_to_string;

use axum::{extract::State, response::Html, Form};
use mongodb::{bson::doc, Client};

use crate::types::User;

pub async fn signup() -> Html<String> {
    Html(read_to_string("pages/signup.html").unwrap())
}

pub async fn signup_done(State(client): State<Client>, Form(user): Form<User>) -> Html<String> {
    let db = client.database("mydb");
    let doc = doc! {"full_name":user.full_name.clone(),"username":user.username.clone(),"email":user.email.clone(),"password":user.password.clone(),"gender":user.gender.clone(),"phone":user.phone};
    match db.collection("users").insert_one(doc, None).await {
        Ok(_) => Html(read_to_string("pages/login.html").unwrap()),
        Err(e) => Html(e.to_string()),
    }
}
