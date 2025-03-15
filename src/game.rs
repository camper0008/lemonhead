use std::{collections::HashMap, sync::mpsc::Sender, time::Duration};

use sdl2::{event::Event, keyboard::Keycode, render::WindowCanvas, Sdl};

use crate::{
    actor::Actor,
    audio::{audio_thread, Configuration},
    globals::{GROUND_LEVEL, PIXEL_PER_DOT},
    helper::draw_interact_prompt,
    scenes::Scenes,
    state::{all_coins_collected, State},
};

pub enum Action {
    GoodEnding,
    Dead,
    Quit,
}

pub fn game(
    sdl_context: &Sdl,
    canvas: &mut WindowCanvas,
    music_sender: &Sender<Configuration>,
) -> Result<Action, String> {
    let mut keys_down = HashMap::new();

    let sound_effect_sender = audio_thread();

    let mut animation_timer = 0.0;
    let mut escape_timer = 0.0;

    let mut scene = Scenes::Tutorial;
    let mut state = State::new(sound_effect_sender, music_sender);
    let mut lemonhead = Actor::new("assets/lemonhead.png");
    lemonhead.set_position(PIXEL_PER_DOT, PIXEL_PER_DOT * GROUND_LEVEL);

    state.change_background_track("assets/outside.ogg");

    let action = 'game_loop: loop {
        let delta_time = 1.0 / 60.0;
        canvas.clear();
        scene.inner().draw(&state, canvas, animation_timer)?;
        if scene
            .inner()
            .should_draw_interact_popup(&state, lemonhead.x())
        {
            draw_interact_prompt(canvas, &state, animation_timer)?;
        }

        lemonhead.idle();
        if *keys_down.get(&Keycode::A).unwrap_or(&false)
            && !(state.ascended || state.escaped)
            && lemonhead.x() > 0.0
        {
            lemonhead.offset_position(PIXEL_PER_DOT * -1.25, 0.0, delta_time);
            lemonhead.run_left();
        }
        if *keys_down.get(&Keycode::D).unwrap_or(&false)
            && !(state.ascended || state.escaped)
            && lemonhead.x() < (9.0 * PIXEL_PER_DOT)
        {
            lemonhead.offset_position(PIXEL_PER_DOT * 1.25, 0.0, delta_time);
            lemonhead.run_right();
        }
        if *keys_down.get(&Keycode::Space).unwrap_or(&false) {
            scene.inner().interact(&mut state, lemonhead.x());
            keys_down.insert(Keycode::Space, false);
        }

        if state.ascended {
            lemonhead.offset_position(0.0, -PIXEL_PER_DOT / 4.0, delta_time);
        }

        if state.escaped {
            lemonhead.run_left();
            lemonhead.offset_position(-PIXEL_PER_DOT / 2.0, 0.0, delta_time);
            escape_timer += delta_time;
            if escape_timer > 5.0 {
                break Action::GoodEnding;
            }
        }

        lemonhead.draw(canvas, animation_timer)?;
        canvas.present();
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game_loop Action::Quit,
                Event::KeyDown {
                    keycode: Some(k @ (Keycode::A | Keycode::D | Keycode::Space)),
                    ..
                } => {
                    keys_down.insert(k, true);
                }
                Event::KeyUp {
                    keycode: Some(k @ (Keycode::A | Keycode::D | Keycode::Space)),
                    ..
                } => {
                    keys_down.insert(k, false);
                }
                _ => {}
            }
        }
        match state.scene_changed {
            None => (),
            Some((position, new_scene)) => {
                scene = new_scene;
                lemonhead.set_position(
                    PIXEL_PER_DOT * position as f64,
                    PIXEL_PER_DOT * GROUND_LEVEL,
                );
                state.scene_changed = None;
            }
        }

        if all_coins_collected(&state.living_room.coins) && !state.living_room.confronted {
            state.living_room.dad_confrontation_progress += delta_time;
            let dad_position = PIXEL_PER_DOT * 13.65
                - (state.living_room.dad_confrontation_progress * 2.0 * PIXEL_PER_DOT);
            if dad_position <= lemonhead.x() {
                break Action::Dead;
            }
        }

        animation_timer += delta_time;
        animation_timer %= 1.0;
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    };

    Ok(action)
}
