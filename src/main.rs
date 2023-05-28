mod actor;
mod audio;
mod globals;
mod helper;
mod scene;
mod scenes;
mod state;

use actor::Actor;
use audio::audio_thread;
use helper::draw_interact_prompt;
use scenes::child_room::ChildRoom;
use scenes::entryway::Entryway;
use scenes::kitchen::Kitchen;
use scenes::living_room::LivingRoom;
use scenes::murder_living_room::MurderLivingRoom;
use sdl2::render::WindowCanvas;
use std::collections::HashMap;
use std::time::Duration;

use globals::{GROUND_LEVEL, PIXEL_PER_DOT};
use scene::Scene;
use scenes::outside::Outside;
use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;
use sdl2::Sdl;
use state::State;

fn prepare_window(sdl_context: &Sdl) -> Result<Window, String> {
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem
        .window(
            "the adventures of lemonhead",
            PIXEL_PER_DOT as u32 * 10,
            PIXEL_PER_DOT as u32 * 10,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    Ok(window)
}

pub fn prepare_canvas(window: Window) -> Result<WindowCanvas, String> {
    window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())
}

pub fn run() -> Result<(), String> {
    let mut keys_down = HashMap::new();

    let sound_effect_sender = audio_thread();
    let music_effect_sender = audio_thread();

    let sdl_context = sdl2::init()?;
    let window = prepare_window(&sdl_context)?;
    let mut canvas = prepare_canvas(window)?;

    let mut animation_timer = 0.0;

    let outside = Outside::default();
    let entryway = Entryway::default();
    let kitchen = Kitchen::default();
    let living_room = LivingRoom::default();
    let murder_living_room = MurderLivingRoom::default();
    let child_room = ChildRoom::default();

    let mut scene: &dyn Scene = &living_room;
    let mut state = State::new(sound_effect_sender, music_effect_sender);
    let mut lemonhead = Actor::new("assets/lemonhead.png");
    lemonhead.set_position(
        f64::from(PIXEL_PER_DOT),
        (PIXEL_PER_DOT * GROUND_LEVEL).into(),
    );

    state.change_background_track("assets/outside.ogg");

    'game_loop: loop {
        let delta_time = 1.0 / 60.0;
        canvas.clear();
        scene.draw_scenery(&state, &mut canvas, animation_timer)?;
        let should_draw_interact = scene.should_draw_interact_popup(&state, lemonhead.x());
        if should_draw_interact {
            draw_interact_prompt(&mut canvas, &state, animation_timer)?;
        }

        lemonhead.idle();
        if *keys_down.get(&Keycode::A).unwrap_or(&false) && !state.ascended {
            lemonhead.offset_position(f64::from(PIXEL_PER_DOT) * -1.25, 0.0, delta_time);
            lemonhead.run_left();
        }
        if *keys_down.get(&Keycode::D).unwrap_or(&false) && !state.ascended {
            lemonhead.offset_position(f64::from(PIXEL_PER_DOT) * 1.25, 0.0, delta_time);
            lemonhead.run_right();
        }
        if *keys_down.get(&Keycode::Space).unwrap_or(&false) {
            scene.interact(&mut state, lemonhead.x());
            keys_down.insert(Keycode::Space, false);
        }

        if state.ascended {
            lemonhead.offset_position(0.0, -PIXEL_PER_DOT as f64 / 4.0, delta_time)
        }

        lemonhead.draw(&mut canvas, animation_timer);
        canvas.present();
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game_loop,
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
            Some((position, ref new_scene)) => {
                match new_scene {
                    scenes::Scenes::Entryway => scene = &entryway,
                    scenes::Scenes::Outside => scene = &outside,
                    scenes::Scenes::Kitchen => scene = &kitchen,
                    scenes::Scenes::LivingRoom => scene = &living_room,
                    scenes::Scenes::MurderLivingRoom => scene = &murder_living_room,
                    scenes::Scenes::ChildRoom => scene = &child_room,
                };
                lemonhead.set_position(
                    f64::from(PIXEL_PER_DOT) * position,
                    (PIXEL_PER_DOT * GROUND_LEVEL).into(),
                );
                state.scene_changed = None;
            }
        }

        if state.coin_7 && state.coin_8 {
            state.confronting_animation_timer += delta_time;
        }

        animation_timer += delta_time;
        animation_timer %= 1.0;
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn main() -> Result<(), String> {
    run()
}
