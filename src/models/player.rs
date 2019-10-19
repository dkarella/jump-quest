use crate::geometry::{Point, Rectangle};

pub enum PlayerState {
  Standing,
  Walking,
  Jumping,
  Climbing,
}

pub struct Player {
  pub body: Rectangle,
  pub v_x: f64,
  pub v_y: f64,
  pub a_x: f64,
  pub a_y: f64,
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
      v_x: 0.,
      v_y: 0.,
      a_x: 0.,
      a_y: 0.,
      state: PlayerState::Jumping,
    }
  }

  pub fn bottom_sensor(&self) -> Rectangle {
    Rectangle {
      tl: Point {
        x: self.body.tl.x,
        y: self.body.tl.y + self.body.height,
      },
      width: self.body.width,
      height: 5.,
    }
  }
}