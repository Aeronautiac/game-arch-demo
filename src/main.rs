use std::{
    collections::VecDeque,
    process::exit,
    sync::{Arc, atomic::AtomicU64},
    thread,
    time::Instant,
};

use crate::simulation::{SimInput, Simulation};
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
    // let (mut views_in, mut views_out) = triple_buffer(&SimView {
    //     tick_views: VecDeque::new(),
    // });
    let (results_in, results_out) = unbounded();

    // simulation
    let last_viewed_sim = last_viewed_tick.clone();
    thread::spawn(move || {
        let mut sim = Simulation::new();

        // track sim time for Null action dt, but use external dt when supplied
        let mut accumulated_view = SimView {
            tick_views: VecDeque::new(),
        };
        let mut last = Instant::now();
        loop {
            let dt = last.elapsed().as_nanos();
            last = Instant::now();

            let view = if let Ok(interaction) = actions_out.try_recv() {
                // for now, just send it into the simulation and discard the response
                let out = sim.exec(interaction);
                results_in.send(out.action_result).unwrap();
                out.view
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
                out.view
            };

            accumulated_view.merge_with(view);
            accumulated_view.prune_to(last_viewed_sim.load(std::sync::atomic::Ordering::SeqCst));

            views_in.write(accumulated_view.clone());
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

        let mut x = 0.0;
        let mut y = 0.0;
        let mut rot = 0.0;
        for tick_view in &view.tick_views {
            for vp in &tick_view.viewports {
                for entity in &vp.entities {
                    let prot = entity.pos.rotation;
                    let px = entity.pos.position.x;
                    let py = entity.pos.position.y;

                    x = px;
                    y = py;
                    rot = prot;
                }
            }
            last_viewed_tick.store(tick_view.tick, std::sync::atomic::Ordering::SeqCst);
        }

        clear_background(BLACK);
        draw_poly(x, y, 3, 35.0, rot, WHITE);

        next_frame().await;
    }
}
