use crate::model::api::RespGameType;
use crate::schema::game_types;

#[derive(Insertable)]
#[table_name="game_types"]
pub struct NewGameType<'a> {
    pub category: &'a str,
    pub description: &'a str,
    pub postseason: &'a bool,
    pub value: u32,
}

impl<'a> NewGameType<'a> {
    pub fn from_api_game_type(api_game_type: &RespGameType) -> NewGameType {
        let mut value: u32 = 0;
        match api_game_type.id.as_ref() {
            "PR" => value = 0,
            "R" => value = 5,
            "P" => value = 10,
            "F" => value = 20,
            _ => ()
        };

        NewGameType {
            category: &api_game_type.id,
            description: &api_game_type.description,
            postseason: &api_game_type.postseason,
            value: value
        }
    }
}

#[derive(Queryable)]
pub struct GameType {
    pub id: Option<u32>,
    pub category: String,
    pub description: String,
    pub postseason: bool,
    pub value: u32,
}