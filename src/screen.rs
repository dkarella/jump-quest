use stdweb::web::{document, CanvasRenderingContext2d};
use stdweb::web::html_element::{CanvasElement};
use stdweb::traits::*;
use stdweb::unstable::TryInto;

pub struct Screen {
  canvas: CanvasElement,
  width: f64,
  height: f64,
}

impl Screen {
  pub fn new(canvas_id: &str, width: f64, height: f64) -> Screen {
    let canvas: CanvasElement = document()
      .query_selector(canvas_id)
      .unwrap()
      .unwrap()
      .try_into()
      .unwrap();

    Screen {
      canvas,
      width,
      height,
    }
  }

  pub fn set_font(&self, font: &str) {
    self.ctx().set_font(font);
  }

  pub fn set_fill_style_color(&self, color: &str) {
    self.ctx().set_fill_style_color(color);
  }

  pub fn fill_text(&self, text: &str, x: f64, y: f64) {
    self.ctx().fill_text(
      text,
      x * self.scaled_width(),
      y * self.scaled_height(),
      None
    );
  }

  pub fn fill_rect(&self, x: f64, y: f64, width: f64, height: f64) {
    let sw = self.scaled_width();
    let sh = self.scaled_height();
    self.ctx().fill_rect(x * sw, y * sh, width * sw, height * sh);
  }

  fn scaled_width(&self) -> f64 {
    self.canvas.width() as f64 / self.width
  }

  fn scaled_height(&self) -> f64 {
    self.canvas.height() as f64 / self.height
  }

  fn ctx(&self) -> CanvasRenderingContext2d {
    self.canvas.get_context().unwrap()
  }
}