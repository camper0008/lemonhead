use std::sync::mpsc::Sender;

use crate::audio::Configuration;
use crate::scenes::Scenes;

pub fn all_coins_collected<const N: usize>(coins: &[bool; N]) -> bool {
    coins.iter().all(|v| *v)
}

pub struct Tutorial {
    pub coin: bool,
}

pub struct Outside {
    pub key_collected: bool,
}

pub struct Entryway {
    pub coins: [bool; 4],
}

pub struct Kitchen {
    pub coins: [bool; 3],
    pub weapon_collected: bool,
}

pub struct LivingRoom {
    pub coins: [bool; 2],
    pub dad_confrontation_progress: f64,
    pub confronted: bool,
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

pub struct State<'a> {
    pub tutorial: Tutorial,
    pub outside: Outside,
    pub entryway: Entryway,
    pub kitchen: Kitchen,
    pub living_room: LivingRoom,
    pub murder_living_room: MurderLivingRoom,
    pub child_room: ChildRoom,

    pub ascended: bool,
    pub escaped: bool,

    pub scene_changed: Option<(u8, Scenes)>,
    sound_effect: Sender<Configuration>,
    music: &'a Sender<Configuration>,
}

impl<'a> State<'a> {
    pub fn new(sound_effect: Sender<Configuration>, music: &'a Sender<Configuration>) -> Self {
        Self {
            tutorial: Tutorial { coin: false },
            outside: Outside {
                key_collected: false,
            },
            entryway: Entryway { coins: [false; 4] },
            kitchen: Kitchen {
                coins: [true; 3],
                weapon_collected: false,
            },
            living_room: LivingRoom {
                coins: [false; 2],
                dad_confrontation_progress: 0.0,
                confronted: false,
            },
            murder_living_room: MurderLivingRoom {
                murderous_intent: false,
                dad_dead: false,
            },
            child_room: ChildRoom { child_stabs: 0 },
            ascended: false,
            scene_changed: None,
            sound_effect,
            music,
            escaped: false,
        }
    }

    pub fn send_audio(&self, path: &'static str) {
        if self.child_room.child_stabs >= 3 {
            return;
        }
        self.sound_effect
            .send(Configuration::Play(1.0, path))
            .unwrap();
    }

    pub fn change_background_track(&self, path: &'static str) {
        self.music.send(Configuration::Play(0.5, path)).unwrap();
    }

    pub fn stop_background_track(&self) {
        self.music.send(Configuration::Stop).unwrap();
    }

    pub fn play_ascension_track(&self) {
        self.music
            .send(Configuration::Play(1.0, "assets/ascension.ogg"))
            .unwrap();
    }
}
