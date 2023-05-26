use std::path::Path;

use crate::rect;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct Actor {
    position: (i32, i32),
    state: ActorState,
    animation_cycle: u32,
    asset: &'static str,
}

impl Actor {
    pub fn new(asset: &'static str) -> Self {
        Actor {
            position: (0, 0),
            state: ActorState::Idle,
            animation_cycle: 0,
            asset,
        }
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
    pub fn offset_position(&mut self, x: i32, y: i32) {
        self.position.0 += x;
        self.position.1 += y;
    }

    pub fn present(&mut self, canvas: &mut WindowCanvas) {
        self.animation_cycle += 1;
        self.animation_cycle %= 500;
        let offset = if self.animation_cycle < 250 { 0 } else { 32 };
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
                rect!(self.position.0 / 4, self.position.1 / 4, 64, 64),
            )
            .unwrap();
    }
}

enum ActorState {
    RunningLeft,
    RunningRight,
    Idle,
}
