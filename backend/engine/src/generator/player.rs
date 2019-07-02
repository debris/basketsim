use crate::models::player::NewPlayer;
use super::Seed;

const FIRST_NAMES: &'static [&'static str] = &[
	"LeBron", "Kevin", "Anthony", "James", "Stephen",
];

const LAST_NAMES: &'static [&'static str] = &[
	"James", "Durant", "Davis", "Harden", "Curry",
];

fn first_name(seed: u32) -> &'static str {
	FIRST_NAMES[seed as usize % FIRST_NAMES.len()]
}

fn last_name(seed: u32) -> &'static str {
	LAST_NAMES[seed as usize % LAST_NAMES.len()]
}

fn number_in_range(from: u8, to: u8, seed: u32) -> u8 {
	assert!(from <= to);
	let diff = to - from;
	from + ((seed % (diff as u32)) as u8)
}

fn age(seed: u32) -> u8 {
	number_in_range(18, 35, seed)
}

fn height(seed: u32) -> u8 {
	number_in_range(170, 230, seed)
}

fn weight(seed: u32) -> u8 {
	number_in_range(70, 150, seed)
}

pub fn generate(seed: u32) -> NewPlayer<'static> {
	let mut seed = Seed::new(seed);
	NewPlayer {
		team_id: None,
		first_name: first_name(seed.get()),
		last_name: last_name(seed.get()),
		age: age(seed.get()),
		height: height(seed.get()),
		weight: weight(seed.get()),		
	}
}
