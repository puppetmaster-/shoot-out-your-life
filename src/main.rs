#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate log;
extern crate simple_logger;

mod scenes;
mod bullet;
mod enemy;
mod particle;
mod assets;

use crate::scenes::manager::SceneManager;
use crate::scenes::title::TitleScene;

use tetra::ContextBuilder;
use tetra::graphics::{Vec2, ScreenScaling};

#[macro_use]
extern crate lazy_static;

lazy_static! {
  static ref GAMEINFO: GameInformation = GameInformation::new();
}

fn main() -> tetra::Result {
    color_backtrace::install();
    simple_logger::init().unwrap();
    ContextBuilder::new(format!("Shoot out your life (LD44) v{}",
                    GAMEINFO.version).as_str(),
                    GAMEINFO.window.width as i32,
                    GAMEINFO.window.height as i32)
        .window_scale(2)
        .maximized(false)
        .fullscreen(false)
        .resizable(false)
        .scaling(ScreenScaling::ShowAllPixelPerfect)
        .vsync(false)
        .quit_on_escape(false)
        .build()?
        .run_with(|ctx| {
            let scene = TitleScene::new(ctx)?;
            Ok(SceneManager::new(Box::new(scene)))
        })
}

struct Window {
    width: u32,
    height: u32,
}

impl Window {
    pub fn get_half(&self) -> Vec2 {
        Vec2::new((self.width / 2) as f32, (self.height / 2) as f32)
    }
}


struct GameInformation{
    window: Window,
    version: String,
}

impl GameInformation{
    pub fn new() -> GameInformation{
        let version = env!("CARGO_PKG_VERSION").to_owned();
        GameInformation{
            window: Window { width: 240, height: 460},
            version,
        }
    }
}