
#[derive(Queryable, RustcDecodable, RustcEncodable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub wins: i32,
    pub losses: i32,
    pub elo: i32,
    pub draws: i32,
}

#[derive(Queryable, RustcDecodable, RustcEncodable)]
pub struct Game {
    pub id: i32,
    pub winner_id: i32,
    pub loser_id: i32,
    pub winner_elo: i32,
    pub loser_elo: i32,
    pub winner_score: i32,
    pub loser_score: i32,
    pub date: i64,
    pub draw: bool,
}

use super::schema::users;
use super::schema::games;

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
}

#[derive(Insertable)]
#[table_name="games"]
pub struct NewGame {
    pub winner_id: i32,
    pub loser_id: i32,
    pub winner_elo: i32,
    pub loser_elo: i32,
    pub winner_score: i32,
    pub loser_score: i32,
    pub date: i64,
    pub draw: bool,
}
