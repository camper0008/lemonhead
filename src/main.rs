#![warn(clippy::unwrap_used)]
mod audio;
mod ctx;
mod globals;
mod logic;
mod menu;
mod sdl_ctx;
mod sprite;
mod state;

use std::time::Duration;

use audio::{audio_thread, Configuration};
use ctx::{Ctx, Key};
use menu::menu;
use sdl_ctx::SdlCtx;

fn main() -> Result<(), String> {
    let music_effect_sender = audio_thread();
    music_effect_sender
        .send(Configuration::Repeat(true))
        .map_err(|e| e.to_string())?;

    let mut ctx = SdlCtx::new()?;
    let mut seconds_elapsed = 0.0;
    loop {
        ctx.pre_step()?;
        if ctx.key_down(Key::Quit) {
            break;
        }
        menu(&mut ctx, seconds_elapsed, &music_effect_sender)?;
        // game_step(&sdl_context, &mut canvas, &music_effect_sender)?;
        // match action {
        //     Action::Dead => dead_ending(&sdl_context, &mut canvas, &music_effect_sender)?,
        //     Action::GoodEnding => good_ending(&sdl_context, &mut canvas, &music_effect_sender)?,
        //     Action::Quit => break,
        // };
        ctx.post_step();
        seconds_elapsed += 1.0 / 60.0;
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
