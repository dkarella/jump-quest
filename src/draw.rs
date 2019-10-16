use crate::models::player::Player;
use crate::models::tiles::Tile;
use crate::geometry::Rectangle;
use crate::screen::Screen;

pub fn clear(screen: &Screen) {
  screen.set_fill_style_color("white");
  screen.fill_rect(0., 0., 800., 500.);
}

pub fn draw_player(screen: &Screen, player: &Player) {
  screen.set_fill_style_color("blue");
  draw_rect(screen, &player.body);
}

pub fn draw_tiles(screen: &Screen, tiles: &Vec<Tile>) {
  screen.set_fill_style_color("black");
  tiles.iter().for_each(|tile| draw_rect(screen, &tile.body));
}

fn draw_rect(screen: &Screen, rectangle: &Rectangle) {
    screen.fill_rect(rectangle.tl.x, rectangle.tl.y, rectangle.width, rectangle.height);
}