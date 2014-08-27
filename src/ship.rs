use graphics::*;
use opengl_graphics::{
  Texture,
  Gl
};
use piston::AssetStore;
use settings;

static MAX_X: f64 = settings::WINDOW_SIZE[0] as f64;
static MAX_Y: f64 = settings::WINDOW_SIZE[1] as f64;

enum Direction {
  Left,
  Right,
  Straight
}

pub struct Ship {
  texture: Texture,
  accelerating: bool,
  turning: Direction,
  bearing: f64,
  x_acceleration: f64,
  y_acceleration: f64,
  x_position: f64,
  y_position: f64
}

impl Ship {
  pub fn new(image: Texture) -> Ship {
    Ship {
      texture:image,
      accelerating:false,
      turning: Straight,
      bearing: 0.0,
      x_acceleration:0.0,
      y_acceleration:0.0,
      x_position:100.0,
      y_position:100.0
    }
  }

  pub fn render(&self, context: Context, gl: &mut Gl) {
    let (xs, ys) = self.texture.get_size();
    context
      .trans(self.x_position, self.y_position)
      .rot_rad(self.bearing)
      .trans(-(xs as f64 / 2.0), -(ys as f64 / 2.0))
      .image(&self.texture)
      .draw(gl);
  }

  pub fn start_accelerating(&mut self) {
    self.accelerating = true;
  }

  pub fn stop_accelerating(&mut self) {
    self.accelerating = false;
  }

  pub fn start_turning_left(&mut self) {
    self.turning = Left;
  }

  pub fn start_turning_right(&mut self) {
    self.turning = Right;
  }

  pub fn stop_turning_left(&mut self) {
    match self.turning {
      Left => self.turning = Straight,
      _ => {}
    }
  }

  pub fn stop_turning_right(&mut self) {
    match self.turning {
      Right => self.turning = Straight,
      _ => {}
    }
  }

  pub fn update(&mut self) {
    match self.turning {
      Left => self.bearing -= 0.01,
      Right => self.bearing += 0.01,
      Straight => {},
    }
    self.bearing = self.bearing % Float::two_pi();

    if self.accelerating {
      self.x_acceleration += 0.01 * self.bearing.sin();
      self.y_acceleration += 0.01 * self.bearing.cos();
    } else {
      if self.x_acceleration != 0.0 {
        self.x_acceleration *= 0.99;
      }
      if self.y_acceleration != 0.0 {
        self.y_acceleration *= 0.99;
      }
    }

    self.x_position += self.x_acceleration;
    self.y_position -= self.y_acceleration;

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
}
