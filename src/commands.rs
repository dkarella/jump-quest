use std::cmp::Ordering;

#[derive(Eq)]
pub enum Command {
  Jump,
  Stand,
  MoveLeft,
  MoveRight,
}

impl Command {
  fn priority(&self) -> u8 {
    use Command::*;

    match self {
      &Stand => 0,
      &MoveLeft => 1,
      &MoveRight => 1,
      &Jump => 2,
    }
  }
}

impl Ord for Command {
  fn cmp(&self, other: &Self) -> Ordering {
    self.priority().cmp(&other.priority())
  }
}

impl PartialOrd for Command {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for Command {
  fn eq(&self, other: &Self) -> bool {
    self == other
  }
}