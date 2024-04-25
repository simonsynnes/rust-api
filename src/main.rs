use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct StatusMessage {
    message: String,
}

async fn get_users() -> impl Responder {
    HttpResponse::Ok().json(vec![
        User {
            id: 1,
            name: "Simon".into(),
            email: "simonsynnes@gmail.com".into(),
        },
        User {
            id: 2,
            name: "John".into(),
            email: "john@example.com".into(),
        },
    ])
}

async fn get_user(user_id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().json(User {
        id: *user_id,
        name: "Charles".into(),
        email: "charles@example.com".into(),
    })
}

async fn add_user(user: web::Json<User>) -> impl Responder {
    HttpResponse::Created().json(StatusMessage {
        message: format!("Added user: {}", user.name),
    })
}

async fn update_user(user_id: web::Path<u32>, user: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().json(StatusMessage {
        message: format!("Updated user with ID: {}", user_id),
    })
}

async fn delete_user(user_id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().json(StatusMessage {
        message: format!("Deleted user with ID: {}", user_id),
    })
}
fn main() {}
