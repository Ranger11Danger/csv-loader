pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::*;
use diesel::prelude::*;
use self::models::{NewScan, Scan};
use serde::Deserialize;
use serde_json::{Deserializer, Value};
use std::fs;
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn create_post(conn: &mut PgConnection, ip: &str, region: &str) -> Scan {
    use crate::schema::scans;

    let new_post = NewScan { ip, region };

    diesel::insert_into(scans::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}


#[derive(Deserialize, Debug)]
struct location {
    region : String
}

#[derive(Deserialize, Debug)]
struct data {
    ip_str: String,
    _shodan: location,
}

pub fn read_file(conn: &mut PgConnection) {
    
    let data = fs::read_to_string("/home/josh/Downloads/data.json").expect("test test");
    let objects= serde_json::Deserializer::from_str(&data).into_iter::<Value>();
    for object in objects{
        let scan: data = serde_json::from_value(object.unwrap()).unwrap();
        create_post(conn, &scan.ip_str, &scan._shodan.region);
    }
}

fn main() {
    use self::schema::scans::dsl::*;
    
    let connection = &mut establish_connection();
    //read_file(connection);
    //create_post(connection, "test", "AR");
    let results = scans
        .filter(region.eq("na"))
        .limit(5)
        .load::<Scan>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for scan in results {
        println!("{}", scan.ip);
        println!("-----------\n");
        println!("{}", scan.region);
    }
}