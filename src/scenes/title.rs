use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::{Context, input, audio};
use tetra::glm::Vec2;
use tetra::input::{Key, GamepadButton};
use tetra::audio::{SoundInstance, Sound};

use crate::scenes::game::GameScene;
use crate::scenes::manager::{Scene, Transition};
use crate::particle::{Particle};
use crate::GAMEINFO;

use rand::prelude::*;

#[allow(dead_code)]
pub struct TitleScene {
	titel: Texture,
	background_music_instance: SoundInstance,
	particles: Vec<Particle>,
	spawn_rate: i32,
	gfx: Vec<Texture>,
	randomizer: ThreadRng,
}

impl TitleScene {
	pub fn new(ctx: &mut Context) -> tetra::Result<TitleScene> {
		let titel = Texture::from_file_data(ctx,include_bytes!("../../assets/art/titel.png"))?;
		audio::set_master_volume(ctx, 2.0);
		let background_music = Sound::from_file_data(include_bytes!("../../assets/music/014.mp3"));
		let background_music_instance = background_music.spawn(ctx)?;
		background_music_instance.set_repeating(true);
		background_music_instance.play();
		background_music_instance.set_volume(0.1);
		let randomizer = rand::thread_rng();

		Ok(TitleScene {
			titel,
			background_music_instance,
			particles: vec![],
			spawn_rate: 0,
			gfx: build_gfx(ctx)?,
			randomizer,
		})
	}

	fn spawn_particles(&mut self){
		if self.particles.len() < 100{
			let x = self.randomizer.gen_range(-5.0, GAMEINFO.window.width as f32);
			let speed_y = self.randomizer.gen_range(1.0,1.5);
			let particle = Particle::new(Vec2::new(x,-20.0),Vec2::new(0.0,speed_y))
				.set_aging(self.randomizer.gen_range(0.002,0.004))
				.set_kind(self.randomizer.gen_range(0,3));
			self.particles.push(particle);
		}
	}
}

impl Scene for TitleScene {
	fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
		self.particles.retain(|p| !p.is_dead());
		//self.particles.sort_by(|a, b| b.get_age().partial_cmp(&a.get_age()).unwrap()); //not working
		for particle in self.particles.iter_mut(){
			particle.update();
		}

		self.spawn_rate += 1;
		if self.spawn_rate > 5{
			self.spawn_particles();
			self.spawn_rate = 0;
		}


		if input::is_key_released(ctx, Key::Space) || input::is_key_released(ctx, Key::Return) ||
			input::is_gamepad_button_released(ctx,0,GamepadButton::A) ||
			input::is_gamepad_button_released(ctx,0,GamepadButton::Start){
			Ok(Transition::Push(Box::new(GameScene::new(ctx)?)))
		}else if input::is_key_released(ctx, Key::Escape) || input::is_key_released(ctx, Key::Backspace) ||
				input::is_gamepad_button_released(ctx,0,GamepadButton::Back) {
			Ok(Transition::Quit)
		}else{
			Ok(Transition::None)
		}
	}

	fn draw(&mut self, ctx: &mut Context, _dt: f64) -> tetra::Result<Transition> {
		graphics::clear(ctx, Color::rgb(0.122, 0.055, 0.11));
		for particle in self.particles.iter() {
			graphics::draw(ctx, &self.gfx[particle.get_kind() as usize], DrawParams::new()
				.position(particle.get_position())
				.color(particle.get_color())
			);
		}
		graphics::draw(ctx, &self.titel, DrawParams::default());

		Ok(Transition::None)

	}

}

fn build_gfx(ctx: &mut Context) -> tetra::Result<Vec<Texture>>{
	let gfx = vec![
		Texture::from_file_data(ctx,include_bytes!("../../assets/art/particle_00.png"))?,
		Texture::from_file_data(ctx,include_bytes!("../../assets/art/particle_01.png"))?,
		Texture::from_file_data(ctx,include_bytes!("../../assets/art/particle_02.png"))?,
	];
	Ok(gfx)
}
