// src/main.rs - Rocket application
#[macro_use]
extern crate rocket;

mod cors;
use cors::Cors;

use rocket::serde::json::Json;
use rocket_db_pools::sqlx::Row;
use rocket_db_pools::{sqlx, Connection, Database};
use serde::Serialize;

#[derive(Database)]
#[database("mydb")]
struct MyDb(sqlx::PgPool);

#[derive(Serialize)]
struct Message {
    message: String,
}

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
}

#[derive(Serialize)]
struct Health {
    status: String,
    database: String,
}

#[get("/")]
fn index() -> Json<Message> {
    Json(Message {
        message: "Hello from Rocket!".to_string(),
    })
}

#[get("/users")]
async fn list_users(mut db: Connection<MyDb>) -> Json<Vec<User>> {
    let users = sqlx::query("SELECT id, name FROM users")
        .fetch_all(&mut **db)
        .await
        .map(|rows| {
            rows.into_iter()
                .map(|row| User {
                    id: row.get("id"),
                    name: row.get("name"),
                })
                .collect()
        })
        .unwrap_or_default();

    Json(users)
}

#[get("/health")]
async fn health(mut db: Connection<MyDb>) -> Json<Health> {
    match sqlx::query("SELECT 1").execute(&mut **db).await {
        Ok(_) => Json(Health {
            status: "ok".to_string(),
            database: "connected".to_string(),
        }),
        Err(_) => Json(Health {
            status: "degraded".to_string(),
            database: "error".to_string(),
        }),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(MyDb::init())
        .attach(Cors)
        .mount("/", routes![index, list_users, health])
}
