use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RespTeam {
    pub id: u32,
    pub abbreviation: String,
    pub location_name: String,
    pub team_name: String
}

#[derive(Deserialize)]
pub struct RespTeams {
    pub teams: Vec<RespTeam>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RespGameType {
    pub id: String,
    pub description: String,
    pub postseason: bool
}