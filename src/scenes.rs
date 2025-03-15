mod child_room;
mod entryway;
mod kitchen;
mod living_room;
mod murder_living_room;
mod outside;
mod tutorial;

use child_room::ChildRoom;
use entryway::Entryway;
use kitchen::Kitchen;
use living_room::LivingRoom;
use murder_living_room::MurderLivingRoom;
use outside::Outside;
use tutorial::Tutorial;

use crate::scene::Scene;

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
            Self::Tutorial => &Tutorial,
            Self::Entryway => &Entryway,
            Self::LivingRoom => &LivingRoom,
            Self::MurderLivingRoom => &MurderLivingRoom,
            Self::Outside => &Outside,
            Self::Kitchen => &Kitchen,
            Self::ChildRoom => &ChildRoom,
        }
    }
}
