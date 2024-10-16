use std::time::{Duration, Instant};
use std::sync::atomic::Ordering;

mod body;
mod partition;
mod quadtree;
mod renderer;
mod simulation;
mod utils;

use renderer::Renderer;
use simulation::Simulation;

fn main() {
    let threads = std::thread::available_parallelism().unwrap().get().max(3) - 2;
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .unwrap();

    let config = quarkstrom::Config {
        window_mode: quarkstrom::WindowMode::Windowed(900, 900),
    };

    let mut simulation = Simulation::new();
    let mut fps_timer = Instant::now();
    let mut frames = 0;
    let mut fps = 0.0;

    std::thread::spawn(move || {
        loop {
            if renderer::PAUSED.load(Ordering::Relaxed) {
                std::thread::yield_now();
            } else {
                simulation.step();
            }
            render(&mut simulation, fps);

            // Count frames and calculate FPS every second
            frames += 1;
            if fps_timer.elapsed() >= Duration::from_secs(1) {
                fps = frames as f64 / fps_timer.elapsed().as_secs_f64();
                fps_timer = Instant::now();  // Reset the timer
                frames = 0;  // Reset the frame counter
            }
        }
    });

    quarkstrom::run::<Renderer>(config);
}

fn render(simulation: &mut Simulation, fps: f64) {
    let mut lock = renderer::UPDATE_LOCK.lock();
    for body in renderer::SPAWN.lock().drain(..) {
        simulation.bodies.push(body);
    }
    {
        let mut lock = renderer::BODIES.lock();
        lock.clear();
        lock.extend_from_slice(&simulation.bodies);
    }
    {
        let mut lock = renderer::QUADTREE.lock();
        lock.clear();
        lock.extend_from_slice(&simulation.quadtree.nodes);
    }
    *lock |= true;

    // Pass FPS to the renderer for displaying
    renderer::set_fps(fps);
}
