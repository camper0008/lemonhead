use crate::{ctx::Ctx, scenes::Scenes};

pub struct Tutorial {
    pub coin: bool,
}

pub struct Outside {
    pub key_collected: bool,
}

pub struct Entryway {
    pub coins: [bool; 4],
}

impl Entryway {
    pub fn all_coins_collected(&self) -> bool {
        self.coins.iter().all(|v| *v)
    }
}

pub struct Kitchen {
    pub coins: [bool; 3],
    pub weapon_collected: bool,
}

impl Kitchen {
    pub fn all_coins_collected(&self) -> bool {
        self.coins.iter().all(|v| *v)
    }
}

pub struct LivingRoom {
    pub coins: [bool; 2],
    pub dad_attack_seconds: f64,
    pub has_escaped_dad: bool,
}

impl LivingRoom {
    pub fn all_coins_collected(&self) -> bool {
        self.coins.iter().all(|v| *v)
    }
}

pub struct MurderLivingRoom {
    pub murderous_intent: bool,
    pub dad_dead: bool,
}

pub struct ChildRoom {
    pub child_stabs: u8,
}

impl ChildRoom {
    pub fn child_dead(&self) -> bool {
        self.child_stabs > 0
    }
}

pub enum EndingChosen {
    Ascended,
    Escaped,
}

pub struct State<C: Ctx> {
    pub tutorial: Tutorial,
    pub outside: Outside,
    pub entryway: Entryway,
    pub kitchen: Kitchen,
    pub living_room: LivingRoom,
    pub murder_living_room: MurderLivingRoom,
    pub child_room: ChildRoom,
    pub ending_chosen: Option<EndingChosen>,
    pub scene_changed: Option<(f64, Scenes<C>)>,
}

fn no_coins_collected<const N: usize>() -> [bool; N] {
    [false; N]
}

impl<C: Ctx> State<C> {
    pub fn new() -> Self {
        Self {
            tutorial: Tutorial { coin: false },
            outside: Outside {
                key_collected: false,
            },
            entryway: Entryway {
                coins: no_coins_collected(),
            },
            kitchen: Kitchen {
                coins: no_coins_collected(),
                weapon_collected: false,
            },
            living_room: LivingRoom {
                coins: no_coins_collected(),
                dad_attack_seconds: 0.0,
                has_escaped_dad: false,
            },
            murder_living_room: MurderLivingRoom {
                murderous_intent: false,
                dad_dead: false,
            },
            child_room: ChildRoom { child_stabs: 0 },
            ending_chosen: None,
            scene_changed: None,
        }
    }
}
