use std::{
    process::exit,
    sync::{Arc, atomic::AtomicU64},
    thread,
    time::Instant,
};

use crate::simulation::{InputPayload, SimInput, Simulation};
use crossbeam::channel::unbounded;
use macroquad::prelude::*;
use triple_buffer::triple_buffer;

mod common;
mod simulation;

// quick prototype
#[macroquad::main("combat-demo-rs")]
async fn main() {
    // sim input
    let (actions_in, actions_out) = unbounded::<SimInput>();

    // sim output
    let last_viewed_tick: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
    let (mut views_in, mut views_out) = triple_buffer(&Vec::new());
    let (results_in, results_out) = unbounded();

    // simulation
    let last_viewed_sim = last_viewed_tick.clone();
    thread::spawn(move || {
        let mut sim = Simulation::new();

        let mut last = Instant::now();
        loop {
            let dt = last.elapsed().as_nanos() as u64;
            last = Instant::now();

            if let Ok(mut interaction) = actions_out.try_recv() {
                interaction.dt = dt;
                let out = sim.exec(interaction);
                results_in.send(out).unwrap();
            } else {
                if dt == 0 {
                    continue;
                }
                // if there are no actions to execute, inject a null action
                // a null action still triggers the simulation's adaptive tick loop (deterministic
                // regardless of dt and number of inputs)
                let out = sim.exec(SimInput {
                    payload: InputPayload::Null,
                    dt,
                });
            };

            sim.lossy_view_buf
                .discard_to(last_viewed_sim.load(std::sync::atomic::Ordering::Relaxed));
            let input_buf = views_in.input_buffer_mut();
            input_buf.clear();
            input_buf.extend(sim.lossy_view_buf.data.iter().cloned());
            views_in.publish();
        }
    });

    // game initialization
    // actions_in
    //     .send(SimInteraction {
    //         action: Action::CreateShip,
    //         dt: 0,
    //     })
    //     .unwrap();
    // let create_ship_result = results_out.recv().unwrap();
    // let action_response = create_ship_result.unwrap();
    // let ActionResponse::Entity(player_id) = action_response else {
    //     unreachable!()
    // };

    // begin rendering and input loop
    // later separate them
    let mut last = Instant::now();
    loop {
        if is_key_down(KeyCode::Q) {
            exit(0);
        }

        let view = views_out.read();

        clear_background(BLACK);
        // draw_poly(x, y, 3, 35.0, rot, WHITE);

        next_frame().await;
    }
}
