mod minimax;

#[macro_use]
extern crate rocket;

use include_dir::{include_dir, Dir};
use std::path::Path;
use std::time::Instant;
use rocket::form::Form;

static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static_build");

use rocket_contrib::serve::StaticFiles;
use rocket::fs::{FileServer, relative};
use rocket::response::content;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::{Json, json};
use rocket::serde::json::serde_json::Value;

use shakmaty::{Chess, Move, perft, Position, Role, Square, fen::Fen, CastlingMode, EnPassantMode, san::San};

// fn main() {
//     let mut pos = Chess::default();
//     let mut legals = pos.legal_moves();
//     for m in &legals {
//         println!("{}", m);
//     }
//     println!("{}", pos.board());
//     // pos = pos.play_unchecked(&legals.remove(5));
//     pos.play_unchecked(&Move::Normal {
//         role: Role::Pawn,
//         from: Square::E2,
//         to: Square::E4,
//         capture: None,
//         promotion: None,
//     });
//     println!("{}", pos.board());
//     println!("{}", perft(&pos, 6));
//
// }

#[derive(FromForm)]
struct MakeMoveForm<'r> {
    fen: &'r str,
}

#[get("/")]
fn index() -> content::RawHtml<&'static str> {
    content::RawHtml(PROJECT_DIR.get_file("index.html").unwrap().contents_utf8().unwrap())
}

#[post("/make_move", data = "<input>")]
fn make_move(input: Form<MakeMoveForm<'_>>) -> Value {
    println!("Input FEN {}", input.fen);
    let fen: Fen = input.fen.parse()
        .expect("Couldn't parse FEN input.");
    let mut pos: Chess = fen.into_position(CastlingMode::Standard)
        .expect("Couldn't convert FEN to chess position.");
    let num_moves = pos.legal_moves().len();
    println!("Num moves {}", num_moves);
    let now = Instant::now();
    let m = minimax::minimax(&pos, 6)
        .expect("Failed to minimax");
    let minimax_time = now.elapsed().as_millis().to_string();
    let m_san_str = San::from_move(&pos, &m).to_string();
    println!("{}", m.to_string());
    pos.play_unchecked(&m);
    let output_fen = Fen::from_position(pos, EnPassantMode::Legal).to_string();
    json!({
        "best_move": m_san_str,
        "time": minimax_time,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/static", FileServer::from(relative!("static")))
        .mount("/", routes![index, make_move])
}
