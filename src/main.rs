use actix_web::{web, get, post, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Board {
    id: String,
    creator_id: String,
    card_list: Vec<Card>,
    board_title: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Card {
    id: String,
    board_id: String,
    writer_id: String,
    writer_name: String,
    contents: String
}


#[derive(Debug, Serialize, Deserialize, Clone)]
struct BoardInput {
    creator_id: String,
    board_title: String
}

#[get("/board/{board_id}")]
async fn get_board(board_id: web::Path<String>) -> impl Responder {
    println!("get_board {}", board_id.to_string());
    HttpResponse::Ok().json(Board {
        id: board_id.to_string(),
        creator_id: String::from("test"),
        card_list: [Card{
            id: "00000000".to_string(),
            board_id: board_id.to_string(),
            writer_id: "00000000".to_string(),
            writer_name: "Yuri".to_string(),
            contents: "Thank you!".to_string()
        }].to_vec(),
        board_title: String::from("Alice")
    })
}

#[get("/card/{card_id}")]
async fn get_card(card_id: web::Path<String>) -> impl Responder {
    println!("get_card {}", card_id.to_string());
    HttpResponse::Ok().json(Card{
            id: card_id.to_string(),
            board_id: "00000000".to_string(),
            writer_id: "00000000".to_string(),
            writer_name: "Yuri".to_string(),
            contents: "Thank you!".to_string()
    })
}


#[post("/card")]
async fn post_card(cardinfo: web::Json<Card>) -> impl Responder {
    println!("post_card");
    HttpResponse::Ok().json({})
}

#[post("/board")]
async fn post_board(info: web::Json<BoardInput>) -> impl Responder {
    println!("post_board");
    HttpResponse::Ok().json(Board{id: "00000000".to_string(),
                                  board_title: "Hi".to_string(),
                                  card_list: [].to_vec(),
                                  creator_id: "00000000".to_string()})
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_board).service(get_card).service(post_card).service(post_board))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}