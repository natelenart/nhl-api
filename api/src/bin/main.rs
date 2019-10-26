extern crate diesel;
extern crate reqwest;
extern crate nhl_api;

use reqwest::Client;
use nhl_api::*;

fn main() {
    let client = Client::new();
    let conn = db::establish_connection();

    fetch_game_types(&client, &conn).expect("Fetch game types failed");
    fetch_teams(&client, &conn).expect("Fetch teams failed");
}