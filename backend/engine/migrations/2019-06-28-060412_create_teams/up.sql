CREATE TABLE teams (
	id INTEGER AUTO_INCREMENT PRIMARY KEY,
	user_id INTEGER REFERENCES users (id),
	league_id INTEGER REFERENCES leagues (id),
	name TEXT NOT NULL
)