use crate::{
    ctx::{Ctx, Key},
    globals::GROUND_LEVEL,
    scenes::{Scene, Scenes},
    sprite::{self, ActorState, Text},
    state::{EndingChosen, State},
};

pub enum GameResult {
    GoodEnding,
    Dead,
    Quit,
}

struct Lemonhead {
    x: f64,
    y: f64,
    state: ActorState,
}

fn draw_interact_prompt<C: Ctx>(ctx: &mut C, state: &State<C>) {
    let offset = (ctx.seconds_elapsed() * std::f64::consts::PI * 2.0).sin() * 0.05;

    let text = if !state.living_room.has_escaped_dad {
        Text::Space
    } else if !state.kitchen.weapon_collected {
        Text::SelfDefense
    } else if !state.murder_living_room.dad_dead {
        Text::NoWitnesses
    } else if !state.child_room.child_dead() {
        Text::OneLeft
    } else if !state.child_room.child_stabs < 3 {
        Text::More
    } else {
        Text::Ascend
    };

    let centered = (10.0 - text.width()) / 2.0;

    ctx.draw_sprite((centered, 9.0 + offset), (text.width(), 1.0), &text);
}

pub fn game<C: Ctx>(ctx: &mut C) -> Result<GameResult, C::Error> {
    let mut scene = Scenes::Tutorial;
    let mut state = State::new();
    let mut lemonhead = Lemonhead {
        x: 1.0,
        y: GROUND_LEVEL,
        state: ActorState::Idle,
    };
    ctx.set_music(crate::ctx::Music::Outside)?;
    let mut elapsed_last_iter = ctx.seconds_elapsed();
    loop {
        ctx.setup()?;
        if ctx.key_down(Key::Quit) {
            break Ok(GameResult::Quit);
        }
        scene.draw(ctx, &state);
        if scene.should_draw_interact_popup(&state, lemonhead.x) {
            draw_interact_prompt(ctx, &state);
        }

        lemonhead.state = ActorState::Idle;

        let delta_time = ctx.seconds_elapsed() - elapsed_last_iter;
        elapsed_last_iter = ctx.seconds_elapsed();
        if ctx.key_down(Key::Left) && state.ending_chosen.is_none() && lemonhead.x > 0.0 {
            lemonhead.x -= 1.25 * delta_time;
            lemonhead.state = ActorState::Left;
        }

        if ctx.key_down(Key::Right) && state.ending_chosen.is_none() && lemonhead.x < 9.0 {
            lemonhead.x += 1.25 * delta_time;
            lemonhead.state = ActorState::Right;
        }

        if ctx.key_down(Key::Interact) {
            scene.interact(ctx, &mut state, lemonhead.x)?;
        }

        if let Some(ref ending) = state.ending_chosen {
            match ending {
                EndingChosen::Ascended => {
                    lemonhead.y -= 0.25 * delta_time;
                }
                EndingChosen::Escaped => {
                    lemonhead.state = ActorState::Left;
                    lemonhead.x -= 0.5 * delta_time;
                    if lemonhead.x < -1.0 {
                        break Ok(GameResult::GoodEnding);
                    }
                }
            }
        }

        let use_alt = ctx.seconds_elapsed() % 0.5 > 0.25;
        let lemon_sprite = sprite::Actor::lemonhead_sprite(&lemonhead.state, use_alt);
        ctx.draw_sprite((lemonhead.x, lemonhead.y), (1.0, 1.0), &lemon_sprite);

        match state.scene_changed {
            None => (),
            Some((position, new_scene)) => {
                scene = new_scene;
                lemonhead.x = position;
                state.scene_changed = None;
            }
        }

        if state.living_room.all_coins_collected() && !state.living_room.has_escaped_dad {
            state.living_room.dad_attack_seconds += delta_time;
            let dad_position = 13.65 - (state.living_room.dad_attack_seconds * 2.0);
            if dad_position <= lemonhead.x {
                break Ok(GameResult::Dead);
            }
        }
        ctx.finish()?;
    }
}
