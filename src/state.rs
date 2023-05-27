use std::sync::mpsc::Sender;

use crate::scenes::Scenes;

pub struct State {
    pub front_door_key_picked_up: bool,
    pub front_door_opened: bool,
    pub weapon_picked_up: bool,
    pub coin_0: bool,
    pub coin_1: bool,
    pub coin_2: bool,
    pub coin_3: bool,
    pub scene_changed: Option<(f64, Scenes)>,
    audio_sender: Sender<&'static str>,
}

impl State {
    pub fn new(audio_sender: Sender<&'static str>) -> Self {
        Self {
            front_door_key_picked_up: false,
            weapon_picked_up: false,
            front_door_opened: false,
            coin_0: false,
            coin_1: false,
            coin_2: false,
            coin_3: false,
            scene_changed: None,
            audio_sender,
        }
    }

    pub fn send_audio(&self, path: &'static str) {
        self.audio_sender.send(path).unwrap();
    }
}
