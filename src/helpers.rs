use macroquad::math::Vec2;
use rand::Rng;

pub fn remap(value: f32, from_start: f32, from_end: f32, to_start: f32, to_end: f32) -> f32 {
	(value - from_start) / (from_end - from_start) * (to_end - to_start) + to_start
}
pub fn rand_map<R: Rng>(rng: &mut R, from: f32, to: f32) -> f32 {
	remap(rng.gen::<f32>(), 0.0, 1.0, from, to)
}

pub trait Wrapping {
	fn wrap(&mut self, wrap: &Self);
	fn wrapped(mut self, wrap: &Self) -> Self
	where
		Self: Sized,
	{
		self.wrap(wrap);
		self
	}
	fn wrap_add(&self, other: &Self, wrap: &Self) -> Self;
	fn wrap_sub(&self, other: &Self, wrap: &Self) -> Self;
}
impl Wrapping for Vec2 {
	fn wrap(&mut self, wrap: &Self) {
		while self.x > wrap.x {
			self.x -= wrap.x;
		}
		while self.x < 0.0 {
			self.x += wrap.x;
		}
		while self.y > wrap.y {
			self.y -= wrap.y;
		}
		while self.y < 0.0 {
			self.y += wrap.y;
		}
	}
	fn wrap_add(&self, other: &Self, wrap: &Self) -> Self {
		let new = *self + *other;
		new.wrapped(wrap)
	}
	fn wrap_sub(&self, other: &Self, wrap: &Self) -> Self {
		let curr = self.wrapped(wrap);
		let other = other.to_owned().wrapped(wrap);
		let others = [
			other,
			other + Vec2::new(wrap.x, 0.0),
			other + Vec2::new(wrap.x, wrap.y),
			other + Vec2::new(wrap.x, -wrap.y),
			other + Vec2::new(0.0, wrap.y),
			other + Vec2::new(0.0, -wrap.y),
			other + Vec2::new(-wrap.x, 0.0),
			other + Vec2::new(-wrap.x, wrap.y),
			other + Vec2::new(-wrap.x, -wrap.y),
		];

		others
			.into_iter()
			.map(|other| curr - other)
			.min_by(|a, b| a.length_squared().total_cmp(&b.length_squared()))
			.unwrap()
	}
}
