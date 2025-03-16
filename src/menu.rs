use std::f64::consts::PI;

use crate::{
    ctx::{Ctx, Music, Rgb},
    sprite::{Actor, ActorState, Tile},
};

pub enum MenuResult {
    Start,
    Quit,
}

pub fn menu<C: Ctx>(ctx: &mut C) -> Result<MenuResult, C::Error> {
    ctx.set_music(Music::Lemonhead)?;
    loop {
        ctx.pre_step()?;
        if ctx.key_down(crate::ctx::Key::Quit) {
            break Ok(MenuResult::Quit);
        } else if ctx.key_down(crate::ctx::Key::Interact) {
            break Ok(MenuResult::Start);
        }
        ctx.fill_background(Rgb(255, 255, 255))?;
        ctx.draw_sprite((0.0, 9.0), (10.0, 1.0), &Tile::Ground)?;

        let lemon_offset = ctx.seconds_elapsed().sin() * 6.5 + 4.5;
        let dad_offset = -ctx.seconds_elapsed().cos();
        let state = if dad_offset.is_sign_negative() {
            ActorState::Right
        } else {
            ActorState::Left
        };
        let use_alt = ctx.seconds_elapsed() % 0.2 > 0.10;
        let lemonhead = Actor::lemonhead_sprite(&state, use_alt);
        let dad = Actor::npc_sprite(&state, use_alt, Actor::Dad);
        ctx.draw_sprite((lemon_offset, 8.0), (1.0, 1.0), &lemonhead)?;
        ctx.draw_sprite((lemon_offset + dad_offset, 8.0), (1.0, 1.0), &dad)?;
        let offset = (ctx.seconds_elapsed() * PI * 2.0).sin() * 0.125;
        ctx.draw_sprite((1.0, 1.0 + offset), (8.0, 8.0), &Tile::Logo)?;
        ctx.post_step()?;
    }
}
