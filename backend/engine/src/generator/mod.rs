pub mod player;

struct Seed(u32);

impl Seed {
	fn new(seed: u32) -> Self {
		Seed(seed)
	}

	fn get(&mut self) -> u32 {
		let result = self.0;
		self.0 = self.0.rotate_left(1);
		result
	}
}
