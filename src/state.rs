use std::sync::mpsc::Sender;

use crate::scenes::Scenes;

pub struct State {
    pub front_door_key_picked_up: bool,
    pub front_door_opened: bool,
    pub weapon_picked_up: bool,
    pub scene_changed: Option<(f64, Scenes)>,
    audio_sender: Sender<&'static str>,
}

impl State {
    pub fn new(audio_sender: Sender<&'static str>) -> Self {
        Self {
            front_door_key_picked_up: false,
            weapon_picked_up: false,
            front_door_opened: false,
            scene_changed: None,
            audio_sender,
        }
    }

    pub fn send_audio(&self, path: &'static str) {
        self.audio_sender.send(path).unwrap();
    }
}
