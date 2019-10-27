#[macro_use]
extern crate stdweb;

mod commands;

mod constants;
mod controllers;
mod draw;
mod game;
mod geometry;
mod inputs;
mod player;
mod screen;
mod tiles;


use game::Game;
use inputs::Inputs;
use screen::Screen;
use std::cell::RefCell;
use std::rc::Rc;
use stdweb::traits::*;
use stdweb::web::{event::KeyDownEvent, event::KeyUpEvent, IEventTarget};

const CANVAS_ELEMENT_ID: &str = "#canvas";
const SCREEN_WIDTH: f64 = 1024.;
const SCREEN_HEIGHT: f64 = 576.;

pub fn start() {
  let game = Game::new();
  let inputs = Rc::new(RefCell::new(Inputs::new()));
  let screen = Screen::new(CANVAS_ELEMENT_ID, SCREEN_WIDTH, SCREEN_HEIGHT);

  stdweb::web::document().add_event_listener({
    let inputs = inputs.clone();
    move |event: KeyDownEvent| {
      inputs.borrow_mut().set_key_pressed(&event.key());
    }
  });

  stdweb::web::document().add_event_listener({
    let inputs = inputs.clone();
    move |event: KeyUpEvent| {
      inputs.borrow_mut().unset_key_pressed(&event.key());
    }
  });

  game_loop(game, inputs, screen);
}

fn game_loop(mut game: Game, inputs: Rc<RefCell<Inputs>>, screen: Screen) {
  let mut old_time = 0.;
  let callback = move |time: f64| {
    game.update(&inputs.borrow());
    game.draw(&screen);

    // draw fps counter
    screen.set_font("14px Arial");
    screen.set_fill_style_color("black");
    screen.fill_text(
      &format!("FPS: {:.2}", 1. / ((time - old_time) / 1000.)),
      10.,
      20.,
    );
    old_time = time;
  };

  js! { @(no_return)
    const callback = @{callback};

    function loop(time) {
        requestAnimationFrame(loop);
        callback(time);
    }

    requestAnimationFrame(loop);
  }
}
