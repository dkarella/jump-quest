use crate::io::Io;
use crate::screen::Screen;
use crate::models::player::Player;
use crate::models::tiles::{Tile, TILE_SIZE};
use crate::controllers;
use crate::draw;

pub struct World {
  pub player: Player,
  pub tiles: Vec<Tile>,
}

impl World {
  fn new() -> World {
    World {
      player: Player::new(),
      tiles: vec![
        Tile::new(TILE_SIZE * 0., 400.),  
        Tile::new(TILE_SIZE * 1., 400.),  
        Tile::new(TILE_SIZE * 2., 400.),  
        Tile::new(TILE_SIZE * 3., 400.),  
        Tile::new(TILE_SIZE * 4., 400.),  
        Tile::new(TILE_SIZE * 5., 400.),
        Tile::new(TILE_SIZE * 6., 400.),  
        Tile::new(TILE_SIZE * 7., 400.),  
        Tile::new(TILE_SIZE * 8., 400.),  
        Tile::new(TILE_SIZE * 9., 400.),  
        Tile::new(TILE_SIZE * 7., 315.),
        Tile::new(TILE_SIZE * 8., 315.),
        Tile::new(TILE_SIZE * 9., 315.),
        Tile::new(TILE_SIZE * 3., 230.),
        Tile::new(TILE_SIZE * 4., 230.),
        Tile::new(TILE_SIZE * 5., 230.),
      ],
    }
  }
}

pub struct Game {
  pub world: World,
}

impl Game {
  pub fn new() -> Game {
    Game {
      world: World::new(),
    }
  }

  pub fn update(&mut self, io: &Io) {
    controllers::handle_player_input(&mut self.world, io);
 }

  pub fn draw(&self, screen: &Screen) {
    draw::clear(screen);
    draw::draw_tiles(screen, &self.world.tiles);
    draw::draw_player(screen, &self.world.player);
  }
}