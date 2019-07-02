use diesel::{Insertable, Queryable, Identifiable};
use crate::schema::teams;

#[derive(Debug, Insertable)]
#[table_name="teams"]
pub struct NewTeam<'a> {
	pub user_id: Option<i32>,
	pub league_id: Option<i32>,
	pub name: &'a str,
}

#[derive(Debug, Queryable, Identifiable)]
#[table_name="teams"]
pub struct Team {
	pub id: i32,
	pub user_id: Option<i32>,
	pub league_id: Option<i32>,
	pub name: String,
}
