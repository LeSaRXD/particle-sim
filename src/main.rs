mod helpers;
#[cfg(test)]
mod tests;

use std::sync::LazyLock;

use helpers::{rand_map, remap, Wrapping};
use macroquad::{
	color::*,
	color_u8,
	miniquad::window::screen_size,
	prelude::{clear_background, draw_circle, next_frame, Vec2},
	time::get_frame_time,
};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

static ATTRACTIONS: LazyLock<[f32; 25]> =
	LazyLock::new(|| std::array::from_fn(|_| rand_map(&mut thread_rng(), -2.0, 2.0)));

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ParticleKind {
	One = 0,
	Two = 1,
	Three = 2,
	Four = 3,
	Five = 4,
}
impl ParticleKind {
	const ALL: [ParticleKind; 5] = [Self::One, Self::Two, Self::Three, Self::Four, Self::Five];
	fn color(&self) -> Color {
		use ParticleKind::*;

		const COLOR1: Color = color_u8!(250, 131, 52, 255);
		const COLOR2: Color = color_u8!(143, 213, 166, 255);
		const COLOR3: Color = color_u8!(56, 134, 151, 255);
		const COLOR4: Color = color_u8!(142, 108, 136, 255);
		const COLOR5: Color = color_u8!(216, 30, 91, 255);

		match self {
			One => COLOR1,
			Two => COLOR2,
			Three => COLOR3,
			Four => COLOR4,
			Five => COLOR5,
		}
	}
	fn attraction(&self, other: &Self) -> f32 {
		let idx = *self as usize * Self::ALL.len() + *other as usize;
		ATTRACTIONS[idx]
	}
}

const REPULSION: f32 = -25.0;
const REPULSION_RANGE: f32 = 20.0;
const ATTRACTION_PEAK: f32 = 25.0;
const ATTRACTION_RANGE: f32 = 60.0;

#[derive(Debug, Clone, PartialEq)]
struct Particle {
	pos: Vec2,
	vel: Vec2,
	kind: ParticleKind,
}
impl Particle {
	fn new(x: impl Into<f32>, y: impl Into<f32>, kind: ParticleKind) -> Self {
		Self {
			pos: Vec2::new(x.into(), y.into()),
			vel: Vec2::ZERO,
			kind,
		}
	}
	fn color(&self) -> Color {
		self.kind.color()
	}
	fn draw(&self) {
		draw_circle(self.pos.x, self.pos.y, PARTICLE_RADIUS, self.color());
	}

	fn attraction(attraction: f32, distance: f32) -> f32 {
		match distance {
			small @ (0.0..REPULSION_RANGE) => remap(small, 0.0, REPULSION_RANGE, REPULSION, 0.0),
			mid @ (REPULSION_RANGE..ATTRACTION_PEAK) => {
				remap(mid, REPULSION_RANGE, ATTRACTION_PEAK, 0.0, attraction)
			}
			large @ (ATTRACTION_PEAK..ATTRACTION_RANGE) => {
				remap(large, ATTRACTION_PEAK, ATTRACTION_RANGE, attraction, 0.0)
			}
			_ => 0.0,
		}
	}
	fn accumilate_velocity(&mut self, other: &Self, dt: f32, wrap: &Vec2) {
		let mut dir = other.pos.wrap_sub(&self.pos, wrap);
		let attraction = Self::attraction(self.kind.attraction(&other.kind), dir.length());
		dir = dir.normalize_or_zero();
		dir *= attraction * dt * 10.0;
		self.vel += dir;
	}
	fn apply_velocity(&mut self, wrap: &Vec2) {
		self.pos = self.pos.wrap_add(&self.vel, wrap);
		self.vel = Vec2::ZERO;
	}
}

const SIM_ITERS: usize = 10;
const NUM_PARTICLES: u16 = 80;
const PARTICLE_RADIUS: f32 = 7.5;
const PARTICLE_BROWNIAN: f32 = 2.5;

#[macroquad::main("Particle sim")]
async fn main() {
	println!("{:?}", *ATTRACTIONS);

	let rng = &mut thread_rng();
	let (mut sx, mut sy) = screen_size();

	let mut particles: Box<_> = (0..=NUM_PARTICLES)
		.map(|_| {
			Particle::new(
				rng.gen::<f32>() * sx,
				rng.gen::<f32>() * sy,
				ParticleKind::ALL.choose(rng).unwrap().to_owned(),
			)
		})
		.collect();

	let mut particles_buf = Box::clone(&particles);

	loop {
		(sx, sy) = screen_size();
		let screen = Vec2::from(screen_size());
		let dt = get_frame_time().min(0.05);

		clear_background(BLACK);

		for _ in 0..SIM_ITERS {
			for (idx1, particle) in particles_buf.iter_mut().enumerate() {
				for (idx2, other) in particles.iter().enumerate() {
					if idx1 == idx2 {
						continue;
					}
					particle.accumilate_velocity(other, dt, &screen);
				}
				let mut brownian = || rand_map(rng, -PARTICLE_BROWNIAN, PARTICLE_BROWNIAN);
				particle.vel += Vec2::new(brownian(), brownian()) * dt;
				particle.apply_velocity(&screen);
			}
			particles.clone_from_slice(&particles_buf);
		}

		let offset_x = Vec2::new(sx, 0.0);
		let offset_y = Vec2::new(0.0, sy);

		for particle in &particles {
			particle.draw();
			if particle.pos.x < PARTICLE_RADIUS {
				let mut particle = particle.clone();
				particle.pos += offset_x;
				particle.draw();
			}
			if particle.pos.y < PARTICLE_RADIUS {
				let mut particle = particle.clone();
				particle.pos += offset_y;
				particle.draw();
			}
			if sx - particle.pos.x < PARTICLE_RADIUS {
				let mut particle = particle.clone();
				particle.pos -= offset_x;
				particle.draw();
			}
			if sy - particle.pos.y < PARTICLE_RADIUS {
				let mut particle = particle.clone();
				particle.pos -= offset_y;
				particle.draw();
			}
		}

		next_frame().await
	}
}
