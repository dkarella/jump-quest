use crate::controllers;
use crate::draw;
use crate::inputs::{Inputs, Key};
use crate::player::Player;
use crate::screen::Screen;
use crate::tiles::{Tile, TILE_SIZE};
use std::collections::HashSet;

pub struct World {
  pub player: Player,
  pub tiles: Vec<Tile>,
}

impl World {
  fn new() -> World {
    World {
      player: Player::new(),
      tiles: vec![
        Tile::new(TILE_SIZE * 0., 500.),
        Tile::new(TILE_SIZE * 1., 500.),
        Tile::new(TILE_SIZE * 2., 500.),
        Tile::new(TILE_SIZE * 3., 500.),
        Tile::new(TILE_SIZE * 4., 500.),
        Tile::new(TILE_SIZE * 5., 500.),
        Tile::new(TILE_SIZE * 6., 500.),
        Tile::new(TILE_SIZE * 7., 500.),
        Tile::new(TILE_SIZE * 8., 500.),
        Tile::new(TILE_SIZE * 9., 500.),
        Tile::new(TILE_SIZE * 7., 415.),
        Tile::new(TILE_SIZE * 8., 415.),
        Tile::new(TILE_SIZE * 9., 415.),
        Tile::new(TILE_SIZE * 3., 330.),
        Tile::new(TILE_SIZE * 4., 330.),
        Tile::new(TILE_SIZE * 5., 330.),
      ],
    }
  }
}

pub struct Game {
  world: World,
  prev_keys_pressed: HashSet<Key>,
}

impl Game {
  pub fn new() -> Game {
    Game {
      world: World::new(),
      prev_keys_pressed: HashSet::new(),
    }
  }

  pub fn update(&mut self, inputs: &Inputs) {
    let keys_pressed: HashSet<Key> = inputs.keys_pressed();
    let keys_unpressed: HashSet<Key> = self
      .prev_keys_pressed
      .difference(&keys_pressed)
      .cloned()
      .collect();

    let commands =
      controllers::handle_inputs(&mut self.world, keys_pressed.iter(), keys_unpressed.iter());

    controllers::update_player(&mut self.world, commands);

    self.prev_keys_pressed = keys_pressed;
  }

  pub fn draw(&self, screen: &Screen) {
    draw::clear(screen);
    draw::draw_tiles(screen, &self.world.tiles);
    draw::draw_player(screen, &self.world.player);
  }
}