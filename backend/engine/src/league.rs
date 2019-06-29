use diesel::{Insertable, Queryable, Identifiable};
use crate::schema::leagues;

#[derive(Debug, Insertable)]
#[table_name="leagues"]
pub struct NewLeague {
	pub tier: u8,
}

#[derive(Debug, Queryable, Identifiable)]
#[table_name="leagues"]
pub struct League {
	pub id: i32,
	pub tier: u8,
}
