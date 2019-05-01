use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::{Context, input, audio};

use crate::scenes::game::GameScene;
use crate::scenes::manager::{Scene, Transition};
use tetra::input::{Key, GamepadButton};
use tetra::audio::{SoundInstance, Sound};

#[allow(dead_code)]
pub struct TitleScene {
	titel: Texture,
	background_music_instance: SoundInstance,
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

		Ok(TitleScene {
			titel,
			background_music_instance,
		})
	}
}

impl Scene for TitleScene {
	fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {

		let gamepad_connected = input::is_gamepad_connected(ctx, 0);

		if input::is_key_released(ctx, Key::Space) || input::is_key_released(ctx, Key::Return) || gamepad_connected && input::is_gamepad_button_released(ctx,0,GamepadButton::A) || gamepad_connected && input::is_gamepad_button_released(ctx,0,GamepadButton::Start){
			Ok(Transition::Push(Box::new(GameScene::new(ctx)?)))
		}else{
			Ok(Transition::None)
		}

	}

	fn draw(&mut self, ctx: &mut Context, _dt: f64) -> tetra::Result<Transition> {
		graphics::clear(ctx, Color::rgb(0.122, 0.055, 0.11));
		graphics::draw(ctx, &self.titel, DrawParams::default());

		Ok(Transition::None)
	}
}