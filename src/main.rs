#[warn(clippy::unwrap_used)]
mod actor;
mod audio;
mod dead_ending;
mod game;
mod globals;
mod good_ending;
mod helper;
mod menu;
mod scene;
mod scenes;
mod state;
mod tileset;
mod ui_ctx;

use audio::{audio_thread, Configuration};
use dead_ending::dead_ending;
use game::{game, Action};
use good_ending::good_ending;
use menu::menu;
use sdl2::render::WindowCanvas;

use globals::PIXEL_PER_DOT;
use sdl2::image::InitFlag;
use sdl2::video::Window;
use sdl2::Sdl;

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
        .resizable()
        .maximized()
        .build()
        .map_err(|e| e.to_string())?;
    Ok(window)
}

fn prepare_canvas(window: Window) -> Result<WindowCanvas, String> {
    window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let window = prepare_window(&sdl_context)?;
    let mut canvas = prepare_canvas(window)?;

    let music_effect_sender = audio_thread();
    music_effect_sender
        .send(Configuration::Repeat(true))
        .map_err(|e| e.to_string())?;

    loop {
        menu(&sdl_context, &mut canvas, &music_effect_sender)?;
        let action = game(&sdl_context, &mut canvas, &music_effect_sender)?;
        match action {
            Action::Dead => dead_ending(&sdl_context, &mut canvas, &music_effect_sender)?,
            Action::GoodEnding => good_ending(&sdl_context, &mut canvas, &music_effect_sender)?,
            Action::Quit => break,
        };
    }

    Ok(())
}
