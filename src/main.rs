#[macro_use] extern crate nickel;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate fifa_app_api;
extern crate rustc_serialize;

use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult, MediaType};
use nickel::status::StatusCode;
use rustc_serialize::json;
use self::fifa_app_api::*;
use self::fifa_app_api::models::*;

#[derive(RustcDecodable, RustcEncodable)]
struct User {
    id: i32,
    name: String,
    wins: i32,
    losses: i32,
    elo: i32
}

#[derive(RustcDecodable, RustcEncodable)]
struct NewUser {
    name: String,
}

#[derive(RustcDecodable, RustcEncodable)]
struct Game {
    id: i32,
    winner_id: i32,
    loser_id: i32,
    winner_score: i32,
    loser_score: i32,
    winner_elo: i32,
    loser_elo: i32,
    date: i64,
    draw: bool,
}

#[derive(RustcDecodable, RustcEncodable)]
struct GameData {
    winner_id: i32,
    loser_id: i32,
    winner_score: i32,
    loser_score: i32,
}

fn main() {

    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/users", middleware! { |request, mut response|
        let connection = establish_connection();
        let users = get_users(&connection);
        let json = json::encode(&users).expect("Error encoding JSON.");
        response.set(MediaType::Json);
        response.set(StatusCode::Ok);
        return response.send(json);
    });

    router.post("/users", middleware! { |request, mut response|
        let user = request.json_as::<NewUser>().unwrap();
        let name = user.name.to_string();
        let connection = establish_connection();
        let new_user = create_user(&connection, &name);
        let json = json::encode(&new_user).expect("Error encoding JSON.");
        response.set(MediaType::Json);
        response.set(StatusCode::Ok);
        return response.send(json);
    });

    router.get("/users/:id", middleware! { |request, mut response|
        let connection = establish_connection();
        let user = get_user(&connection, request.param("id").expect("Error parsing string.").parse::<i32>().unwrap());
        let json = json::encode(&user).expect("Error encoding JSON.");
        response.set(MediaType::Json);
        response.set(StatusCode::Ok);
        return response.send(json);
    });

    router.get("/users/:id/games", middleware! { |request, mut response|
        let connection = establish_connection();
        let games = get_games(&connection, request.param("id").expect("Error parsing string.").parse::<i32>().unwrap());
        let json = json::encode(&games).expect("Error encoding JSON.");
        response.set(MediaType::Json);
        response.set(StatusCode::Ok);
        return response.send(json);
    });

    router.post("/games", middleware! { |request, response|
        let game = request.json_as::<GameData>().unwrap();
        let connection = establish_connection();
        create_game(&connection, game.winner_id, game.loser_id, game.winner_score, game.loser_score);

    });

    server.utilize(router);

    server.listen("127.0.0.1:6767");
}
