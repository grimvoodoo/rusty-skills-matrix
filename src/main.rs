use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use futures_util::StreamExt;
use mongodb::{
    bson::{doc, Document, Uuid},
    Client, Collection,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, time};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
struct User {
    id: Uuid,
    name: String,
    email: String,
    role: String,
    practice: String,
    skills: Vec<Skill>,
    created: time::SystemTime,
    updated: time::SystemTime,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
struct Skill {
    id: Uuid,
    name: String,
    description: String,
    level: i32,
    last_used: time::SystemTime,
    offset_months: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
struct MongoSkill {
    id: Uuid,
    name: String,
    description: String,
}

struct AppState {
    mongo_collection: Collection<Document>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Setting up Mongodb Connection");
    let mongo_collection: Collection<Document> = mongo_setup().await;
    println!("mongodb setup successfully");
    println!("Setting up actix_web on http:://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                mongo_collection: mongo_collection.clone(),
            }))
            .service(hello)
            .service(echo)
            .service(people)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn mongo_setup() -> Collection<Document> {
    let uri = env::var("MONGO_URI").unwrap_or("mongodb://localhost:27017".to_string());
    let client = Client::with_uri_str(&uri).await.unwrap();
    let database = client.database(&env::var("MONGO_DB").unwrap_or("skills-matrix".to_string()));
    database.collection(&env::var("MONGO_COLLECTION").unwrap_or("users".to_string()))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[get("/people")]
async fn people(data: web::Data<AppState>) -> impl Responder {
    let filter = doc! {};
    let mut cursor = data.mongo_collection.find(filter).await.unwrap();
    let mut users: Vec<Document> = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(user) => users.push(user),
            Err(_) => return HttpResponse::InternalServerError().body("Error fetching users"),
        }
    }

    HttpResponse::Ok().json(users)
}

#[post("/createSkill")]
async fn echo(req_body: web::Json<Vec<Skill>>) -> impl Responder {
    let new_skills: Vec<Skill> = Vec::new();
    let json = json!(req_body);

    HttpResponse::Ok().json(json)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
