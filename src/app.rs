use graphics::*;
use opengl_graphics::{
  Texture,
  Gl,
};
use piston::input::keyboard;
use piston::{
  AssetStore,
  GameWindow,
  RenderArgs,
  UpdateArgs
};
use ship;

pub struct App {
  asset_store: AssetStore,
  ship: ship::Ship
}

impl App {
  pub fn new() -> App {
    let asset_store = AssetStore::from_folder("../bin/assets");
    let image = asset_store.path("ship.png").unwrap();
    let image = Texture::from_path(&image).unwrap();
    let ship = ship::Ship::new(image);
    App { asset_store:asset_store, ship:ship }
  }

  pub fn render(&self, args: RenderArgs) {
    let ref mut gl = Gl::new();
    gl.viewport(0, 0, args.width as i32, args.height as i32);

    let c = Context::abs(args.width as f64, args.height as f64);
    c.rgb(0.0, 0.0, 0.0).draw(gl);

    self.ship.render(c, gl);
  }

  pub fn key_press<W: GameWindow>(&mut self, window: &mut W, key: keyboard::Key) {
    match key {
      keyboard::Up => self.ship.start_accelerating(),
      keyboard::Left => self.ship.start_turning_left(),
      keyboard::Right => self.ship.start_turning_right(),
      _ => {}
    }
  }

  pub fn key_release<W: GameWindow>(&mut self, window: &mut W, key: keyboard::Key) {
    match key {
      keyboard::Up => self.ship.stop_accelerating(),
      keyboard::Left => self.ship.stop_turning_left(),
      keyboard::Right => self.ship.stop_turning_right(),
      _ => {}
    }
  }

  pub fn update<W: GameWindow>(&mut self, window: &mut W, args: UpdateArgs) {
    self.ship.update();
  }
}
