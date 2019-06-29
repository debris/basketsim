use diesel::{Insertable, Queryable, Identifiable};
use crate::schema::players;

#[derive(Debug, Insertable)]
#[table_name="players"]
pub struct NewPlayer<'a> {
	pub team_id: Option<i32>,
	pub first_name: &'a str,
	pub last_name: &'a str,
	pub age: u8,
	pub height: u8,
	pub weight: u8,
}

#[derive(Debug, Queryable, Identifiable)]
#[table_name="players"]
pub struct Player {
	pub id: i32,
	pub team_id: Option<i32>,
	pub first_name: String,
	pub last_name: String,
	pub age: u8,
	pub height: u8,
	pub weight: u8,
}
