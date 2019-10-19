pub struct Point {
  pub x: f64,
  pub y: f64,
}

pub struct Rectangle {
  pub tl: Point,
  pub width: f64,
  pub height: f64,
}

impl Rectangle {
  pub fn intersects(&self, other: &Rectangle) -> bool {
    let (left, right) = if self.tl.x <= other.tl.x {
      (&self, &other)
    } else {
      (&other, &self)
    };

    let (top, bottom) = if self.tl.y <= other.tl.y {
      (&self, &other)
    } else {
      (&other, &self)
    };

    right.tl.x <= left.tl.x + left.width && top.tl.y + top.height >= bottom.tl.y
  }
}