use crate::state::State;
use sdl2::render::WindowCanvas;

pub trait Scene {
    fn draw_scenery(
        &self,
        state: &State,
        canvas: &mut WindowCanvas,
        animation_timer: f64,
    ) -> Result<(), String>;
    fn should_draw_interact_popup(&self, state: &State, position: f64) -> bool;
    fn interact(&self, state: &mut State, position: f64);
}
