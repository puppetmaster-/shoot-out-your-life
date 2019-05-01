use tetra::glm::Vec2;

#[derive(Debug)]
pub struct Bullet{
	position: Vec2,
	area: (i32,i32),
	velocity: Vec2,
	lifetime: i32,
	tick: i32,
	broken: bool,
	force: i32,
}

impl Bullet{
	pub fn new(position: Vec2, force: i32) -> Self{
		let velocity = Vec2::new(0.0,-3.0);
		let area = (4,4);
		let lifetime = 500;
		Bullet{
			position,
			area,
			velocity,
			lifetime,
			tick: 0,
			broken: false,
			force,
		}
	}
	pub fn update(&mut self){
		self.tick += 1;
		if self.lifetime < self.tick{
			self.broken = true;
		}
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
		}else if self.position.y >= 392.0{
			self.velocity = Vec2::new(0.0,0.0);
		}
	}

	pub fn get_area(&self) -> (i32,i32){
		self.area
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

	pub fn get_velocity(&self) -> Vec2{
		self.velocity
	}

	pub fn consume_force(&mut self){
		if self.force >= 1{
			self.force -=1;
		}
		if self.force == 0{
			self.broken = true;
			self.velocity = Vec2::new(0.0,0.0);
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