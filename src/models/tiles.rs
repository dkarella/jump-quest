use crate::geometry::{Rectangle, Point};

pub const TILE_SIZE: f64 = 50.;

pub struct Tile {
  pub body: Rectangle,
}

impl Tile {
  pub fn new(x: f64, y: f64) -> Tile {
    Tile {
      body: Rectangle {
        tl: Point { x, y },
        width: TILE_SIZE,
        height: TILE_SIZE,
      }
    }
  }

  pub fn landing_sensor(&self) -> Rectangle {
    Rectangle {
      tl: Point {
        x: self.body.tl.x,
        y: self.body.tl.y,
      },
      width: self.body.height,
      height: 5.,
    }
  }
}