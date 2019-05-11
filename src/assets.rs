use tetra::Context;
use tetra::audio::Sound;
use tetra::graphics::{Animation,Texture, Text, Rectangle, Font};
use std::collections::HashMap;

pub struct Assets{
	sounds: HashMap<SoundName, Sound>,
	animations: HashMap<AnimationName, Animation>,
	textures: HashMap<TextureName, Texture>,
	texts: Vec<Text>,
}

impl Assets{
	pub fn new(ctx: &mut Context) -> tetra::Result<Self>{
		Ok(Assets{
			sounds: build_sounds(),
			animations: build_animations(ctx)?,
			textures: build_textures(ctx)?,
			texts: build_texts(ctx),
		})
	}

	pub fn update(&mut self){
		for animation in self.animations.values_mut(){
			animation.tick();
		}
	}

	pub fn get_animation(&self, name: &AnimationName) -> &Animation{
		&self.animations[&name]
	}

	pub fn get_texture(&self, name: &TextureName) -> &Texture{
		&self.textures[&name]
	}

	pub fn get_sound(&self, name: &SoundName) -> &Sound{
		&self.sounds[&name]
	}

	pub fn get_text(&self, name: &TextName) -> &Text{
		//cheap solution
		match name{
			TextName::Life => &self.texts[0],
			TextName::Level => &self.texts[1],
			TextName::Score => &self.texts[2],
			TextName::GameOver => &self.texts[3],
		}

	}

	pub fn get_text_mut(&mut self, name: TextName) -> &mut Text{
		//cheap solution
		match name{
			TextName::Life => &mut self.texts[0],
			TextName::Level => &mut self.texts[1],
			TextName::Score => &mut self.texts[2],
			TextName::GameOver => &mut self.texts[3],
		}
	}

}

fn build_animations(ctx: &mut Context) ->tetra::Result<HashMap<AnimationName, Animation>>{
	let tileset = Texture::from_file_data(ctx, include_bytes!("../assets/art/shootOutYourLife.png"))?;
	let animations: HashMap<AnimationName, Animation> = [
		(AnimationName::Player, Animation::new(tileset.clone(),Rectangle::row(0.0, 0.0, 32.0, 32.0).take(4).collect(), 10)),
		(AnimationName::Enemy1, Animation::new(tileset.clone(),Rectangle::row(128.0, 0.0, 32.0, 32.0).take(4).collect(), 10)),
		(AnimationName::Enemy2, Animation::new(tileset.clone(),Rectangle::row(128.0, 32.0, 32.0, 32.0).take(4).collect(), 10)),
		(AnimationName::Enemy3, Animation::new(tileset.clone(),Rectangle::row(0.0, 32.0, 32.0, 32.0).take(4).collect(), 10)),
	].iter().cloned().collect();
	Ok(animations)
}

fn build_textures(ctx: &mut Context) ->tetra::Result<HashMap<TextureName, Texture>>{
	let textures: HashMap<TextureName, Texture> = [
		(TextureName::Life, Texture::from_file_data(ctx,include_bytes!("../assets/art/life_art.png"))?),
		(TextureName::Level, Texture::from_file_data(ctx,include_bytes!("../assets/art/level_art.png"))?),
		(TextureName::BulletDown, Texture::from_file_data(ctx,include_bytes!("../assets/art/shoot_0.png"))?),
		(TextureName::BulletUp, Texture::from_file_data(ctx,include_bytes!("../assets/art/shoot_1.png"))?),
		(TextureName::Background, Texture::from_file_data(ctx,include_bytes!("../assets/art/background.png"))?),
		(TextureName::Particle0, Texture::from_file_data(ctx,include_bytes!("../assets/art/particle_4.png"))?),
		(TextureName::Particle1, Texture::from_file_data(ctx,include_bytes!("../assets/art/particle_5.png"))?),
	].iter().cloned().collect();
	Ok(textures)
}

fn build_sounds() -> HashMap<SoundName, Sound>{
	let sounds: HashMap<SoundName, Sound> = [
		(SoundName::ShootSlow, Sound::from_file_data(include_bytes!("../assets/sound/shoot3.wav"))),
		(SoundName::ShootFast, Sound::from_file_data(include_bytes!("../assets/sound/shoot1.wav"))),
		(SoundName::Hurt, Sound::from_file_data(include_bytes!("../assets/sound/ouch.wav"))),
		(SoundName::Hurt2, Sound::from_file_data(include_bytes!("../assets/sound/ouch5.wav"))),
		(SoundName::Pickup, Sound::from_file_data(include_bytes!("../assets/sound/pickup.wav"))),
		(SoundName::NewLevel, Sound::from_file_data(include_bytes!("../assets/sound/level.wav"))),
	].iter().cloned().collect();
	sounds
}

fn build_texts(_ctx: &mut Context) -> Vec<Text>{
	let font = Font::default();
	let mut texts = vec![];
	texts.push(Text::new("", font, 24.0));
	texts.push(Text::new("", font, 24.0));
	texts.push(Text::new("", font, 18.0));
	texts.push(Text::new("GAME OVER", font, 32.0));
	texts
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SoundName {
	ShootSlow,
	ShootFast,
	Hurt,
	Hurt2,
	Pickup,
	NewLevel,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnimationName {
	Player,
	Enemy1,
	Enemy2,
	Enemy3,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextureName {
	Life,
	Level,
	BulletUp,
	BulletDown,
	Background,
	Particle0,
	Particle1,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextName {
	Life,
	Level,
	Score,
	GameOver,
}