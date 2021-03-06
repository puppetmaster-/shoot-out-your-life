use tetra::Context;
use tetra::glm::{self, Vec2};
use tetra::graphics::{self, DrawParams};
use tetra::input::{self, Key, GamepadButton};
use crate::{GAMEINFO};
use crate::scenes::manager::{Scene, Transition};
use crate::bullet::{self, Bullet};
use crate::enemy::Enemy;
use tetra::graphics::Color;
use rand::prelude::*;
use crate::assets::{Assets, SoundName, TextureName, AnimationName, TextName};
use crate::particle::Particle;

const SEED:[u8;32] = [12; 32];
const TIMEOUT_MAX: i32 = 8;
const MAX_BOTTOM: f32 = 412.0;

pub struct GameScene {
	state: State,
	tick: i32,
	tick_max: i32,
	life: i32,
	level: i32,
	force: i32,
	score: i32,
	count_spawn: i32,
	spawn_rate: i32,
	assets: Assets,
	player_position: Vec2,
	bullets: Vec<Bullet>,
	enemies: Vec<Enemy>,
	particles: Vec<Particle>,
	randomizer: ThreadRng,
	spawn_rnd: StdRng,
	input_timeout: i32,
}

impl GameScene{
	pub fn new(ctx: &mut Context) -> tetra::Result<GameScene> {
		Ok(GameScene {
			state: State::Normal,
			tick: 0,
			tick_max: 10,
			life: 10,
			force: 0,
			score: 0,
			level: 1,
			count_spawn: 0,
			spawn_rate: 0,
			player_position: Vec2::new((GAMEINFO.window.width/2) as f32,(GAMEINFO.window.height - 56) as f32),
			bullets: vec![],
			enemies: vec![],
			particles: vec![],
			assets: Assets::new(ctx)?,
			randomizer:  rand::thread_rng(),
			spawn_rnd: SeedableRng::from_seed(SEED),
			input_timeout: 0,
		})
	}

	fn reset(&mut self){
		self.enemies.clear();
		self.bullets.clear();
		self.score = 0;
		self.state = State::Normal;
		self.life = 10;
		self.force = 0;
		self.level = 1;
		self.count_spawn = 0;
		self.player_position = Vec2::new(GAMEINFO.window.get_half().x,(GAMEINFO.window.height - 76) as f32);
		self.spawn_rnd = SeedableRng::from_seed(SEED);
	}

	fn shoot(&mut self,ctx: &mut Context, force: i32, spawn_position: Vec2){
		let kind = glm::clamp_scalar(force, 1, 4);
		let mut b = Bullet::new(spawn_position+Vec2::new(0.0, 10.0), force, kind); //y=10 can shoot enemy at the bottom line
		b.set_velocity(bullet::get_velocity_from_force(kind));
		self.bullets.push(b);
		if kind == 1{
			self.assets.get_sound(&SoundName::ShootSlow).play_with(ctx, 0.1, self.randomizer.gen_range(0.8, 1.1)).ok();
			input::start_gamepad_vibration(ctx, 0, 0.05, 200);
		}else{
			self.assets.get_sound(&SoundName::ShootSlow).play_with(ctx, 0.1, self.randomizer.gen_range(0.8, 1.1)).ok();
			input::start_gamepad_vibration(ctx, 0, 0.1, 280);
		}
	}

	fn check_collision(&mut self, ctx: &mut Context){
		for enemy in self.enemies.iter_mut(){
			if !enemy.is_dead() {
				let x = enemy.get_position().x as i32;
				let y = enemy.get_position().y as i32;
				let w = enemy.get_area().0;
				let h = enemy.get_area().1;
				for bullet in self.bullets.iter_mut() {
					if !bullet.is_broken() {
						let bx = bullet.get_position().x as i32;
						let by = bullet.get_position().y as i32;
						let bw = bullet.get_area().0;
						let bh = bullet.get_area().1;
						if w > 0 && h > 0 && bw > 0 && bh > 0 && x < bx + bw && x + w > bx && y < by + bh && y + h > by {
							// score logic
							let points = 50;
							let bonus = ((480.0 - enemy.get_position().y)/5.0) as i32 * glm::clamp_scalar(bullet.get_force(), 1, 4);
							self.score += points + bonus;
							self.assets.get_text_mut(TextName::ScoreGui).set_content(format!("{}",self.score));
							for _i in 0..bullet.get_force(){
								if enemy.get_life() > 0{
									enemy.hurt();
									bullet.consume_force();
									self.life +=1;
								}
							}
							self.assets.get_sound(&SoundName::Hurt).play_with(ctx, 0.1, self.randomizer.gen_range(0.9, 1.1)).ok();
						}
					}
				}
			}
		}
	}

	fn check_player_get_hurt(&mut self, ctx: &mut Context){
		for enemy in self.enemies.iter_mut(){
			if enemy.get_position().y >= MAX_BOTTOM && !enemy.is_dead(){
				self.life -= 1;
				enemy.hurt();
				self.assets.get_sound(&SoundName::Hurt2).play_with(ctx, 0.2, self.randomizer.gen_range(0.8, 1.1)).ok();
				input::start_gamepad_vibration(ctx, 0, 0.3, 200);
			}
		}
	}

	fn check_player_get_life(&mut self, ctx: &mut Context){
		for bullet in self.bullets.iter_mut(){
			if bullet.get_position().y >= MAX_BOTTOM && !bullet.is_broken() && bullet.is_returning(){
				self.life += bullet.get_force();
				bullet.set_broken();
				self.assets.get_sound(&SoundName::Pickup).play_with(ctx, 0.08, self.randomizer.gen_range(0.9, 1.0)).ok();
			}
		}
	}

	fn spawn_enemy(&mut self, ctx: &mut Context){
		// setup spawn logic
		let mut positions = vec![40.0,60.0,80.0,100.0,120.0,140.0,160.0,180.0,200.0];
		let mut vec_velocity = 0;
		let mut max_enemy = self.level;
		if self.level <= 2{
			positions = vec![80.0,100.0,120.0,140.0,160.0];
		}else if self.level <= 4{
			vec_velocity = self.spawn_rnd.gen_range(0,2);
			positions = vec![60.0,80.0,100.0,140.0,160.0,180.0,200.0];
		}else if self.level == 5{
			max_enemy = 4;
			positions = vec![40.0,60.0,80.0,100.0,140.0,160.0,180.0,200.0];
		} else if self.level == 6{
			if self.count_spawn % 5 == 0 {
				vec_velocity = self.spawn_rnd.gen_range(1, 3);
			}
			positions = vec![80.0,100.0,120.0,160.0,180.0];
		} else if self.level == 7{
			max_enemy = 5;
			vec_velocity = self.spawn_rnd.gen_range(0,2);
			positions = vec![80.0,100.0,120.0,140.0,160.0];
		}else if self.level == 8{
			max_enemy = 6;
			positions = vec![40.0,80.0,120.0,160.0,200.0];
		}else{
			if self.count_spawn % 10 == 0{
				vec_velocity = self.spawn_rnd.gen_range(1,3);
			}else if self.count_spawn % 4 == 0 {
				vec_velocity = self.spawn_rnd.gen_range(0,2);
			}
		}

		// enemy spawn
		let max_len = positions.len();
		let x = positions[self.spawn_rnd.gen_range(0,max_len)];
		if self.enemies.len() < max_enemy as usize{
			let mut enemy = Enemy::new(Vec2::new(x,-20.0));
			enemy.set_velocity(Vec2::new(0.0,enemy.get_velocity().y + vec_velocity as f32));
			if vec_velocity == 2{
				enemy.set_life(2);
			}
			self.enemies.push(enemy);
			self.count_spawn +=1
		}

		// next level logic
		if self.count_spawn >=10 * glm::clamp_scalar(self.level, 1, 8){
			self.level +=1;
			self.count_spawn = 0;
			self.assets.get_sound(&SoundName::NewLevel).play_with(ctx, 0.1, 1.0).ok();
		}
	}

	fn add_bullet_particle(&mut self, position: Vec2){
		self.particles.push(Particle::new(position,Vec2::new(0.0,0.0))
			.set_aging(0.1)
			.set_texture_name(TextureName::Particle0)
			.set_offset(Vec2::new(self.randomizer.gen_range(2.0,4.0),0.0))
		)
	}
	fn add_enemy_particle(&mut self, position: Vec2){
		self.particles.push(Particle::new(position,Vec2::new(0.0,0.0))
			.set_aging(0.07)
			.set_texture_name(TextureName::Particle1)
			.set_offset(Vec2::new(self.randomizer.gen_range(2.0,6.0),-3.0))
		)
	}
}

impl Scene for GameScene {
	fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
		self.assets.update();

		//GAME
		if self.state != State::Dead && self.state != State::Pause {

			// dont't spawn on same place
			self.spawn_rate += 1;
			if self.spawn_rate > 20{
				self.spawn_enemy(ctx);
				self.spawn_rate = 0;
			}

			//move
			if (input::is_key_pressed(ctx, Key::Right) || input::is_gamepad_button_pressed(ctx, 0, GamepadButton::Right)) && self.player_position.x <= 180.0 {
				self.input_timeout = TIMEOUT_MAX;
				self.player_position.x += 20.0;
			}else if input::is_key_pressed(ctx, Key::Up) || input::is_gamepad_button_pressed(ctx, 0, GamepadButton::RightShoulder){
				self.input_timeout = TIMEOUT_MAX;
				self.player_position.x = 200.0;
			}else if input::is_key_pressed(ctx, Key::Down) || input::is_gamepad_button_pressed(ctx, 0, GamepadButton::LeftShoulder){
				self.input_timeout = TIMEOUT_MAX;
				self.player_position.x = 40.0;
			}else if (input::is_key_pressed(ctx, Key::Left) || input::is_gamepad_button_pressed(ctx, 0, GamepadButton::Left)) && self.player_position.x >= 60.0 {
				self.input_timeout = TIMEOUT_MAX;
				self.player_position.x -= 20.0;
			}else if input::is_key_down(ctx, Key::Right)||input::is_gamepad_button_down(ctx, 0, GamepadButton::Right){
				self.input_timeout -= 1;
				if self.input_timeout <= 0 {
					self.input_timeout = TIMEOUT_MAX;
					if self.player_position.x <= 180.0 {
						self.player_position.x += 20.0;
					}
				}
			}else if input::is_key_down(ctx, Key::Left) ||input::is_gamepad_button_down(ctx, 0, GamepadButton::Left){
				self.input_timeout -= 1;
				if self.input_timeout <= 0 {
					self.input_timeout = TIMEOUT_MAX;
					if self.player_position.x >= 60.0 {
						self.player_position.x -= 20.0;
					}
				}
			}


			//shoot
			if (input::is_key_down(ctx, Key::Space) ||
				input::is_gamepad_button_down(ctx, 0, GamepadButton::A)) && self.life > 0 {
				if self.state == State::Normal{
					self.state = State::Pressed;
					self.force += 1;
					self.life -= 1;
					self.assets.get_sound(&SoundName::Charge).play_with(ctx, 0.1, self.randomizer.gen_range(0.9, 1.1)).ok();
				} else if self.state == State::Pressed {
					self.tick += 1;
					if self.tick > self.tick_max {
						self.force += 1;
						self.life -= 1;
						self.tick = 0;
						let pitch = self.force as f32 * 0.04;
						self.assets.get_sound(&SoundName::Charge).play_with(ctx, 0.1, self.randomizer.gen_range(0.9 + pitch, 1.1 + pitch)).ok();
					}
				}
			}else if (input::is_key_released(ctx, Key::Space) ||
				input::is_gamepad_button_released(ctx, 0, GamepadButton::A)) && self.force > 0 {
				self.state = State::Normal;
				self.shoot(ctx, self.force, self.player_position);
				self.force = 0;
				self.tick = 0;
			}

			// check for player dead
			if self.life < 0 {
				self.state = State::Dead;
				self.assets.get_text_mut(TextName::Score).set_content(format!("{}",self.score));
				input::start_gamepad_vibration(ctx, 0, 0.4, 500);
			}

			//update bullets
			self.bullets.retain(|b|!b.is_broken());
			for bullet in self.bullets.iter_mut() {
				bullet.update();
			}
			let bullet_positions: Vec<Vec2> = self.bullets.iter().map(|bullet|bullet.get_position()).collect();
			/*
			for i in 0..bullet.get_force() {
				bullet_positions.push(bullet.get_position() - Vec2::new(0.0, -(i as f32 * 2.0)));
			}
			*/


			//update enemy
			self.enemies.retain(|e| !e.is_dead());
			for enemy in self.enemies.iter_mut(){
				enemy.update();
			}
			let enemy_positions: Vec<_> = self.enemies.iter().map(|e|e.get_position()).collect();

			//update and create particles
			for pos in bullet_positions{
				self.add_bullet_particle(pos);
			}
			for pos in enemy_positions{
				self.add_enemy_particle(pos);
			}
			self.particles.retain(|p| !p.is_dead());
			for particle in self.particles.iter_mut(){
				particle.update();
			}

			// check collision, player hurt and player get life back
			self.check_collision(ctx);
			self.check_player_get_hurt(ctx);
			self.check_player_get_life(ctx);

			self.assets.get_text_mut(TextName::Life).set_content(format!("{}", self.life));
			self.assets.get_text_mut(TextName::Level).set_content(format!("{}", self.level));

		}

		// pause
		if self.state == State::Normal &&
			(input::is_key_released(ctx, Key::Escape) || input::is_gamepad_button_released(ctx,0,GamepadButton::Start)){
			self.state = State::Pause;
		}else if self.state == State::Pause &&
			(input::is_key_released(ctx, Key::Escape) || input::is_gamepad_button_released(ctx,0,GamepadButton::Start)){
			self.state = State::Normal;
		}

		// reset game after dead
		if self.state == State::Dead && (input::is_key_released(ctx, Key::Return) ||
			input::is_gamepad_button_released(ctx,0,GamepadButton::Start)){
			self.reset();
		}

		if input::is_key_released(ctx, Key::Backspace) ||
			input::is_gamepad_button_released(ctx,0,GamepadButton::Back){
			Ok(Transition::Pop)
		}else{
			Ok(Transition::None)
		}
	}

	fn draw(&mut self, ctx: &mut Context, _dt: f64) -> tetra::Result<Transition> {
		graphics::clear(ctx, Color::rgb(0.122, 0.055, 0.11));
		graphics::draw(ctx,self.assets.get_texture(&TextureName::Background),
					   Vec2::new(0.0, 0.0));
		graphics::draw(ctx,self.assets.get_animation(&AnimationName::Line),self.player_position-Vec2::new(3.0,379.0));

		//draw player
		graphics::draw(ctx,self.assets.get_animation(&AnimationName::Player),
					   self.player_position-Vec2::new(15.0,0.0));

		//draw particles
		for particle in self.particles.iter() {
			graphics::draw(ctx, self.assets.get_texture(particle.get_texture_name()), DrawParams::new()
				.position(particle.get_position()-particle.get_offset())
				.color(particle.get_color())
			);
		}

		//draw bullets
		for bullet in self.bullets.iter(){
			let name = if bullet.is_returning() {
				if bullet.get_kind() == 1{
					TextureName::Bullet1Down
				}else if bullet.get_kind() == 2{
					TextureName::Bullet2Down
				}else{
					TextureName::Bullet3Down
				}
			}else if bullet.get_kind() == 1{
				TextureName::Bullet1Up
			}else if bullet.get_kind() == 2{
				TextureName::Bullet2Up
			}else{
				TextureName::Bullet3Up
			};
			graphics::draw(ctx, self.assets.get_texture(&name),bullet.get_position()-Vec2::new(8.0,0.0))
		}

		//draw enemy
		for enemy in self.enemies.iter(){
			if enemy.get_velocity().y as i32 == 1{
				graphics::draw(ctx,
							   self.assets.get_animation(&AnimationName::Enemy1),
							   enemy.get_position()-Vec2::new(10.0,0.0));
			}else if enemy.get_velocity().y as i32 == 2{
				graphics::draw(ctx,
							   self.assets.get_animation(&AnimationName::Enemy2),
							   enemy.get_position()-Vec2::new(10.0,0.0));
			}else{
				graphics::draw(ctx,
							   self.assets.get_animation(&AnimationName::Enemy3),
							   enemy.get_position()-Vec2::new(10.0,0.0));
			}
		}

		//draw gui
		graphics::draw(ctx, self.assets.get_texture(&TextureName::Life), Vec2::new(10.0, 13.0)); //436
		graphics::draw(ctx, self.assets.get_text(&TextName::Life), Vec2::new(32.0, 10.0)); //433
		graphics::draw(ctx, self.assets.get_texture(&TextureName::Level), Vec2::new(180.0, 436.0)); //180,436
		graphics::draw(ctx, self.assets.get_text(&TextName::Level), Vec2::new(200.0, 433.0)); //200,433
		//score
		let bound_score = self.assets.get_text(&TextName::ScoreGui).get_bounds(ctx).unwrap();
		graphics::draw(ctx, self.assets.get_text(&TextName::ScoreGui), Vec2::new(230.0-bound_score.width, 12.0));
		if self.state == State::Dead {
			let bound = self.assets.get_text(&TextName::GameOver).get_bounds(ctx).unwrap();
			let bound2 = self.assets.get_text(&TextName::Score).get_bounds(ctx).unwrap();
			graphics::draw(ctx, self.assets.get_text(&TextName::GameOver),
						   Vec2::new(GAMEINFO.window.get_half().x - bound.width / 2.0,
									 GAMEINFO.window.get_half().y));
			graphics::draw(ctx, self.assets.get_text(&TextName::Score),
						   Vec2::new(GAMEINFO.window.get_half().x - bound2.width / 2.0,
									 GAMEINFO.window.get_half().y + 30.0));
		}else if self.state == State::Pause{
			let bound = self.assets.get_text(&TextName::Pause).get_bounds(ctx).unwrap();
			graphics::draw(ctx, self.assets.get_text(&TextName::Pause),
						   Vec2::new(GAMEINFO.window.get_half().x - bound.width / 2.0,
									 GAMEINFO.window.get_half().y));
		}
		Ok(Transition::None)
	}

}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum State {
	Normal,
	Pressed,
	Dead,
	Pause,
}
