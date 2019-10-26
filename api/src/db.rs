use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

static DATABASE_URL: &'static str = env!("DATABASE_URL");

pub fn establish_connection() -> MysqlConnection {
    MysqlConnection::establish(DATABASE_URL)
        .expect(&format!("Error connecting to {}", DATABASE_URL))
}