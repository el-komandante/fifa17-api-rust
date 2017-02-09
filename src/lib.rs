
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

pub mod schema;
pub mod models;

extern crate rustc_serialize;
extern crate dotenv;
extern crate chrono;

use diesel::prelude::*;
use diesel::pg::PgConnection;
// use dotenv::dotenv;
use std::env;
use self::models::{User, NewUser, Game, NewGame};

pub fn establish_connection() -> PgConnection {
    // dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn calc_elo(conn: &PgConnection, winner_id: i32, loser_id: i32, draw: bool, k: i32) -> (i32, i32) {
    use std::f32;
    let constant: f32 = 10.0;

    let winner = get_user(conn, winner_id);
    let loser = get_user(conn, loser_id);

    let expected_winner_prob = 1.0 / (1.0 + constant.powf((loser.elo as f32 - winner.elo as f32) / 400.0));
    let expected_loser_prob = 1.0 / (1.0 + constant.powf((winner.elo as f32 - loser.elo as f32) / 400.0));

    let winner_score = if draw { 0.5 } else { 1.0 };
    let loser_score = if draw { 0.5 } else { 0.0 };

    let new_winner_elo = (winner.elo as f32 + k as f32 * (winner_score as f32 - expected_winner_prob)).round() as i32;
    let new_loser_elo = (loser.elo as f32 + k as f32 * (loser_score as f32 - expected_loser_prob)).round() as i32;

    println!("Difference: {} Winner: {} -> {} Loser: {} -> {}", (winner.elo - loser.elo).abs(), winner.elo, new_winner_elo as i32, loser.elo, new_loser_elo as i32);
    (new_winner_elo, new_loser_elo)
}

pub fn create_user<'a>(conn: &PgConnection, name: &'a str) -> User {
    use schema::users;

    let new_user = NewUser {
        name: name,
    };

    let user = diesel::insert(&new_user).into(users::table)
        .get_result(conn)
        .expect("Error creating new user.");
    user
}

pub fn update_elo(conn: &PgConnection, user_id: i32, new_elo: i32) {
    use self::schema::users::dsl::*;
    diesel::update(users.find(user_id))
        .set(elo.eq(new_elo))
        .execute(conn)
        .expect(&format!("Unable to find user with id {}", user_id));
}

pub fn  add_win(conn: &PgConnection, user_id: i32) {
    use self::schema::users::dsl::*;
    let user = users.filter(id.eq(user_id))
        .get_result::<User>(conn)
        .expect("Error loading user.");
    diesel::update(users.find(user_id))
        .set(wins.eq(user.wins + 1));
}

pub fn  add_loss(conn: &PgConnection, user_id: i32) {
    use self::schema::users::dsl::*;
    let user = users.filter(id.eq(user_id))
        .get_result::<User>(conn)
        .expect("Error loading user.");
    diesel::update(users.find(user_id))
        .set(losses.eq(user.losses + 1));
}

pub fn  add_draw(conn: &PgConnection, user_id: i32) {
    use self::schema::users::dsl::*;
    let user = users.filter(id.eq(user_id))
        .get_result::<User>(conn)
        .expect("Error loading user.");
    diesel::update(users.find(user_id))
        .set(draws.eq(user.draws + 1));
}

pub fn get_user(conn: &PgConnection, user_id: i32)-> User {
    use self::schema::users::dsl::*;
    let user = users.filter(id.eq(user_id))
        .get_result::<User>(conn)
        .expect("Error loading user.");
    user
}

pub fn get_users(conn: &PgConnection)-> Vec<User> {
    use self::schema::users::dsl::*;
    let all_users = users
        .get_results::<User>(conn)
        .expect("Error loading users.");
    all_users
}

pub fn get_games(conn: &PgConnection, user_id: i32) -> Vec<Game> {
    use self::schema::games::dsl::*;
    let all_games = games.filter(id.eq(user_id))
        .get_results::<Game>(conn)
        .expect("Error loading games.");
    all_games
}

pub fn create_game<'a>(conn: &PgConnection, winner_id: i32, loser_id: i32, winner_score: i32, loser_score: i32) -> Game {
    use chrono::*;
    use schema::games;
    let game_date: i64 = UTC::now().timestamp();
    let draw = if winner_score == loser_score { true } else { false };
    let (new_winner_elo, new_loser_elo) = calc_elo(conn, winner_id, loser_id, draw, 20);

    if draw {
        add_draw(conn, winner_id);
        add_draw(conn, loser_id);
    } else {
        add_win(conn, winner_id);
        add_loss(conn, loser_id);
    }

    update_elo(conn, winner_id, new_winner_elo);
    update_elo(conn, loser_id, new_loser_elo);

    let new_game = NewGame {
        winner_id: winner_id,
        loser_id: loser_id,
        winner_score: winner_score,
        loser_score: loser_score,
        winner_elo: new_winner_elo,
        loser_elo: new_loser_elo,
        date: game_date,
        draw: draw,
    };

    diesel::insert(&new_game).into(games::table)
        .get_result(conn)
        .expect("Error creating new game.")
}

// pub fn authenticator<'mw>(request: &mut Request, response : Response<'mw>, )-> MiddlewareResult<'mw> {
//     if request.origin.method.to_string == "POST".to_string() && request.origin.uri.to_string() == "/users".to_string() {
//
//     } else {
//         response.next_middleware()
//     }
// }
