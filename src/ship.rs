use graphics::*;
use opengl_graphics::{
  Texture,
  Gl
};
use settings;

static MAX_X: f64 = settings::WINDOW_SIZE[0] as f64;
static MAX_Y: f64 = settings::WINDOW_SIZE[1] as f64;
static MAX_SPEED: f64 = 20.0;

enum Direction {
  Left,
  Right,
  Straight
}

pub struct Ship {
  texture: Texture,
  accelerating: bool,
  turning: Direction,
  pub bearing: f64,
  x_speed: f64,
  y_speed: f64,
  pub x_position: f64,
  pub y_position: f64
}

impl Ship {
  pub fn new(image: Texture) -> Ship {
    Ship {
      texture:image,
      accelerating:false,
      turning: Straight,
      bearing: 0.0,
      x_speed:0.0,
      y_speed:0.0,
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

  pub fn update(&mut self) {
    match self.turning {
      Left => self.bearing -= 0.02,
      Right => self.bearing += 0.02,
      Straight => {},
    }
    self.bearing = self.bearing % Float::two_pi();

    if self.accelerating {
      self.x_speed += 0.01 * self.bearing.sin();
      self.y_speed += 0.01 * self.bearing.cos();
    } else {
      if self.x_speed != 0.0 {
        //self.x_speed *= 0.995;
      }
      if self.y_speed != 0.0 {
        //self.y_speed *= 0.995;
      }
    }

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
}
