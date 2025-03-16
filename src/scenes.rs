mod child_room;
mod entryway;
mod kitchen;
mod living_room;
mod murder_living_room;
mod outside;
mod tutorial;

use crate::{logic::Unit, state::State};
use sdl2::render::WindowCanvas;

pub struct InteractableId(pub u8);

pub trait Item {
    fn id(&self) -> InteractableId;
}

pub struct Items(Vec<(Unit, Box<dyn Item>)>);

impl Items {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, position: impl Into<Unit>, item: impl Item + 'static) {
        self.0.push((position.into(), Box::new(item)));
    }
}

pub trait Scene {
    fn draw(
        &self,
        state: &State,
        canvas: &mut WindowCanvas,
        animation_timer: f64,
    ) -> Result<(), String>;
    fn interact(&self, state: &mut State, position: Unit);
    fn prepare_items(&self, state: &State) -> Items;
    fn closest_item_within_distance(&self, state: &State, position: Unit) -> Option<Box<dyn Item>> {
        let items = self.prepare_items(&state).0;
        if items.is_empty() {
            return None;
        }
        items
            .into_iter()
            .map(|(dist, item)| ((dist - position).abs(), item))
            .filter(|(dist, _)| *dist < Unit::new_decimal(0.5))
            .min_by(|a, b| (a.0).cmp(&b.0))
            .map(|(_dist, item)| item)
    }
    fn should_draw_interact_popup(&self, state: &State, position: Unit) -> bool {
        self.closest_item_within_distance(state, position).is_some()
    }
}

pub enum Scenes {
    Tutorial,
    Entryway,
    LivingRoom,
    MurderLivingRoom,
    Outside,
    Kitchen,
    ChildRoom,
}

impl Scenes {
    pub fn inner(&self) -> &dyn Scene {
        match self {
            Self::Tutorial => &tutorial::Tutorial,
            Self::Entryway => &entryway::Entryway,
            Self::LivingRoom => &living_room::LivingRoom,
            Self::MurderLivingRoom => &murder_living_room::MurderLivingRoom,
            Self::Outside => &outside::Outside,
            Self::Kitchen => &kitchen::Kitchen,
            Self::ChildRoom => &child_room::ChildRoom,
        }
    }
}
