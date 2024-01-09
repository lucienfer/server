use serde::Serialize;

#[derive (Serialize)]
struct User {
    name: String,
    pwd: String,
}

