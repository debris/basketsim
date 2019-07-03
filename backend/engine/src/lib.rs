#[macro_use]
extern crate diesel;

mod database;
mod generator;
mod models;
mod schema;

use std::io;
use database::Database;
use models::{
	user::NewUser,
	league::NewLeague,
};

pub struct Settings<'a> {
	database_url: &'a str,
}

pub struct Game {
	db: Database,
}

impl Game {
	pub fn start(settings: Settings) -> io::Result<Game> {
		let game = Game {
			db: Database::new(settings.database_url)?,
		};

		Ok(game)
	}

	pub fn register_new_user(&mut self, email: &str, name: &str, password: &str) -> io::Result<i32> {
		const TEAMS_IN_LEAGUE: usize = 16;
		const PLAYERS_IN_TEAM: usize = 12;

		let user_id = self.db.execute_batch(|database| {
			let user_id = database.new_user(NewUser {
				email,
				name,
				password
			})?;

			if let Some(_team_id) = database.assign_user_to_a_team(user_id)? {
				return Ok(user_id)
			}

			let tier = 0;
			let league_id = database.new_league(NewLeague { tier })?;

			for i in 0..TEAMS_IN_LEAGUE {
				let owner_id = match i {
					0 => Some(user_id),
					_ => None,
				};

				let seed = 0;
				let mut team = generator::team::generate(seed);
				team.league_id = Some(league_id);
				team.user_id = owner_id;

				let team_id = database.new_team(team)?;

				for _j in 0..PLAYERS_IN_TEAM {
					let mut player = generator::player::generate(seed);
					player.team_id = Some(team_id);

					let _player_id = database.new_player(player)?;
				}
			}
			

			
			Ok(user_id)
		})?;

		Ok(user_id)
	}
}

