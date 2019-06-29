use diesel::{Insertable, Queryable, Identifiable};
use crate::schema::users;

#[derive(Debug, Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
	pub email: &'a str,
	pub name: &'a str,
	/// Hashes password.
	pub password: &'a str,
}

#[derive(Debug, Queryable, Identifiable)]
#[table_name="users"]
pub struct User {
	pub id: i32,
	pub email: String,
	pub name: String,
	pub password: String,
}
