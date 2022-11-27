use actix_web::{web, get, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Board {
    id: String,
    content: String,
    writer_id: String,
}

#[get("/board/{board_id}")]
async fn get_board(board_id: web::Path<String>) -> impl Responder {
    println!("get_board {}", board_id.to_string());
    HttpResponse::Ok().json(Board {
        id: board_id.to_string(),
        content: String::from("test"),
        writer_id: String::from("test")
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_board))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}