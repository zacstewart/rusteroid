#![crate_name = "rusteroid"]
#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use sdl2_game_window::WindowSDL2;
use opengl_graphics::Gl;
use piston::input::{
  Press,
  Release,
  Keyboard
};
use piston::{
  EventIterator,
  EventSettings,
  WindowSettings,
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
  let opengl = piston::shader_version::opengl::OpenGL_3_2;
  let mut window: WindowSDL2 = WindowSDL2::new(
    opengl,
    WindowSettings {
      title: "Rusteroids".to_string(),
      size: settings::WINDOW_SIZE,
      fullscreen: false,
      exit_on_esc: true,
      samples: 0
    }
  );

  let game_iter_settings = EventSettings {
    updates_per_second: 120,
    max_frames_per_second: 60,
  };

  let mut app = app::App::new();
  let ref mut gl = Gl::new(opengl);

  for e in EventIterator::new(&mut window, &game_iter_settings) {
    match e {
      Render(args) => app.render(args, gl),
      Input(Press(Keyboard(key))) => app.key_press(key),
      Input(Release(Keyboard(key))) => app.key_release(key),
      Update(args) => app.update(args),
      _ => {}
    }
  }
}
