use std::{collections::HashMap, sync::mpsc, thread, time::Instant};

use crate::simulation::{SimInteraction, SimOutput, SimViewport, Simulation, action::Action};
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
// the simulation pushes outputs to a lock free ring buffer. the main thread reads from that buffer
// and pops from it. it associates responses to inputs using a local stack. it may choose to discard
// visual outputs.
//
// there needs to be a separate input thread, a networking thread, etc...

#[macroquad::main("Cool game")]
async fn main() {
    // sim input
    let (tx, rx) = mpsc::channel::<SimInteraction>();

    // sim output

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
