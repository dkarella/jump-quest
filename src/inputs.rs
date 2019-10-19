use std::collections::HashSet;

pub struct Inputs {
  pub keys_pressed: HashSet<String>,
}

impl Inputs {
  pub fn new() -> Inputs {
    Inputs {
      keys_pressed: HashSet::new(),
    }
  }
}