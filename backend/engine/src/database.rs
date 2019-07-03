use std::io;
use diesel::connection::{Connection, AnsiTransactionManager, TransactionManager};
use diesel::mysql::MysqlConnection;
use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};
use schema::{players, teams};

use crate::schema;
use crate::models::{
	league::NewLeague,
	team::{NewTeam, Team},
	user::NewUser,
	player::{NewPlayer, Player}
};

pub struct Database {
	connection: MysqlConnection,
}

impl Database {
	pub fn new(database_url: &str) -> io::Result<Self> {
		let connection = MysqlConnection::establish(database_url).expect("TODO");

		let engine = Database {
			connection,
		};

		Ok(engine)
	}

	pub fn execute_batch<T, F: Fn(&mut Database) -> io::Result<T>>(&mut self, batch: F) -> io::Result<T> {
		let transaction = AnsiTransactionManager::new();
		transaction.begin_transaction(&self.connection).expect("TODO");
		match batch(self) {
			Ok(result) => {
				transaction.commit_transaction(&self.connection).expect("TODO");
				Ok(result)
			},
			Err(err) => {
				transaction.rollback_transaction(&self.connection).expect("TODO");
				Err(err)
			}
		}
	}

	pub fn new_user<'a>(&mut self, _user: NewUser<'a>) -> io::Result<i32> {
		unimplemented!();
	}

	pub fn new_league(&mut self, _league: NewLeague) -> io::Result<i32> {
		unimplemented!();
	}

	pub fn new_team<'a>(&mut self, team: NewTeam<'a>) -> io::Result<i32> {
		diesel::insert_into(teams::table)
			.values(team)
			.execute(&self.connection)
			.expect("TODO");

		let team: Team = teams::table.order(teams::id.desc())
			.first(&self.connection)
			.expect("TODO");

		Ok(team.id)
	}

	pub fn get_team(&self, id: i32) -> io::Result<Team> {
		let team = teams::table.find(id)
			.first(&self.connection)
			.expect("TODO");

		Ok(team)
	}

	/// Assigns user to a first team without an owner.
	/// Returns `Ok(None)` if such team does not exist.
	pub fn assign_user_to_a_team(&mut self, _user_id: i32) -> io::Result<Option<i32>> {
		unimplemented!();
	}

	pub fn new_player<'a>(&mut self, player: NewPlayer<'a>) -> io::Result<i32> {
		diesel::insert_into(players::table)
			.values(player)
			.execute(&self.connection)
			.expect("TODO");

		let player: Player = players::table.order(players::id.desc())
			.first(&self.connection)
			.expect("TODO");

		Ok(player.id)
	}

	pub fn get_player(&self, id: i32) -> io::Result<Player> {
		let player = players::table.find(id)
			.first(&self.connection)
			.expect("TODO");

		Ok(player)
	}
}

#[cfg(test)]
mod tests {
	use std::env;
	use dotenv::dotenv;
	use super::*;

	fn test_database_url() -> String {
		dotenv().ok();
		env::var("DATABASE_URL").unwrap()
	}

	#[test]
	fn test_new_player() {
		let first_name = "first";
		let last_name = "last";
		let age = 26;

		let new_player = NewPlayer {
			team_id: None,
			first_name,
			last_name,
			age,
			height: 189,
			weight: 91,
		};

		let mut engine = Database::new(&test_database_url()).unwrap();
		let id = engine.new_player(new_player).unwrap();
		let player = engine.get_player(id).unwrap();

		assert_eq!(first_name, player.first_name);
		assert_eq!(last_name, player.last_name);
		assert_eq!(age, player.age);
	}
}
