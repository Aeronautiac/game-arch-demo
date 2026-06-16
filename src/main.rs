use std::{process::exit, sync::atomic::AtomicU64, thread, time::Instant};

use crate::simulation::{
    SimInteraction, SimView, Simulation,
    action::{Action, ActionResponse, ActionResult},
    ecs::movement::{MovementDir, MovementIntent},
};
use crossbeam::channel::unbounded;
use macroquad::prelude::*;
use triple_buffer::triple_buffer;

mod common;
mod simulation;

// rather than having the truth tied to fps, the truth is simply derived from the timeline. this
// is a very simple and powerful shift in thinking, but doesnt change much internally. most game
// code will look the exact same. the difference is that it runs in what is essentially a virtual
// machine with a game specific instruction set rather than a standard game loop which is tightly
// coupled to real world time.
//
// not everything must run within the simulation. for instance, ai can be treated the same as any
// other player with the difference being that the decisions ai makes are computed on separate threads.
//
// world generation (if applicable) can be handled externally, and "chunks" can be injected through
// sim interactions.
//
// rendering is obviously handled externally as well.
//
// netcode? have players run their own client, and a central server or similar. send action
// sequences from a unified checkpoint. predict things on the client initially, but the server will
// eventually send back the truth and the client must correct itself.
//
// there is no need for the server to send heavy snapshots. rather, it can send back the "safe"
// point and tell the client to append the new "correct" timeline after that point. clients can
// store their own snapshots.
//
// any decimal math in the simulation layer must use fixed point numbers.
//
// the simulation pushes outputs to a lock free ring buffer. the main thread reads from that buffer
// and pops from it. it associates responses to inputs using a local stack. it may choose to discard
// visual outputs.
//
// there needs to be a separate input thread, a networking thread, etc...

// quick prototype
#[macroquad::main("Cool game")]
async fn main() {
    // sim input
    let (actions_in, actions_out) = unbounded::<SimInteraction>();

    // sim output
    let (mut views_in, mut views_out) = triple_buffer(&SimView {
        x: 0.into(),
        y: 0.into(),
        tick: 0,
    });
    let mut last_viewed_tick: AtomicU64 = AtomicU64::new(0);
    let (results_in, results_out) = unbounded::<ActionResult>();

    // simulation
    thread::spawn(move || {
        let mut sim = Simulation::new();

        // track sim time for Null action dt, but use external dt when supplied
        let mut last = Instant::now();
        loop {
            let dt = last.elapsed().as_nanos();
            last = Instant::now();
            if let Ok(interaction) = actions_out.try_recv() {
                // for now, just send it into the simulation and discard the response
                let out = sim.exec(interaction);
                results_in.send(out.action_result).unwrap();
                views_in.write(out.view);
            } else {
                std::hint::spin_loop();
                if dt == 0 {
                    continue;
                }
                // if there are no actions to execute, inject a null action
                // a null action still triggers the simulation's adaptive tick loop (deterministic
                // regardless of dt and number of inputs)
                let out = sim.exec(SimInteraction {
                    action: Action::Null,
                    dt,
                });
                views_in.write(out.view);
            }
        }
    });

    // game initialization
    actions_in
        .send(SimInteraction {
            action: Action::CreateShip,
            dt: 0,
        })
        .unwrap();
    let create_ship_result = results_out.recv().unwrap();
    let action_response = create_ship_result.unwrap();
    let ActionResponse::Entity(player_id) = action_response else {
        unreachable!()
    };

    // begin rendering and input loop
    // later separate them
    let mut last = Instant::now();
    let mut px: f64;
    let mut py: f64;
    let mut last_intent = MovementIntent::EMPTY;
    loop {
        let dt = last.elapsed().as_nanos();
        if is_key_down(KeyCode::Q) {
            exit(0);
        }

        // TODO: update movement intent and send
        let mut move_intent = MovementIntent::EMPTY;
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            move_intent |= MovementDir::Right;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            move_intent |= MovementDir::Left;
        }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            move_intent |= MovementDir::Up;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            move_intent |= MovementDir::Down;
        }
        if last_intent != move_intent {
            actions_in
                .send(SimInteraction {
                    action: Action::SetMovementIntent {
                        target: player_id,
                        intent: move_intent,
                    },
                    dt,
                })
                .unwrap();
        }
        last_intent = move_intent;

        let view = views_out.read();
        px = view.x.to_num::<f64>();
        py = view.y.to_num::<f64>();

        clear_background(BLACK);

        draw_circle(px as f32, py as f32, 10.0, BLUE);

        last = Instant::now();
        next_frame().await;
    }
}
