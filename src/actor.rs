use std::path::Path;

use crate::rect;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct Actor {
    position: (f64, f64),
    state: ActorState,
    asset: &'static str,
}

impl Actor {
    pub fn new(asset: &'static str) -> Self {
        Actor {
            position: (0.0, 0.0),
            state: ActorState::Idle,
            asset,
        }
    }

    pub fn x(&self) -> f64 {
        self.position.0
    }
    pub fn idle(&mut self) {
        self.state = ActorState::Idle;
    }
    pub fn run_left(&mut self) {
        self.state = ActorState::RunningLeft;
    }
    pub fn run_right(&mut self) {
        self.state = ActorState::RunningRight;
    }

    pub fn set_position(&mut self, x: f64, y: f64) {
        self.position.0 = x;
        self.position.1 = y;
    }
    pub fn offset_position(&mut self, x: f64, y: f64, delta_time: f64) {
        self.position.0 += x * delta_time;
        self.position.1 += y * delta_time;
    }

    pub fn draw(&mut self, canvas: &mut WindowCanvas, delta_time: f64) {
        let offset = if delta_time % 0.5 < 0.25 { 0 } else { 32 };
        let offset = offset
            + match self.state {
                ActorState::Idle => 0,
                ActorState::RunningRight => 64,
                ActorState::RunningLeft => 128,
            };
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture(Path::new(self.asset)).unwrap();

        canvas
            .copy(
                &texture,
                rect!(offset, 0, 32, 32),
                rect!(self.position.0, self.position.1, 64, 64),
            )
            .unwrap();
    }
}

enum ActorState {
    RunningLeft,
    RunningRight,
    Idle,
}
