use crate::state::State;
use sdl2::render::WindowCanvas;

pub struct Id(pub u8);

pub trait Item {
    fn id(&self) -> Id;
}

pub struct Items(Vec<(u8, Box<dyn Item>)>);

impl Items {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, position: u8, item: impl Item + 'static) {
        self.0.push((position, Box::new(item)));
    }
}

pub trait Scene {
    fn draw(
        &self,
        state: &State,
        canvas: &mut WindowCanvas,
        animation_timer: f64,
    ) -> Result<(), String>;
    fn interact(&self, state: &mut State, position: f64);
    fn prepare_items(&self, state: &State) -> Items;
    fn closest_item_within_distance(&self, state: &State, position: f64) -> Option<Box<dyn Item>> {
        let items = self.prepare_items(&state).0;
        if items.is_empty() {
            return None;
        }

        items
            .into_iter()
            .map(|(dist, item)| ((dist as f64 - position).abs(), item))
            .filter(|(dist, _)| *dist < 0.5)
            .min_by(|a, b| (a.0).total_cmp(&b.0))
            .map(|(_dist, item)| item)
    }
    fn should_draw_interact_popup(&self, state: &State, position: f64) -> bool {
        self.closest_item_within_distance(state, position).is_some()
    }
}
