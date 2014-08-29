#![crate_name = "rusteroid"]
#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use sdl2_game_window::GameWindowSDL2;
use piston::input;
use piston::{
  GameIterator,
  GameIteratorSettings,
  GameWindowSettings,
  Render,
  Input,
  Update
};

mod settings;
mod app;
mod ship;
mod plasma;
mod asteroid;

fn main() {
  let mut window: GameWindowSDL2 = GameWindowSDL2::new(
    piston::shader_version::opengl::OpenGL_3_2,
    GameWindowSettings {
      title: "Rusteroids".to_string(),
      size: settings::WINDOW_SIZE,
      fullscreen: false,
      exit_on_esc: true,
    }
  );

  let game_iter_settings = GameIteratorSettings {
    updates_per_second: 120,
    max_frames_per_second: 60,
  };

  let mut app = app::App::new();

  for e in GameIterator::new(&mut window, &game_iter_settings) {
    match e {
      Render(args) => app.render(args),
      Input(input::KeyPress { key, .. }) => app.key_press(&mut window, key),
      Input(input::KeyRelease { key, .. }) => app.key_release(&mut window, key),
      Update(args) => app.update(&mut window, args),
      _ => {}
    }
  }
}
