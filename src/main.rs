use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json,
    Router
};
use serde::{
    Deserialize,
    Serialize
};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    match dotenv() {
        Ok(_) => println!(".env file loaded"),
        Err(err) => println!(".env file not found: {}", err),
    }

    let address = std::env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0:8080".to_string());

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/static", get("Hello, World!"))
        .route("/test/{name}", get(get_test))
        .route("/test", post(create_test));

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();

}


async fn get_test(
    Path(name): Path<String>,
) -> (StatusCode, Json<Message>) {
    (StatusCode::OK, Json(Message {
        message: format!("Hello, {}!", name),
    }))
}

async fn create_test(
    Json(payload): Json<CreateTest>,
) -> (StatusCode, Json<Test>) {
    let object = Test {
        id: 1,
        name: payload.name,
        age: payload.age,
    };

    (StatusCode::CREATED, Json(object))
}


#[derive(Deserialize)]
struct CreateTest {
    name: String,
    age: u8,
}

#[derive(Serialize)]
struct Test {
    id: u64,
    name: String,
    age: u8,
}

#[derive(Serialize)]
struct Message {
    message: String,
}