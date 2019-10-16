#[macro_use]
extern crate stdweb;

mod game;
mod io;
mod screen;
mod models;
mod controllers;
mod draw;
mod geometry;

use std::cell::RefCell;
use std::rc::Rc;

use stdweb::web::{event::KeyDownEvent, event::KeyUpEvent, IEventTarget};
use stdweb::traits::*;

use game::Game;
use io::Io;
use screen::Screen;

const CANVAS_ELEMENT_ID: &str = "#canvas";
const SCREEN_WIDTH: f64 = 1024.;
const SCREEN_HEIGHT: f64 = 576.;

pub fn start() {
  let game = Game::new();
  let io = Rc::new(RefCell::new(Io::new()));
  let screen = Screen::new(CANVAS_ELEMENT_ID, SCREEN_WIDTH, SCREEN_HEIGHT);
  
  stdweb::web::document().add_event_listener({
    let io = io.clone();
    move |event: KeyDownEvent| {
      io.borrow_mut().keys_pressed.insert(event.key().clone());
    }
  });

  stdweb::web::document().add_event_listener({
    let io = io.clone();
    move |event: KeyUpEvent| {
      io.borrow_mut().keys_pressed.remove(&event.key());
    }
  });

  game_loop(game, io, screen);
}

fn game_loop(mut game: Game, io:Rc<RefCell<Io>>, screen: Screen) {
  let mut old_time = 0.;
  let callback = move |time: f64| {
    game.update(&io.borrow());
    game.draw(&screen);

    // draw fps counter
    screen.set_font("14px Arial");
    screen.set_fill_style_color("black");
    screen.fill_text(&format!("FPS: {:.2}", 1. / ((time - old_time) / 1000.)), 10., 20.);
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
