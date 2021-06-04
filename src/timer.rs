use std::time::Instant;

#[derive(Debug)]
pub struct Timer {
	start: Instant,
}

impl Timer {
	pub fn new() -> Timer {
		Timer {
			start: Instant::now(),
		}
	}

	pub fn stop(self) -> u128 {
		self.start.elapsed().as_millis()
	}
}
