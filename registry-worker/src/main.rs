use std::env;

use diesel::prelude::*;
use dotenvy::dotenv;
use model::{NewPlatform, Platform};

mod model;
mod schema;

pub fn create_post(conn: &mut PgConnection, name: &str) -> Platform {
    use crate::schema::platforms;

    let new_platform = NewPlatform { id: 1, name };

    diesel::insert_into(platforms::table)
        .values(&new_platform)
        .returning(Platform::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let platform = create_post(&mut conn, "x86_64-linux");

    println!("Hello, world!, {:?}", platform);
}
