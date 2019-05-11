use tetra::glm::Vec2;
use tetra::graphics::{Color};
use crate::assets::TextureName;


//#[derive(Debug, PartialEq, PartialOrd)]
#[allow(dead_code)]
pub struct Particle{
	position: Vec2,
	velocity: Vec2,
	acceleration: Vec2,
	offset: Vec2,
	aging: f32,
	age: f32,
	dead: bool,
	kind: i32,
	texture_name: TextureName,
	color: Color,
}

#[allow(dead_code)]
impl Particle{
	pub fn new(position: Vec2, velocity: Vec2) -> Self{
		Particle{
			position,
			velocity,
			acceleration: Vec2::new(0.0,0.0),
			offset: Vec2::new(0.0,0.0),
			aging: 0.005,
			age: 0.0,
			dead: false,
			kind: 0,
			texture_name: TextureName::Particle0,
			color: Color::rgba(1.0, 1.0, 1.0, 1.0),
		}
	}

	pub fn set_position(self, position: Vec2) -> Self{
		Particle { position, ..self}
	}

	pub fn set_velocity(self, velocity: Vec2) -> Self{
		Particle { velocity, ..self}
	}

	pub fn set_acceleration(self, acceleration: Vec2) -> Self{
		Particle { acceleration, ..self}
	}

	pub fn set_offset(self, offset: Vec2) -> Self{
		Particle { offset, ..self}
	}

	pub fn set_aging(self, aging: f32) -> Self{
		Particle { aging, ..self}
	}

	pub fn set_kind(self, kind: i32) -> Self{
		Particle { kind, ..self}
	}

	pub fn set_texture_name(self, texture_name: TextureName) -> Self{
		Particle {texture_name, ..self}
	}

	pub fn is_dead(&self) -> bool{
		self.dead
	}

	pub fn get_age(&self) -> f32{
		self.age
	}

	pub fn get_position(&self) -> Vec2{
		self.position
	}

	pub fn get_offset(&self) -> Vec2{
		self.offset
	}

	pub fn get_kind(&self) -> i32{
		self.kind
	}

	pub fn get_texture_name(&self) -> &TextureName{
		&self.texture_name
	}

	pub fn get_velocity(&self) -> Vec2{
		self.velocity
	}

	pub fn get_color(&self) -> Color{
		let a = if self.age > 0.6 {
			1.0 / 0.4 * (1.0 - self.age)
		}else{
			1.0
		};
		Color::rgba(1.0,1.0,1.0, a)
	}

	pub fn update(&mut self){
		self.age += self.aging;
		if self.age > 1.0{
			self.dead = true;
		}
		if !self.dead {
			self.position += self.velocity + self.acceleration;
		}
	}
}

