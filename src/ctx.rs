use crate::sprite::Sprite;

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

pub trait Ctx {
    type Error;
    fn fill_background(&mut self, color: Rgb) -> Result<(), Self::Error>;
    fn draw_sprite(
        &mut self,
        position: (f64, f64),
        size: (f64, f64),
        sprite: impl Sprite,
    ) -> Result<(), Self::Error>;
    fn to_world_scale(&self, size: (f64, f64)) -> (f64, f64) {
        let (win_x, win_y) = self.window_size();
        let win_min = f64_min(win_x, win_y);
        let win_min = win_min - win_min % 10.0;
        let pixels_per_dot = win_min / 10.0;
        (size.0 * pixels_per_dot, size.1 * pixels_per_dot)
    }
    fn to_world_position(&self, position: (f64, f64)) -> (f64, f64) {
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
    fn key_down(&self, key: Key) -> bool;
    fn pre_step(&mut self) -> Result<(), Self::Error>;
    fn post_step(&mut self) -> Result<(), Self::Error>;
}
