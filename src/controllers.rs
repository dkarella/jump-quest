
use crate::game::World;
use crate::inputs::Inputs;
use crate::models::player::PlayerState;
use std::collections::HashSet;

const GRAVITY: f64 = 0.5;
const FALL_SPEED: f64 = 11.;
const WALK_SPEED: f64 = 2.;
const JUMPING_MOVE_ACCELERATION: f64 = 0.05;

pub fn handle_inputs(world: &mut World, inputs: &Inputs, prev_keys_pressed: &HashSet<String>) {
  let Inputs { keys_pressed } = inputs;
  let World { player, .. } = world;
  let keys_pressed: HashSet<String> = keys_pressed.iter().cloned().collect();
  let keys_unpressed = prev_keys_pressed.difference(&keys_pressed);

  for key in keys_pressed.iter() {
    match key.as_str() {
      "ArrowLeft" | "ArrowRight" => match player.state {
        PlayerState::Jumping { v_x, a_y, v_y, .. } => {
          player.state = PlayerState::Jumping {
            a_x: if key.as_str() == "ArrowRight" {
              JUMPING_MOVE_ACCELERATION
            } else {
              -1. * JUMPING_MOVE_ACCELERATION
            },
            a_y,
            v_y,
            v_x,
          };
        }

        PlayerState::Walking { v, .. } => {
          let a = if key.as_str() == "ArrowRight" {
            WALK_SPEED
          } else {
            -1. * WALK_SPEED
          };

          player.state = PlayerState::Walking { a, v };
        }

        PlayerState::Standing => {
          let a = if key.as_str() == "ArrowRight" {
            WALK_SPEED
          } else {
            -1. * WALK_SPEED
          };

          player.state = PlayerState::Walking { a, v: 0. };
        }
      },

      " " => match player.state {
        PlayerState::Walking { a, v } => {
          player.state = PlayerState::Jumping {
            a_x: a,
            v_x: v,
            a_y: GRAVITY,
            v_y: -9.25,
          };
        }

        PlayerState::Standing => {
          player.state = PlayerState::Jumping {
            a_x: 0.,
            v_x: 0.,
            a_y: GRAVITY,
            v_y: -9.25,
          };
        }

        _ => (),
      },
      _ => (),
    }
  }

  for key in keys_unpressed {
    match key.as_str() {
      "ArrowLeft" | "ArrowRight" => match player.state {
        PlayerState::Walking { .. } => {
          player.state = PlayerState::Standing;
        }
        PlayerState::Jumping { v_x, a_y, v_y, .. } => {
          player.state = PlayerState::Jumping {
            a_x: 0.,
            v_x,
            a_y,
            v_y,
          };
        }
        _ => (),
      },
      _ => (),
    }
  }
}

pub fn update_player(world: &mut World) {
  let World { player, tiles } = world;

  match player.state {
    PlayerState::Jumping { a_y, v_y, a_x, v_x } => {
      let tile = tiles
        .iter()
        .find(|tile| player.bottom_sensor().intersects(&tile.landing_sensor()));

      match tile {
        Some(tile) if v_y >= 0. => {
          player.state = PlayerState::Standing;
          player.body.tl.y = tile.body.tl.y - player.body.height;
          player.body.tl.x += v_x;
        }
        _ => {
          player.body.tl.x += v_x;
          player.body.tl.y += v_y;

          player.state = PlayerState::Jumping {
            a_y,
            a_x,
            v_x: match v_x + a_x {
              v_x if v_x >= 0. => v_x.min(WALK_SPEED),
              v_x => v_x.max(-1. * WALK_SPEED),
            },
            v_y: (v_y + a_y).min(FALL_SPEED),
          };
        }
      }
    }

    PlayerState::Walking { a, v } => {
      if tiles
        .iter()
        .any(|tile| player.bottom_sensor().intersects(&tile.landing_sensor()))
      {
        player.body.tl.x += v;
        player.state = PlayerState::Walking {
          a,
          v: match a + v {
            a if a >= 0. => a.min(WALK_SPEED),
            a => a.max(-1. * WALK_SPEED),
          },
        }
      } else {
        player.state = PlayerState::Jumping {
          a_x: 0.,
          v_x: v,
          a_y: GRAVITY,
          v_y: 0.,
        };
      }
    }

    PlayerState::Standing => (),
  }
}
