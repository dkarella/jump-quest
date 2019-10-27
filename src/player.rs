use crate::geometry::{Point, Rectangle};

#[derive(PartialEq)]
pub enum PlayerState {
  Standing,
  Walking { v: f64 },
  Jumping { v_x: f64, v_y: f64 },
}

pub struct Player {
  pub body: Rectangle,
  pub state: PlayerState,
}

impl Player {
  pub fn new() -> Player {
    Player {
      body: Rectangle {
        tl: Point { x: 300., y: 100. },
        width: 50.,
        height: 50.,
      },
      state: PlayerState::Jumping { v_x: 0., v_y: 0. },
    }
  }

  pub fn bottom_sensor(&self) -> Rectangle {
    Rectangle {
      tl: Point {
        x: self.body.tl.x + 5.,
        y: self.body.tl.y + self.body.height,
      },
      width: self.body.width - 10.,
      height: 4.,
    }
  }
}