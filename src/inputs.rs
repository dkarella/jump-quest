use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Key {
  ArrowDown,
  ArrowLeft,
  ArrowRight,
  ArrowUp,
  Space,
}

impl Key {
  fn from(s: &str) -> Option<Key> {
    use Key::*;
    match s {
      "ArrowDown" => Some(ArrowDown),
      "ArrowLeft" => Some(ArrowLeft),
      "ArrowRight" => Some(ArrowRight),
      "ArrowUp" => Some(ArrowUp),
      " " => Some(Space),
      _ => None,
    }
  }
}

pub struct Inputs {
  keys_pressed: HashSet<Key>,
}

impl Inputs {
  pub fn new() -> Inputs {
    Inputs {
      keys_pressed: HashSet::new(),
    }
  }

  pub fn set_key_pressed(&mut self, key: &str) {
    if let Some(key) = Key::from(key) {
      self.keys_pressed.insert(key);
    }
  }

  pub fn unset_key_pressed(&mut self, key: &str) {
    if let Some(key) = Key::from(key) {
      self.keys_pressed.remove(&key);
    }
  }

  pub fn keys_pressed(&self) -> HashSet<Key> {
    self.keys_pressed.iter().cloned().collect()
  }
}