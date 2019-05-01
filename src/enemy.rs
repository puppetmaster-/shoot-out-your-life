use tetra::glm::Vec2;

#[derive(Debug)]
pub struct Enemy{
	position: Vec2,
	area: (i32,i32),
	velocity: Vec2,
	lifetime: i32,
	tick: i32,
	dead: bool,
	life: i32,
}

impl Enemy{
	pub fn new(position: Vec2) -> Self{
		let velocity = Vec2::new(0.0,1.0);
		let lifetime = 1000;
		Enemy{
			position,
			area: (19, 17),
			velocity,
			lifetime,
			tick: 0,
			dead: false,
			life: 1
		}
	}

	pub fn update(&mut self){
		self.tick += 1;
		if self.lifetime < self.tick{
			self.dead = true;
		}
		if !self.dead {
			self.position += self.velocity;
		}

		// bottom same as player y position
		if self.position.y >= 392.0{
			self.velocity = Vec2::new(0.0,0.0);
		}

	}

	pub fn set_life(&mut self, life: i32){
		self.life = life;
	}

	pub fn get_area(&self) -> (i32,i32){
		self.area
	}

	pub fn get_position(&self) -> Vec2{
		self.position
	}

	pub fn is_dead(&self) -> bool{
		self.dead
	}

	pub fn get_velocity(&self) -> Vec2{
		self.velocity
	}

	pub fn hurt(&mut self){
		if self.life == 2{ //blue enemy
			self.velocity = Vec2::new(0.0,1.0);
			self.position.y -= 10.0;
		}
		if self.life >= 1{
			self.life -=1;
		}
		if self.life == 0{
			self.dead = true;
			self.velocity = Vec2::new(0.0,0.0);
		}
	}

	pub fn set_velocity(&mut self, velocity: Vec2){
		self.velocity = velocity;
	}
}