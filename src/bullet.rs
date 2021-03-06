use tetra::glm::Vec2;

#[derive(Debug)]
pub struct Bullet{
	position: Vec2,
	area: (i32,i32),
	velocity: Vec2,
	broken: bool,
	force: i32,
	kind: i32,
}

impl Bullet{
	pub fn new(position: Vec2, force: i32, kind: i32) -> Self{
		let velocity = Vec2::new(0.0,-3.0);
		let area = (4,4);
		Bullet{
			position,
			area,
			velocity,
			broken: false,
			force,
			kind,
		}
	}
	pub fn update(&mut self){
		if !self.broken{
			self.position += self.velocity;
		}

		if self.position.y < -30.0{
			if self.position.x <= 120.0{
				self.position.x = 20.0;
			}else{
				self.position.x = 220.0;
			}
			self.velocity = Vec2::new(0.0, self.velocity.y * -1.0);
		}
	}

	pub fn get_area(&self) -> (i32,i32){
		self.area
	}

	pub fn get_kind(&self) -> i32{
		self.kind
	}

	pub fn get_position(&self) -> Vec2{
		self.position
	}

	pub fn is_broken(&self) -> bool{
		self.broken
	}

	pub fn set_broken(&mut self ){
		self.broken = true;
	}

	pub fn consume_force(&mut self){
		if self.force >= 1{
			self.force -=1;
		}
		if self.force == 0{
			self.broken = true;
		}
	}

	pub fn is_returning(&self) -> bool{
		self.velocity.y >= 0.0
	}

	pub fn get_force(&self) -> i32{
		self.force
	}

	pub fn set_velocity(&mut self, velocity: Vec2){
		self.velocity = velocity;
	}
}

pub fn get_velocity_from_force(force: i32)-> Vec2{
	match force{
		1 => Vec2::new(0.0,-3.0),
		2 => Vec2::new(0.0,-5.0),
		3 => Vec2::new(0.0,-8.0),
		_ => Vec2::new(0.0,-10.0),
	}
}