extern crate fifa_app_api;
extern crate diesel;

use self::fifa_app_api::*;
use self::fifa_app_api::models::*;
use self::diesel::prelude::*;

fn main() {
    use fifa_app_api::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.name);
    }
}
