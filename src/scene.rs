use crate::state::State;
use sdl2::render::WindowCanvas;

pub trait Scene {
    fn draw_scenery(
        &self,
        state: &State,
        canvas: &mut WindowCanvas,
        animation_timer: f64,
    ) -> Result<(), String>;
    fn draw_interact_popup(
        &self,
        state: &State,
        canvas: &mut WindowCanvas,
        position: f64,
        animation_timer: f64,
    ) -> Result<(), String>;
    fn interact(&self, state: &mut State, position: f64) -> Result<(), String>;
}
