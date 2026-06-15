use std::{sync::mpsc, thread, time::Instant};

use crate::simulation::{SimInteraction, Simulation, action::Action};
use hecs::World;
use macroquad::prelude::*;

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
// concurrency mechanism:
// the renderer/input thread sends actions to a queue consumed by the simulation
// the simulation processes these actions sequentually and accumulates an isolated viewport over time (a viewport includes filtered tick
// snapshots up to some granularity)
// it keeps writing that viewport to its current buffer and swaps whenever it does a write.
// the updated entries keep growing and growing until the simulation receives an empty buffer (meaning the renderer has processed it).
// the simulation then writes its current state to the new buffer, and clears its accumulator.
// we can allocate only a fixed amount of memory and reduce the "granularity" of viewports if it
// fills up by deleting staggered ticks. this should rarely ever happen.
// certain ticks (if they include something like an ephemeral message that should really be seen) can be
// marked as important and force a reallocation on overflow.
// basically quadruple buffering. three buffers for juggling with zero lock contention, and one
// buffer to preserve data.

#[macroquad::main("Cool game")]
async fn main() {
    let (tx, rx) = mpsc::channel::<SimInteraction>();

    // simulation
    thread::spawn(move || {
        let mut sim = Simulation::new();

        // track sim time for Null action dt, but use external dt when supplied
        let mut last = Instant::now();
        loop {
            let dt = last.elapsed().as_micros();
            if let Ok(interaction) = rx.try_recv() {
                // for now, just send it into the simulation and discard the response
                let _ = sim.exec(interaction);
            } else {
                // if there are no actions to execute, inject a null action
                let _ = sim.exec(SimInteraction {
                    action: Action::Null,
                    dt,
                });
            }
            last = Instant::now();
        }
    });

    loop {
        if is_key_down(KeyCode::Q) {
            return;
        }

        let dt = get_frame_time();

        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            // tx.send(SimInteraction {
            //     action: Action::StartMove {
            //         entity: (),
            //         dir: (),
            //     },
            // });
        }

        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            // tx.send(SimInteraction {
            //     action: Action::StartMove {
            //         entity: (),
            //         dir: (),
            //     },
            // });
        }

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            // tx.send(SimInteraction {
            //     action: Action::StartMove {
            //         entity: (),
            //         dir: (),
            //     },
            // });
        }

        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            // tx.send(SimInteraction {
            //     action: Action::StartMove {
            //         entity: (),
            //         dir: (),
            //     },
            // });
        }

        clear_background(BLACK);

        next_frame().await;
    }
}
