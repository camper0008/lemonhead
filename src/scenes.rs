mod child_room;
mod entryway;
mod kitchen;
mod living_room;
mod murder_living_room;
mod outside;
mod tutorial;

use std::marker::PhantomData;

use crate::{ctx::Ctx, state::State};

pub struct InteractableId(pub u8);

pub trait Item {
    fn id(&self) -> InteractableId;
}

pub struct Items(Vec<(f64, Box<dyn Item>)>);

impl Items {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, position: f64, item: impl Item + 'static) {
        self.0.push((position, Box::new(item)));
    }
}

pub trait Scene<C: Ctx> {
    fn draw(&self, ctx: &mut C, state: &State<C>) -> Result<(), C::Error>;
    fn interact(&self, ctx: &mut C, state: &mut State<C>, position: f64) -> Result<(), C::Error>;
    fn prepare_items(&self, state: &State<C>) -> Items;
    fn closest_item_within_distance(
        &self,
        state: &State<C>,
        position: f64,
    ) -> Option<Box<dyn Item>> {
        let items = self.prepare_items(state).0;
        if items.is_empty() {
            return None;
        }
        items
            .into_iter()
            .map(|(dist, item)| ((dist - position).abs(), item))
            .filter(|(dist, _)| *dist < 0.5)
            .min_by(|a, b| (a.0).total_cmp(&b.0))
            .map(|(_dist, item)| item)
    }
    fn should_draw_interact_popup(&self, state: &State<C>, position: f64) -> bool {
        self.closest_item_within_distance(state, position).is_some()
    }
}

pub enum Scenes<C: Ctx> {
    Tutorial,
    Entryway,
    LivingRoom,
    MurderLivingRoom,
    Outside,
    Kitchen,
    ChildRoom,
    _Phantom(PhantomData<C>),
}

impl<C: Ctx> Scenes<C> {
    pub fn inner(&self) -> &dyn Scene<C> {
        match self {
            Self::Tutorial => &tutorial::Tutorial,
            Self::Entryway => &entryway::Entryway,
            Self::LivingRoom => &living_room::LivingRoom,
            Self::MurderLivingRoom => &murder_living_room::MurderLivingRoom,
            Self::Outside => &outside::Outside,
            Self::Kitchen => &kitchen::Kitchen,
            Self::ChildRoom => &child_room::ChildRoom,
            Self::_Phantom(_) => unreachable!(),
        }
    }
}
