use std::vec::Vec;
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
use ship::Ship;
use plasma::Plasma;
use asteroid::LargeSilicaceousAsteroid;

pub struct App {
  asset_store: AssetStore,
  ship: Ship,
  plasmas: Vec<Plasma>,
  asteroids: Vec<LargeSilicaceousAsteroid>
}

impl App {
  pub fn new() -> App {
    let asset_store = AssetStore::from_folder("../bin/assets");
    let image = asset_store.path("ship.png").unwrap();
    let image = Texture::from_path(&image).unwrap();
    let ship = Ship::new(image);
    let asteroids: Vec<LargeSilicaceousAsteroid> = range(0u, 3)
      .map(|_| LargeSilicaceousAsteroid::new(&asset_store))
      .collect();

    App {
      asset_store:asset_store,
      ship:ship,
      plasmas:vec!(),
      asteroids:asteroids
    }
  }

  pub fn render(&self, args: RenderArgs) {
    let ref mut gl = Gl::new();
    gl.viewport(0, 0, args.width as i32, args.height as i32);

    let c = Context::abs(args.width as f64, args.height as f64);
    c.rgba(0.0, 0.0, 0.0, 1.0).draw(gl);

    for asteroid in self.asteroids.iter() {
      asteroid.render(c, gl);
    }

    for plasma in self.plasmas.iter() {
      plasma.render(c, gl);
    }

    self.ship.render(c, gl);
  }

  pub fn update<W: GameWindow>(&mut self, window: &mut W, args: UpdateArgs) {
    let dead_plasmas: Vec<uint> = self.plasmas.iter().enumerate()
      .filter(|&(_, p)| p.dead).map(|(i, _)| i).collect();

    for &i in dead_plasmas.iter() {
      self.plasmas.remove(i);
    }

    for asteroid in self.asteroids.mut_iter() {
      asteroid.update(args);
    }

    for plasma in self.plasmas.mut_iter() {
      plasma.update(args);
    }

    self.ship.update();
  }

  pub fn add_plasma(&mut self, x_postition: f64, y_position: f64, bearing: f64) {
    let image = self.asset_store.path("plasma.png").unwrap();
    let image = Texture::from_path(&image).unwrap();
    let plasma: Plasma = Plasma::new(image, x_postition, y_position, bearing);
    self.plasmas.push(plasma);
  }

  pub fn key_press<W: GameWindow>(&mut self, window: &mut W, key: keyboard::Key) {
    match key {
      keyboard::Up => self.ship.start_accelerating(),
      keyboard::Left => self.ship.start_turning_left(),
      keyboard::Right => self.ship.start_turning_right(),
      keyboard::Space => {
        let x_pos = self.ship.x_position;
        let y_pos = self.ship.y_position;
        let bearing = self.ship.bearing;
        self.add_plasma(x_pos, y_pos, bearing);
      },
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
}
