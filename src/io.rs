use std::collections::HashSet;

pub struct Io {
  pub keys_pressed: HashSet<String>,
}

impl Io {
  pub fn new() -> Io {
    Io {
      keys_pressed: HashSet::new(),
    }
  }
}