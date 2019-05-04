use tetra::Context;
use tetra::glm::{self, Vec2};
use tetra::audio::{self, Sound, SoundInstance};
use tetra::graphics::{self, Text, Font, Texture, Animation, Rectangle};
use tetra::input::{self, Key, GamepadButton};
use crate::GAMEINFO;
use crate::scenes::manager::{Scene, Transition};
use crate::bullet::Bullet;
use crate::enemy::Enemy;
use tetra::graphics::Color;
use rand::{Rng};

pub struct GameScene {
	state: State,
	tick: i32,
	tick_max: i32,
	life: i32,
	life_text: Text,
	life_art: Texture,
	level: i32,
	level_text: Text,
	level_art: Texture,
	force: i32,
	player_art: Animation,
	player_position: Vec2,
	bullets: Vec<Bullet>,
	enemy_art: Animation,
	enemy_art2: Animation,
	enemy_art3: Animation,
	enemys: Vec<Enemy>,
	bullet_art_down: Texture,
	bullet_art_up: Texture,
	gameover_text: Text,
	background: Texture,
	snd_shoot_instance: SoundInstance,
	snd_shoot2_instance: SoundInstance,
	snd_hurt_instance: SoundInstance,
	snd_hurt2_instance: SoundInstance,
	snd_pickup_instance: SoundInstance,
	snd_newlevel_instance: SoundInstance,
	score: i32,
	score_text: Text,
	count_spawn: i32,
	spawn_rate: i32,
}

impl GameScene{
	pub fn new(ctx: &mut Context) -> tetra::Result<GameScene> {
		let tileset = Texture::from_file_data(ctx, include_bytes!("../../assets/art/shootOutYourLife.png"))?;

		let life = 10;
		let life_text = Text::new(format!("{}",life), Font::default(), 24.0);
		let life_art = Texture::from_file_data(ctx,include_bytes!("../../assets/art/life_art.png"))?;

		let level = 1;
		let level_text = Text::new(format!("{}",level), Font::default(), 24.0);
		let level_art = Texture::from_file_data(ctx,include_bytes!("../../assets/art/level_art.png"))?;

		let score_text = Text::new("", Font::default(), 18.0);
		let player_position = Vec2::new((GAMEINFO.window.width/2) as f32,(GAMEINFO.window.height - 76) as f32);
		let player_art = Animation::new(tileset.clone(),Rectangle::row(0.0, 0.0, 32.0, 32.0).take(4).collect(), 10);
		let bullet_art_down = Texture::from_file_data(ctx,include_bytes!("../../assets/art/shoot_0.png"))?;
		let bullet_art_up = Texture::from_file_data(ctx,include_bytes!("../../assets/art/shoot_1.png"))?;
		let enemy_art = Animation::new(tileset.clone(),Rectangle::row(128.0, 0.0, 32.0, 32.0).take(4).collect(), 10);
		let enemy_art2 = Animation::new(tileset.clone(),Rectangle::row(128.0, 32.0, 32.0, 32.0).take(4).collect(), 10);
		let enemy_art3 = Animation::new(tileset.clone(),Rectangle::row(0.0, 32.0, 32.0, 32.0).take(4).collect(), 10);
		let background = Texture::from_file_data(ctx,include_bytes!("../../assets/art/background.png"))?;
		let gameover_text = Text::new("GAME OVER", Font::default(), 32.0);

		audio::set_master_volume(ctx, 2.0);
		let snd_shoot = Sound::from_file_data(include_bytes!("../../assets/sound/shoot3.wav"));
		let snd_shoot_instance  = snd_shoot.spawn(ctx)?;
		snd_shoot_instance.set_volume(0.1);
		let snd_shoot2 = Sound::from_file_data(include_bytes!("../../assets/sound/shoot1.wav"));
		let snd_shoot2_instance  = snd_shoot2.spawn(ctx)?;
		snd_shoot2_instance.set_volume(0.1);
		let snd_hurt = Sound::from_file_data(include_bytes!("../../assets/sound/ouch.wav"));
		let snd_hurt_instance  = snd_hurt.spawn(ctx)?;
		snd_hurt_instance.set_volume(0.1);
		let snd_hurt2 = Sound::from_file_data(include_bytes!("../../assets/sound/ouch5.wav"));
		let snd_hurt2_instance  = snd_hurt2.spawn(ctx)?;
		snd_hurt2_instance.set_volume(0.2);
		let snd_pickup = Sound::from_file_data(include_bytes!("../../assets/sound/pickup.wav"));
		let snd_pickup_instance  = snd_pickup.spawn(ctx)?;
		snd_pickup_instance.set_volume(0.1);
		let snd_newlevel = Sound::from_file_data(include_bytes!("../../assets/sound/level.wav"));
		let snd_newlevel_instance  = snd_newlevel.spawn(ctx)?;
		snd_newlevel_instance.set_volume(0.1);


		Ok(GameScene {
			state: State::Normal,
			tick: 0,
			tick_max: 10,
			life,
			life_art,
			life_text,
			force: 0,
			player_art,
			player_position,
			bullets: vec![],
			enemy_art,
			enemy_art2,
			enemy_art3,
			enemys: vec![],
			bullet_art_up,
			bullet_art_down,
			gameover_text,
			background,
			snd_shoot_instance,
			snd_shoot2_instance,
			snd_hurt_instance,
			snd_hurt2_instance,
			snd_pickup_instance,
			score: 0,
			score_text,
			level,
			level_art,
			level_text,
			count_spawn: 0,
			spawn_rate: 0,
			snd_newlevel_instance,
		})
	}

	fn reset(&mut self){
		self.enemys.clear();
		self.bullets.clear();
		self.score = 0;
		self.state = State::Normal;
		self.life = 10;
		self.force = 0;
		self.level = 1;
		self.count_spawn = 0;
		self.player_position = Vec2::new(GAMEINFO.window.get_half().x,(GAMEINFO.window.height - 76) as f32);
	}

	fn shoot(&mut self, force: i32){
		let mut bullet = Bullet::new(self.player_position, force);
		bullet.set_velocity(bullet.get_velocity() * glm::clamp_scalar(force, 1, 4) as f32);
		self.bullets.push(bullet);
		if force == 1{
			self.snd_shoot_instance.play();
		}else{
			self.snd_shoot2_instance.play();
		}
	}

	fn check_collision(&mut self){
		for enemy in self.enemys.iter_mut(){
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
							self.score += ((480.0 - enemy.get_position().y)/5.0) as i32 * bullet.get_force();

							enemy.hurt();
							bullet.consume_force();
							self.life +=1;
							self.snd_hurt_instance.play();
						}
					}
				}
			}
		}
	}

	fn check_player_get_hurt(&mut self){
		for enemy in self.enemys.iter_mut(){
			if enemy.get_position().y >= 392.0 && !enemy.is_dead(){
				self.life -= 1;
				enemy.hurt();
				self.snd_hurt2_instance.play();
			}
		}
	}

	fn check_player_get_life(&mut self){
		for bullet in self.bullets.iter_mut(){
			if bullet.get_position().y >= 392.0 && !bullet.is_broken() && bullet.is_returning(){
				self.life += bullet.get_force();
				bullet.set_broken();
				self.snd_pickup_instance.play();
			}
		}
	}

	fn spawn_enemy(&mut self){
		// setup spawn logic
		let mut rng = rand::thread_rng();
		let mut positions = vec![40.0,60.0,80.0,100.0,120.0,140.0,160.0,180.0,200.0];
		let mut vec_velocity = 0;
		let mut max_enemy = self.level;
		if self.level <= 2{
			positions = vec![80.0,100.0,120.0,140.0,160.0];
		}else if self.level <= 4{
			vec_velocity = rng.gen_range(0,2);
			positions = vec![60.0,80.0,100.0,140.0,160.0,180.0,200.0];
		}else if self.level == 5{
			max_enemy = 4;
			positions = vec![40.0,60.0,80.0,100.0,140.0,160.0,180.0,200.0];
		} else if self.level == 6{
			if self.count_spawn % 5 == 0 {
				vec_velocity = rng.gen_range(1, 3);
			}
			positions = vec![80.0,100.0,120.0,160.0,180.0];
		} else if self.level == 7{
			max_enemy = 5;
			vec_velocity = rng.gen_range(0,2);
			positions = vec![80.0,100.0,120.0,140.0,160.0];
		}else if self.level == 8{
			max_enemy = 6;
			positions = vec![40.0,80.0,120.0,160.0,200.0];
		}else{
			if self.count_spawn % 10 == 0{
				vec_velocity = rng.gen_range(1,3);
			}else if self.count_spawn % 4 == 0 {
				vec_velocity = rng.gen_range(0,2);
			}
		}

		// enemy spawn
		let max_len = positions.len();
		let x = positions[rng.gen_range(0,max_len)];
		if self.enemys.len() < max_enemy as usize{
			let mut enemy = Enemy::new(Vec2::new(x,-20.0));
			enemy.set_velocity(Vec2::new(0.0,enemy.get_velocity().y + vec_velocity as f32));
			if vec_velocity == 2{
				enemy.set_life(2);
			}
			self.enemys.push(enemy);
			self.count_spawn +=1
		}

		// next level logic
		if self.count_spawn >=10*self.level{
			self.level +=1;
			self.count_spawn = 0;
			self.snd_newlevel_instance.play();
		}
	}
}

impl Scene for GameScene {
	fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {

		// play animation
		self.player_art.tick();
		self.enemy_art.tick();
		self.enemy_art2.tick();
		self.enemy_art3.tick();

		let gamepad_connected = input::is_gamepad_connected(ctx, 0);

		//GAME
		if self.state != State::Dead {

			// dont't spawn on same place
			self.spawn_rate += 1;
			if self.spawn_rate > 20{
				self.spawn_enemy();
				self.spawn_rate = 0;
			}

			//move
			if (input::is_key_pressed(ctx, Key::Right) || gamepad_connected && input::is_gamepad_button_pressed(ctx, 0, GamepadButton::Right)) && self.player_position.x <= 180.0 {
				self.player_position.x += 20.0
			}
			if (input::is_key_pressed(ctx, Key::Left) || gamepad_connected && input::is_gamepad_button_pressed(ctx, 0, GamepadButton::Left)) && self.player_position.x >= 60.0{
				self.player_position.x -= 20.0
			}

			//shoot
			if (input::is_key_down(ctx, Key::Space) || gamepad_connected && input::is_gamepad_button_down(ctx,0,GamepadButton::A)) && self.life > 0 {
				if self.state == State::Normal{
					self.state = State::Pressed;
					self.force += 1;
					self.life -= 1;
				} else if self.state == State::Pressed {
					self.tick += 1;
					if self.tick > self.tick_max {
						self.force += 1;
						self.life -= 1;
						self.tick = 0;
					}
				}
			}

			if (input::is_key_released(ctx, Key::Space) || gamepad_connected && input::is_gamepad_button_released(ctx,0,GamepadButton::A)) && self.force > 0 {
				self.state = State::Normal;
				self.shoot(self.force);
				self.force = 0;
				self.tick = 0;
			}

			// check for player dead
			if self.life < 0 {
				self.state = State::Dead;
				self.score_text.set_content(format!("SCORE: {}",self.score));
			}

			//update bullets
			self.bullets.retain(|b| !b.is_broken());
			for bullet in self.bullets.iter_mut(){
				bullet.update();
			}

			//update enemy
			self.enemys.retain(|b| !b.is_dead());
			for enemy in self.enemys.iter_mut(){
				enemy.update();
			}

			// check collision, player hurt and player get life back
			self.check_collision();
			self.check_player_get_hurt();
			self.check_player_get_life();

			self.life_text.set_content(format!("{}", self.life));
			self.level_text.set_content(format!("{}", self.level));

		}

		// reset game after dead
		if self.state == State::Dead && (input::is_key_released(ctx, Key::Return) || gamepad_connected && input::is_gamepad_button_released(ctx,0,GamepadButton::Start)){
			self.reset();
		}

		Ok(Transition::None)
	}

	fn draw(&mut self, ctx: &mut Context, _dt: f64) -> tetra::Result<Transition> {
		graphics::clear(ctx, Color::rgb(0.122, 0.055, 0.11));
		graphics::draw(ctx,&self.background,Vec2::new(0.0, 0.0));
		graphics::draw(ctx,&self.player_art,self.player_position-Vec2::new(15.0,0.0));

		//draw bullets
		for bullet in self.bullets.iter(){
			if bullet.is_returning(){
				graphics::draw(ctx,&self.bullet_art_down,bullet.get_position()-Vec2::new(8.0,0.0))
			}else{
				graphics::draw(ctx,&self.bullet_art_up,bullet.get_position()-Vec2::new(8.0,0.0))
			}
		}

		//draw enemy
		for enemy in self.enemys.iter(){
			if enemy.get_velocity().y == 1.0{
				graphics::draw(ctx, &self.enemy_art,enemy.get_position()-Vec2::new(10.0,0.0));
			}else if enemy.get_velocity().y == 2.0{
				graphics::draw(ctx, &self.enemy_art2,enemy.get_position()-Vec2::new(10.0,0.0));
			}else{
				graphics::draw(ctx, &self.enemy_art3,enemy.get_position()-Vec2::new(10.0,0.0));
			}
		}

		//draw gui
		graphics::draw(ctx, &self.life_art, Vec2::new(10.0, 436.0));
		graphics::draw(ctx, &self.life_text, Vec2::new(32.0, 433.0));
		graphics::draw(ctx, &self.level_art, Vec2::new(180.0, 436.0));
		graphics::draw(ctx, &self.level_text, Vec2::new(200.0, 433.0));
		if self.state == State::Dead {
			let bound = self.gameover_text.get_bounds(ctx).unwrap();
			let bound2 = self.score_text.get_bounds(ctx).unwrap();
			graphics::draw(ctx, &self.gameover_text, Vec2::new(GAMEINFO.window.get_half().x - bound.width / 2.0, GAMEINFO.window.get_half().y));
			graphics::draw(ctx, &self.score_text, Vec2::new(GAMEINFO.window.get_half().x - bound2.width / 2.0, GAMEINFO.window.get_half().y + 30.0));
		}
		Ok(Transition::None)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum State {
	Normal,
	Pressed,
	Dead,
}