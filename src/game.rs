use crate::controllers;
use crate::draw;

use crate::inputs::Inputs;
use crate::models::player::Player;
use crate::models::tiles::{Tile, TILE_SIZE};
use crate::screen::Screen;
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
  prev_keys_pressed: HashSet<String>,
}

impl Game {
  pub fn new() -> Game {
    Game {
      world: World::new(),
      prev_keys_pressed: HashSet::new(),
    }
  }

  pub fn update(&mut self, inputs: &Inputs) {
    controllers::handle_inputs(&mut self.world, inputs, &self.prev_keys_pressed);
    controllers::update_player(&mut self.world);
    self.prev_keys_pressed = inputs.keys_pressed.iter().cloned().collect();
  }

  pub fn draw(&self, screen: &Screen) {
    draw::clear(screen);
    draw::draw_tiles(screen, &self.world.tiles);
    draw::draw_player(screen, &self.world.player);
  }
}