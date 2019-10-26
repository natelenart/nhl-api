use std::fmt;
use crate::model::api::RespTeam;
use crate::schema::teams;

#[derive(Insertable)]
#[table_name="teams"]
pub struct NewTeam<'a> {
    pub nhl_id: &'a u32,
    pub abbreviation: &'a str,
    pub location: &'a str,
    pub name: &'a str,
}

impl<'a> NewTeam<'a> {
    pub fn from_api_team(api_team: &RespTeam) -> NewTeam {
        NewTeam {
            nhl_id: &api_team.id,
            abbreviation: &api_team.abbreviation,
            location: &api_team.location_name,
            name: &api_team.team_name
        }
    }
}

impl<'a> fmt::Display for NewTeam<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", &self.location, &self.name)
    }
}

impl<'a> fmt::Debug for NewTeam<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} (id: {}, abbr: {})", &self.location, &self.name, &self.nhl_id, &self.abbreviation)
    }
}

#[derive(Queryable)]
pub struct Team {
    pub id: Option<u32>,
    pub nhl_id: u32,
    pub abbreviation: String,
    pub location: String,
    pub name: String
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", &self.location, &self.name)
    }
}

impl fmt::Debug for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} (id: {}, abbr: {})", &self.location, &self.name, &self.nhl_id, &self.abbreviation)
    }
}