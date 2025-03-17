use crate::sprite::Sprite;

#[derive(Clone, Copy)]
pub struct Rgb(pub u8, pub u8, pub u8);

#[derive(Hash, PartialEq, Eq)]
pub enum Key {
    Left,
    Right,
    Interact,
    Quit,
}

fn f64_min(left: f64, right: f64) -> f64 {
    if left < right {
        left
    } else {
        right
    }
}

pub enum Effect {
    Interact,
    Stab,
}

#[derive(PartialEq)]
pub enum Music {
    Outside,
    Lemonhead,
    RipBozo,
    Rich,
    Run,
    Heartbeat,
    HeartbeatChild,
    HeartbeatChildWithLemon,
    Ascend,
}

impl Effect {
    pub fn path(&self) -> &'static str {
        match self {
            Effect::Interact => "assets/click.ogg",
            Effect::Stab => "assets/stab.ogg",
        }
    }
    pub fn volume(&self) -> f32 {
        1.0
    }
}

impl Music {
    pub fn path(&self) -> &'static str {
        match self {
            Music::Rich => "assets/rich.ogg",
            Music::Outside => "assets/outside.ogg",
            Music::Lemonhead => "assets/lemonhead.ogg",
            Music::Run => "assets/run.ogg",
            Music::Heartbeat => "assets/heartbeat.ogg",
            Music::HeartbeatChild => "assets/heartbeat-child.ogg",
            Music::HeartbeatChildWithLemon => "assets/heartbeat-child-with-lemon.ogg",
            Music::Ascend => "assets/ascension.ogg",
            Music::RipBozo => "assets/rip-bozo.ogg",
        }
    }

    pub fn volume(&self) -> f32 {
        match self {
            Music::Lemonhead | Music::Ascend => 1.0,
            Music::RipBozo => 0.75,
            _ => 0.5,
        }
    }
}

pub trait Ctx {
    type Error;

    fn enqueue_background_fill(&mut self, color: Rgb);
    fn enqueue_sprite(&mut self, position: (f64, f64), size: (f64, f64), sprite: &impl Sprite);
    fn enqueue_border(&mut self) {
        let border_color = Rgb(50, 50, 50);
        let (left, top) = self.to_screen_position((0.0, 0.0));
        let (right, bottom) = self.to_screen_position((10.0, 10.0));
        let (win_width, win_height) = self.window_size();
        self.enqueue_screen_rect(border_color, (0.0, 0.0), (left, win_height));
        self.enqueue_screen_rect(border_color, (0.0, 0.0), (win_width, top));
        self.enqueue_screen_rect(border_color, (0.0, bottom), (win_width, top));
        self.enqueue_screen_rect(border_color, (right, 0.0), (left, win_height));
    }
    fn enqueue_screen_rect(&mut self, color: Rgb, position: (f64, f64), size: (f64, f64));
    fn to_screen_scale(&self, size: (f64, f64)) -> (f64, f64) {
        let (win_x, win_y) = self.window_size();
        let win_min = f64_min(win_x, win_y);
        let win_min = win_min - win_min % 10.0;
        let pixels_per_dot = win_min / 10.0;
        (size.0 * pixels_per_dot, size.1 * pixels_per_dot)
    }

    fn to_screen_position(&self, position: (f64, f64)) -> (f64, f64) {
        let (win_x, win_y) = self.window_size();
        let win_min = f64_min(win_x, win_y);
        let win_min = win_min - win_min % 10.0;
        let pixels_per_dot = win_min / 10.0;
        let center_x = (win_x - win_min) / 2.0;
        let center_y = (win_y - win_min) / 2.0;
        (
            center_x + position.0 * pixels_per_dot,
            center_y + position.1 * pixels_per_dot,
        )
    }
    fn window_size(&self) -> (f64, f64);

    fn play_effect(&mut self, effect: Effect) -> Result<(), Self::Error>;
    fn set_music(&mut self, music: Music) -> Result<(), Self::Error>;
    fn stop_music(&mut self) -> Result<(), Self::Error>;
    fn key_down(&self, key: Key) -> bool;

    fn setup(&mut self) -> Result<(), Self::Error>;
    fn finish(&mut self) -> Result<(), Self::Error>;
    fn seconds_elapsed(&self) -> f64;
}
