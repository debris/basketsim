use crate::models::team::NewTeam;
use super::Seed;

const TEAM_NAMES: &'static [&'static str] = &[
	"Boston Celtics",
	"LA Lakers",
	"Toronto Raptors",
	"Real Madrid",
	"FC Barcelona",
	"CSKA Moscow",
	"Olympiacos Piraeus",
	"Fenerbahce Istanbul",
];

fn team_name(seed: u32) -> &'static str {
	TEAM_NAMES[seed as usize % TEAM_NAMES.len()]
}

pub fn generate(seed: u32) -> NewTeam<'static> {
	let mut seed = Seed::new(seed);
	NewTeam {
		user_id: None,
		league_id: None,
		name: team_name(seed.get()),
	}
}
