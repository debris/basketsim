#[macro_use]
extern crate diesel;

mod league;
mod player;
mod schema;
mod team;
mod user;

use std::io;
pub use league::NewLeague;
pub use team::NewTeam;
pub use user::NewUser;
pub use player::NewPlayer;

pub struct Engine;

impl Engine {
	pub fn new_user<'a>(&mut self, _user: NewUser<'a>) -> io::Result<i32> {
		unimplemented!();
	}

	pub fn new_league(&mut self, _league: NewLeague) -> io::Result<i32> {
		unimplemented!();
	}

	pub fn new_team<'a>(&mut self, _team: NewTeam<'a>) -> io::Result<i32> {
		unimplemented!();
	}

	pub fn new_player<'a>(&mut self, _player: NewPlayer<'a>) -> io::Result<i32> {
		unimplemented!();
	}
}
