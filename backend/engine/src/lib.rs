#[macro_use]
extern crate diesel;

mod database;
mod generator;
mod models;
mod schema;

use std::io;
use database::Database;

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
}

