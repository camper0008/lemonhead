#![warn(clippy::unwrap_used)]
mod ctx;
mod game;
mod globals;
mod helper;
mod menu;
mod scenes;
mod sdl_rodio_ctx;
mod sprite;
mod state;

use game::game;
use menu::menu;
use sdl_rodio_ctx::SdlRodioCtx;

fn main() -> Result<(), String> {
    let mut ctx = SdlRodioCtx::new()?;
    if let menu::MenuResult::Quit = menu(&mut ctx)? {
        return Ok(());
    };
    let result = game(&mut ctx)?;
    match result {
        game::GameResult::GoodEnding => todo!(),
        game::GameResult::Dead => todo!(),
        game::GameResult::Quit => Ok(()),
    }
}
