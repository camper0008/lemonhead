use std::{f64::consts::PI, sync::mpsc::Sender};

use crate::{
    audio::Configuration,
    ctx::{Ctx, Rgb},
    sprite::{Actor, ActorState, Tile},
};

pub fn menu<C: Ctx>(
    ctx: &mut C,
    seconds_elapsed: f64,
    music_sender: &Sender<Configuration>,
) -> Result<(), C::Error> {
    // music_sender
    //     .send(Configuration::Play(1.0, "assets/lemonhead.ogg"))
    //     .map_err(|e| e.to_string())?;

    ctx.fill_background(Rgb(255, 255, 255))?;
    ctx.draw_sprite((0.0, 9.0), (10.0, 1.0), Tile::Ground)?;

    let lemon;
    let dad;

    let lemon_offset = seconds_elapsed.sin() * 6.5 + 4.5;
    let dad_offset = -seconds_elapsed.cos();
    if dad_offset.is_sign_negative() {
        lemon = ActorState::Right;
        dad = ActorState::Right;
    } else {
        lemon = ActorState::Left;
        dad = ActorState::Left;
    }
    let lemonhead = Actor::animated_lemonhead(lemon, seconds_elapsed);
    let dad = Actor::animated_npc(Actor::Dad, dad, seconds_elapsed);
    ctx.draw_sprite((lemon_offset, 8.0), (1.0, 1.0), lemonhead)?;
    ctx.draw_sprite((lemon_offset + dad_offset, 8.0), (1.0, 1.0), dad)?;

    let offset = (seconds_elapsed * PI * 2.0).sin() * 0.125;
    ctx.draw_sprite((1.0, 1.0 + offset), (8.0, 8.0), Tile::Logo)?;

    Ok(())
}
