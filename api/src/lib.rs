#[macro_use]
extern crate diesel;
extern crate reqwest;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use reqwest::Client;

pub mod db;
pub mod model;
pub mod schema;

use self::model::api::*;
use self::model::teams::*;
use self::model::game_types::*;
use schema::teams;
use schema::game_types;

pub fn fetch_teams(client: &Client, conn: &MysqlConnection) -> Result<(), reqwest::Error> {
    const URL_TEAMS: &str = "https://statsapi.web.nhl.com/api/v1/teams";

    let resp: RespTeams = client
        .get(URL_TEAMS)
        .send()?
        .json()?;

    for api_team in &resp.teams {
        let team = NewTeam::from_api_team(&api_team);
        diesel::insert_into(teams::table)
            .values(&team)
            .execute(conn)
            .expect("Error saving new team");
    }

    Ok(())
}

pub fn fetch_game_types(client: &Client, conn: &MysqlConnection) -> Result<(), reqwest::Error> {
    const URL_GAME_TYPES: &str = "https://statsapi.web.nhl.com/api/v1/gameTypes";

    let mut resp = client.get(URL_GAME_TYPES).send()?;
    if let Ok(api_game_types) = resp.json::<Vec<RespGameType>>() {
        let valid = [String::from("PR"), String::from("R"), String::from("P")];
        for api_game_type in api_game_types {
            if !valid.contains(&api_game_type.id) {
                continue;
            }

            let game_type = NewGameType::from_api_game_type(&api_game_type);
            diesel::insert_into(game_types::table)
                .values(&game_type)
                .execute(conn)
                .expect("Error saving game types");
        }
    }

    let resp_game_type = RespGameType {
        id: "F".to_string(),
        description: "Finals game".to_string(),
        postseason: true
    };
    let game_type = NewGameType::from_api_game_type(&resp_game_type);
    diesel::insert_into(game_types::table)
        .values(&game_type)
        .execute(conn)
        .expect("Error saving finals game type");

    Ok(())
}


// use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// fn index() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// fn index2() -> impl Responder {
//     HttpResponse::Ok().body("Hello world again!")
// }

// fn main() {
//     HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(index))
//             .route("/again", web::get().to(index2))
//     })
//     .bind("0.0.0.0:8080")
//     .unwrap()
//     .run()
//     .unwrap();
// }


// use std::fs;
// use std::io::prelude::*;
// use std::net::TcpStream;
// use std::net::TcpListener;

// fn main() {
//     let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

//     for stream in listener.incoming() {
//         let stream = stream.unwrap();

//         println!("Connection established!");
//         handle_connection(stream);
//     }
// }


// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 512];
//     stream.read(&mut buffer).unwrap();

//     let contents = fs::read_to_string("hello.html").unwrap();

//     let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

//     stream.write(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
// }