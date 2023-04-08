


extern crate piston_window;

use piston_window::*;
use rand::Rng;
use strum::EnumCount;
use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};

//use the particle.rs file
mod particle;
use particle::particle::*;


const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const SIM_SPEEDUP: f64 = 15.0;
const NUM_PARTICLES: usize = 250;
const PARTICLE_SIZE: f64 = 5.0;


fn step(particles: &mut Vec<Particle>, elapsed_time: f64) {
    // Update positions and velocities
    for i in 0..particles.len() {
        let mut total_force : particle::particle::Vector = [0.0, 0.0].into();
        for j in 0..particles.len() {
            if i == j {
                continue;
            }

            total_force += particles[i].calc_force(&particles[j]);
        }
        particles[i].velocity[0] = (1.0-particles[i].damping()) 
            * particles[i].velocity[0] + total_force[0] * elapsed_time;
        particles[i].velocity[1] = (1.0-particles[i].damping()) 
            * particles[i].velocity[1] + total_force[1] * elapsed_time;
        particles[i].position[0] += particles[i].velocity[0] * elapsed_time;
        particles[i].position[1] += particles[i].velocity[1] * elapsed_time;

        // Wrap particle position around window
        if particles[i].position[0] < 0.0 {
            particles[i].position[0] += WINDOW_WIDTH as f64;
        }
        if particles[i].position[0] >= WINDOW_WIDTH as f64{
            particles[i].position[0] -= WINDOW_WIDTH as f64;
        }
        if particles[i].position[1] < 0.0 {
            particles[i].position[1] += WINDOW_HEIGHT as f64;
        }
        if particles[i].position[1] >= WINDOW_HEIGHT as f64 {
            particles[i].position[1] -= WINDOW_HEIGHT as f64;
        }

    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let particles = Arc::new(Mutex::new((0..NUM_PARTICLES).map(|_| {
        Particle {
            particle_type: ParticleTypes::from_repr(rng.gen_range(0..ParticleTypes::COUNT)).unwrap(),
            position: [rng.gen_range(0..WINDOW_WIDTH).into(), rng.gen_range(0..WINDOW_HEIGHT).into()].into(),
            velocity: [0.0, 0.0].into(),
        }
    }).collect()));

    let mut window: PistonWindow = WindowSettings::new("My Window", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new());
    events.max_fps(60);

    window.set_lazy(false);
    window.set_max_fps(60);

    let sim_particles = particles.clone();
    let draw_particles = particles.clone();
    
    // Spawn a new thread for the simulation
    let _sim_handle = std::thread::spawn(move || {

        // Set up timer for fixed-rate updates
        let mut last_update_time = Instant::now();

        loop {
            // Check if it's time to update
            let now = Instant::now();
            let elapsed_time = (now - last_update_time).as_secs_f64();
            last_update_time = now;

            step(&mut sim_particles.lock().unwrap(), elapsed_time * SIM_SPEEDUP);

            // Sleep for a short time to avoid busy waiting
            std::thread::sleep(Duration::from_millis(8));
        }
    });
    
    while let Some(event) = events.next(&mut window) {
        window.draw_2d(&event, |context, graphics, _dev| {
            clear([0.02; 4], graphics);

            // Draw particles
            for particle in draw_particles.lock().unwrap().iter() {
                let color = particle.color();
                ellipse(color, [particle.position[0], particle.position[1], PARTICLE_SIZE, PARTICLE_SIZE], 
                    context.transform, graphics);
            }
        });
    }
}
