use diesel::prelude::*;
use crate::schema::scans;

#[derive(Queryable)]
pub struct Scan {
    pub id: i32,
    pub ip: String,
    pub region: String,
}

#[derive(Insertable)]
#[diesel(table_name = scans)]
pub struct NewScan<'a> {
    pub ip: &'a str,
    pub region: &'a str,
}