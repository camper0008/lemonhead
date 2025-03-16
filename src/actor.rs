use std::path::Path;

use crate::globals::PIXEL_PER_DOT;
use crate::logic::Unit;
use crate::rect;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct Actor {
    position: (Unit, Unit),
    state: ActorState,
    asset: &'static str,
}

enum ActorState {
    RunningLeft,
    RunningRight,
    Idle,
}

impl Actor {
    pub fn new(asset: &'static str) -> Self {
        Actor {
            position: (Unit::new(0), Unit::new(0)),
            state: ActorState::Idle,
            asset,
        }
    }

    pub fn x(&self) -> Unit {
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

    pub fn set_position(&mut self, x: impl Into<Unit>, y: impl Into<Unit>) {
        self.position.0 = x.into();
        self.position.1 = y.into();
    }
    pub fn offset_position(
        &mut self,
        x: impl Into<Unit>,
        y: impl Into<Unit>,
        delta_time: impl Into<Unit>,
    ) {
        let delta_time = delta_time.into();
        self.position.0 += x.into() * delta_time;
        self.position.1 += y.into() * delta_time;
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, animation_timer: f64) -> Result<(), String> {
        let offset = if animation_timer % 0.5 < 0.25 { 0 } else { 32 };
        let offset = offset
            + match self.state {
                ActorState::Idle => 0,
                ActorState::RunningRight => 64,
                ActorState::RunningLeft => 128,
            };
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture(Path::new(self.asset))?;

        canvas.copy(
            &texture,
            rect!(offset, 0, 32, 32),
            rect!(
                self.position.0.decimal() * PIXEL_PER_DOT,
                self.position.1.decimal() * PIXEL_PER_DOT,
                PIXEL_PER_DOT,
                PIXEL_PER_DOT
            ),
        )
    }
}
