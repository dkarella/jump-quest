use crate::commands::Command;
use crate::constants::*;
use crate::game::World;
use crate::inputs::Key;
use crate::models::player::PlayerState;

pub fn handle_inputs<'a, I>(world: &World, keys_pressed: I, keys_unpressed: I) -> Vec<Command>
where
  I: Iterator<Item = &'a Key>,
{
  let World { player, .. } = world;
  let mut commands = Vec::new();

  for key in keys_pressed {
    match key {
      &Key::ArrowLeft => commands.push(Command::MoveLeft),
      &Key::ArrowRight => commands.push(Command::MoveRight),
      Key::Space => match player.state {
        PlayerState::Walking { .. } | PlayerState::Standing => {
          commands.push(Command::Jump);
        }
        _ => (),
      },
      _ => (),
    }
  }

  for key in keys_unpressed {
    match key {
      Key::ArrowLeft | Key::ArrowRight => match player.state {
        PlayerState::Walking { .. } => {
          commands.push(Command::Stand);
        }
        _ => (),
      },
      _ => (),
    }
  }

  commands
}

pub fn update_player(world: &mut World, mut commands: Vec<Command>) {
  let World { player, tiles } = world;

  commands.sort();
  commands.iter().for_each(|command| match command {
    &Command::Stand => {
      player.state = match player.state {
        PlayerState::Standing => PlayerState::Standing,
        PlayerState::Walking { .. } => PlayerState::Standing,
        PlayerState::Jumping { v_y, v_x } => PlayerState::Jumping { v_x, v_y },
      };
    }
    &Command::Jump => {
      player.state = match player.state {
        PlayerState::Walking { v } => PlayerState::Jumping {
          v_x: v,
          v_y: JUMP_FORCE,
        },
        PlayerState::Standing => PlayerState::Jumping {
          v_x: 0.,
          v_y: JUMP_FORCE,
        },
        PlayerState::Jumping { v_x, v_y } => PlayerState::Jumping { v_x, v_y },
      };
    }
    &Command::MoveLeft => {
      player.state = match player.state {
        PlayerState::Jumping { v_x, v_y } => PlayerState::Jumping {
          v_x: (v_x - JUMPING_MOVE_ACCELERATION).max(-1. * WALK_SPEED),
          v_y,
        },
        PlayerState::Walking { v } => PlayerState::Walking {
          v: (v - WALK_SPEED).max(-1. * WALK_SPEED),
        },
        PlayerState::Standing => PlayerState::Walking {
          v: -1. * WALK_SPEED,
        },
      }
    }
    &Command::MoveRight => {
      player.state = match player.state {
        PlayerState::Jumping { v_x, v_y } => PlayerState::Jumping {
          v_x: (v_x + JUMPING_MOVE_ACCELERATION).min(WALK_SPEED),
          v_y,
        },
        PlayerState::Walking { v } => PlayerState::Walking {
          v: (v + WALK_SPEED).min(WALK_SPEED),
        },
        PlayerState::Standing => PlayerState::Walking { v: WALK_SPEED },
      }
    }
  });

  match player.state {
    PlayerState::Jumping { v_y, v_x } => {
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
            v_x,
            v_y: (v_y + GRAVITY).min(FALL_SPEED),
          };
        }
      }
    }

    PlayerState::Walking { v } => {
      if tiles
        .iter()
        .any(|tile| player.bottom_sensor().intersects(&tile.landing_sensor()))
      {
        player.body.tl.x += v;
      } else {
        player.state = PlayerState::Jumping { v_x: v, v_y: 0. };
      }
    }

    PlayerState::Standing => {
      if !tiles
        .iter()
        .any(|tile| player.bottom_sensor().intersects(&tile.landing_sensor()))
      {
        player.state = PlayerState::Jumping { v_x: 0., v_y: 0. };
      }
    }
  }
}
