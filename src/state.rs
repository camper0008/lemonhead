use std::sync::mpsc::Sender;

use crate::audio::AudioConfiguration;
use crate::scenes::Scenes;

pub struct State {
    pub front_door_key_picked_up: bool,
    pub front_door_opened: bool,
    pub weapon_picked_up: bool,
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
    pub bad_guy_dead: bool,
    pub child_dead: bool,
    pub confronting_animation_timer: f64,
    pub scene_changed: Option<(f64, Scenes)>,
    sound_effect: Sender<AudioConfiguration>,
    music: Sender<AudioConfiguration>,
}

impl State {
    pub fn new(
        sound_effect: Sender<AudioConfiguration>,
        music: Sender<AudioConfiguration>,
    ) -> Self {
        Self {
            front_door_key_picked_up: false,
            weapon_picked_up: false,
            front_door_opened: false,
            coin_0: false,
            coin_1: false,
            coin_2: false,
            coin_3: false,
            coin_4: false,
            coin_5: false,
            coin_6: false,
            coin_7: false,
            coin_8: false,
            confronted: false,
            bad_guy_dead: false,
            child_dead: false,
            confronting_animation_timer: 0.0,
            scene_changed: None,
            sound_effect,
            music,
        }
    }

    pub fn send_audio(&self, path: &'static str) {
        self.sound_effect
            .send(AudioConfiguration::Play(1.0, path))
            .unwrap();
    }

    pub fn change_background_track(&self, path: &'static str) {
        self.music.send(AudioConfiguration::Stop).unwrap();

        for _ in 0..500 {
            self.music
                .send(AudioConfiguration::Play(0.5, path))
                .unwrap();
        }
    }
}
