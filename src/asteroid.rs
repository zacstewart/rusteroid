use std::rand;
use std::rand::distributions::{IndependentSample, Range};
use graphics::*;
use opengl_graphics::{
  Texture,
  Gl
};
use piston::{
  AssetStore,
  UpdateArgs
};
use settings;

static MAX_X: f64 = settings::WINDOW_SIZE[0] as f64;
static MAX_Y: f64 = settings::WINDOW_SIZE[1] as f64;
static MAX_SPEED: f64 = 2.0;

pub struct LargeSilicaceousAsteroid {
  texture: Texture,
  x_speed: f64,
  y_speed: f64,
  x_position: f64,
  y_position: f64,
  spin: f64,
  rotation: f64,
}

impl LargeSilicaceousAsteroid {
  pub fn new(asset_store: &AssetStore) -> LargeSilicaceousAsteroid {
    let image = asset_store.path("asteroids/silicaceous/large.png").unwrap();
    let image = Texture::from_path(&image).unwrap();

    let mut rng = rand::task_rng();

    let two_pi: f64 = Float::two_pi();
    let bearing = Range::new(-two_pi, two_pi).ind_sample(&mut rng);
    let between = Range::new(0.0, MAX_SPEED);
    let x_speed = between.ind_sample(&mut rng) * bearing.sin();
    let y_speed = between.ind_sample(&mut rng) * bearing.cos();
    let x_position = MAX_X * rand::random::<f64>();
    let y_position = MAX_Y * rand::random::<f64>();
    let between = Range::new(-1.0, 1.0);
    let spin = between.ind_sample(&mut rng);

    LargeSilicaceousAsteroid {
      texture:image,
      x_speed:x_speed,
      y_speed:y_speed,
      x_position:x_position,
      y_position:y_position,
      spin:spin,
      rotation:0.0,
    }
  }

  pub fn update(&mut self, args: UpdateArgs) {
    self.rotation += self.spin * args.dt;
    self.rotation = self.rotation % Float::two_pi();

    self.x_position += self.x_speed;
    self.y_position -= self.y_speed;

    if self.x_position > MAX_X {
      self.x_position = 0.0;
    } else if self.x_position < 0.0 {
      self.x_position = MAX_X
    }

    if self.y_position > MAX_Y {
      self.y_position = 0.0;
    } else if self.y_position < 0.0 {
      self.y_position = MAX_Y
    }
  }

  pub fn render(&self, context: Context, gl: &mut Gl) {
    let (xs, ys) = self.texture.get_size();

    context
      .trans(self.x_position, self.y_position)
      .rot_rad(self.rotation)
      .trans(-(xs as f64 / 2.0), -(ys as f64 / 2.0))
      .image(&self.texture)
      .draw(gl);
  }
}
