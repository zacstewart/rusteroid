use graphics::*;
use opengl_graphics::{
  Texture,
  Gl
};
use settings;
use piston::UpdateArgs;

static MAX_X: f64 = settings::WINDOW_SIZE[0] as f64;
static MAX_Y: f64 = settings::WINDOW_SIZE[1] as f64;
static TIME_TO_LIVE: f64 = 0.5;

pub struct Plasma {
  texture: Texture,
  x_position: f64,
  y_position: f64,
  x_speed: f64,
  y_speed: f64,
  bearing: f64,
  time_to_live: f64,
  pub dead: bool
}

impl Plasma {
  pub fn new(image: Texture, x_position: f64, y_position: f64, bearing: f64) -> Plasma {
    let x_speed = 10.0 * bearing.sin();
    let y_speed = 10.0 * bearing.cos();
    Plasma {
      texture:image,
      x_position:x_position,
      y_position:y_position,
      x_speed:x_speed,
      y_speed:y_speed,
      bearing:bearing,
      time_to_live:TIME_TO_LIVE,
      dead: false
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

  pub fn update(&mut self, args: UpdateArgs) {
    self.time_to_live -= args.dt;

    if self.time_to_live <= 0.0 {
      self.dead = true
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
}
