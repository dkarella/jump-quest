use crate::models::player::PlayerState;
use crate::game::World;
use crate::io::Io;

const GRAVITY: f64 = 0.5;
const FALL_SPEED: f64 = 11.;
const WALK_SPEED: f64 = 2.;

pub fn handle_player_input(world: &mut World, io: &Io) {
  let mut force_jump = false;
  for key in io.keys_pressed.iter() {
    match key.as_str() {
      "ArrowLeft" => world.player.body.tl.x -= WALK_SPEED,
      "ArrowRight" => world.player.body.tl.x += WALK_SPEED,
      " " => match world.player.state {
        PlayerState::Walking | PlayerState::Standing => {
          world.player.v_y = -9.25;
          world.player.state = PlayerState::Jumping;
          force_jump = true;
        },
        _ => ()
      }
      _ => ()
    }
  }

  match world.player.state {
    PlayerState::Jumping => {
      if !force_jump {
        if let Some(tile) = world.tiles.iter().find(|tile| world.player.bottom_sensor().intersects(&tile.landing_sensor())) {
          world.player.state = PlayerState::Standing;
          world.player.v_y = 0.;
          world.player.body.tl.y = tile.body.tl.y - world.player.body.height;
          return;        
        }
      } 

      world.player.v_y = world.player.v_y + GRAVITY;

      if world.player.v_y > FALL_SPEED {
        world.player.v_y = FALL_SPEED;
      }

      world.player.body.tl.y += world.player.v_y;
    },

    _ => {
      world.player.v_y = 0.;      
      if !world.tiles.iter().any(|tile| world.player.bottom_sensor().intersects(&tile.landing_sensor())) {
        world.player.state = PlayerState::Jumping;
        return;        
      }
    }
  }
}
