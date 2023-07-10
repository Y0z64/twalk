#![allow(dead_code)]

use ruscii::app::{App, State};
use ruscii::terminal::{Window, size};
use ruscii::drawing::{Pencil};
use ruscii::keyboard::{KeyEvent, Key};
use ruscii::spatial::{Vec2};
use ruscii::gui::{FPSCounter};


/// The player object of the game, it is implemented in a very stupid way
struct Player {
    player_pos: Vec2,
    player_move: Vec2,
    speed: f32,
}

impl Player {

    pub fn new() -> Player {
        Player { player_pos: Vec2::xy(size().x/2, size().y/2), 
                 player_move: Vec2::zero(), 
                 speed: 1f32
                }
    }

    pub fn update(&mut self) {
        let future_pos = self.player_pos + self.player_move;
        self.player_move.clear();

        if future_pos.x < (size().x - 1)
            && future_pos.x > 0
            && future_pos.y < (size().y - 1)
            && future_pos.y > 0
        {
            self.player_pos = future_pos;
        }
    }
}


/// Ray



fn main() {
    let mut fps_counter = FPSCounter::default();
    let mut app = App::default();
    let mut player: Player = Player::new();

    app.run(|app_state: &mut State, window: &mut Window| {
        let canvas_dimension: Vec2 = window.size();

        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        for key_down in app_state.keyboard().get_keys_down() {
            match key_down {
                Key::Left | Key::A => player.player_move = Vec2::x(-(2f32 * player.speed)),
                Key::Down | Key::S => player.player_move = Vec2::y(player.speed),
                Key::Up | Key::W => player.player_move = Vec2::y(-player.speed),
                Key::Right | Key::D => player.player_move = Vec2::x(2f32 * player.speed),
                _ => (),
            }
        }

        fps_counter.update();
        player.update();

        let mut pencil = Pencil::new(window.canvas_mut());
        
        pencil.draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(1, 1));
        pencil.draw_text(&format!("Canvas size: {}", canvas_dimension), Vec2::xy(1, 3));
        
        pencil.draw_char('A', player.player_pos);
        
    });
}