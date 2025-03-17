#![warn(clippy::unwrap_used)]
mod ctx;
mod dead_ending;
mod game;
mod globals;
mod good_ending;
mod helper;
mod menu;
mod scenes;
mod sdl_rodio_ctx;
mod sprite;
mod state;

use dead_ending::dead_ending;
use game::game;
use good_ending::good_ending;
use menu::menu;
use sdl_rodio_ctx::SdlRodioCtx;

fn main() -> Result<(), String> {
    let mut ctx = SdlRodioCtx::new()?;
    if let menu::MenuResult::Quit = menu(&mut ctx)? {
        return Ok(());
    };
    let result = game(&mut ctx)?;
    match result {
        game::GameResult::GoodEnding => good_ending(&mut ctx),
        game::GameResult::Dead => dead_ending(&mut ctx),
        game::GameResult::Quit => Ok(()),
    }
}
