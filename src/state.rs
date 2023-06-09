use std::sync::mpsc::Sender;

use crate::audio::Configuration;
use crate::scenes::Scenes;

pub struct State<'a> {
    pub front_door_key_picked_up: bool,
    pub weapon_picked_up: bool,
    pub tutorial_coin: bool,
    pub coin_0: bool,
    pub coin_1: bool,
    pub coin_2: bool,
    pub coin_3: bool,
    pub coin_4: bool,
    pub coin_5: bool,
    pub coin_6: bool,
    pub coin_7: bool,
    pub coin_8: bool,
    pub confronted: bool,
    pub dad_dead: bool,
    pub child_dead: bool,
    pub child_stabs: u32,
    pub ascended: bool,
    pub escaped: bool,
    pub murderous_intent: bool,
    pub confronting_animation_timer: f64,
    pub scene_changed: Option<(f64, Scenes)>,
    sound_effect: Sender<Configuration>,
    music: &'a Sender<Configuration>,
}

impl<'a> State<'a> {
    pub fn new(sound_effect: Sender<Configuration>, music: &'a Sender<Configuration>) -> Self {
        Self {
            front_door_key_picked_up: false,
            weapon_picked_up: false,
            tutorial_coin: false,
            coin_0: false,
            coin_1: false,
            coin_2: false,
            coin_3: false,
            coin_4: false,
            coin_5: false,
            coin_6: false,
            coin_7: false,
            coin_8: false,
            ascended: false,
            confronted: false,
            dad_dead: false,
            murderous_intent: false,
            child_dead: false,
            child_stabs: 0,
            confronting_animation_timer: 0.0,
            scene_changed: None,
            sound_effect,
            music,
            escaped: false,
        }
    }

    pub fn send_audio(&self, path: &'static str) {
        if self.child_stabs >= 3 {
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
