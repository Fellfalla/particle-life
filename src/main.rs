


extern crate piston_window;

use piston_window::*;
use rand::Rng;
use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const DIMENSIONS: usize = 2;
const MIN_DISTANCE: f64 = 1.0;
const SIM_SPEEDUP: f64 = 15.0;
const NUM_PARTICLE_TYPES: usize = 4;
const NUM_PARTICLES: usize = 300;
const PARTICLE_SIZE: f64 = 5.0;

struct ParticleType {
    type_id: usize,
    color: [f32; 4],
    damping: f64,
}

struct Particle<'a> {
    particle_type: &'a ParticleType,
    position: [f64; DIMENSIONS],
    velocity: [f64; DIMENSIONS],
}

const PARTICLE_TYPES: [ParticleType; NUM_PARTICLE_TYPES] = [
    ParticleType {
        type_id: 0,
        damping: 0.01,
        color: [1.0, 0.0, 0.0, 1.0], // Red
    },
    ParticleType {
        type_id: 1,
        damping: 0.02,
        color: [0.0, 1.0, 0.0, 1.0], // Green
    },
    ParticleType {
        type_id: 2,
        damping: 0.03,
        color: [0.0, 0.0, 1.0, 1.0], // Green
    },
    ParticleType {
        type_id: 3,
        damping: 0.04,
        color: [0.7, 0.2, 0.2, 1.0], // Green
    },
];

const FORCE_MATRIX: [[f64; NUM_PARTICLE_TYPES]; NUM_PARTICLE_TYPES] = [
    [0.3, -1.3, 0.5, 0.5], // Force between type 1 and 1, and between type 1 and 2
    [-1.1, 0.2, 1.1, -0.1], // Force between type 2 and 1, and between type 2 and 2
    [0.5, 1.0, 0., 1.0], // Force between type 2 and 1, and between type 2 and 2
    [0.5, -0.2, 1.1, -1.0], // Force between type 2 and 1, and between type 2 and 2
];


fn difference(from: &Particle, to: &Particle) -> [f64; DIMENSIONS] {
    return [
        to.position[0]-from.position[0], 
        to.position[1]-from.position[1]
    ];
}

fn norm(vector: &[f64; DIMENSIONS]) -> f64 {
    return (vector[0].powi(2) + vector[1].powi(2)).sqrt();
}

fn step(particles: &mut Vec<Particle>, elapsed_time: f64) {
    // Update positions and velocities
    for i in 0..particles.len() {
        let mut total_force = [0.0; DIMENSIONS];
        for j in 0..particles.len() {
            if i == j {
                continue;
            }

            let force = FORCE_MATRIX[particles[i].particle_type.type_id][particles[j].particle_type.type_id];
            let diff = difference(&particles[i], &particles[j]);
            let distance = norm(&diff).max(MIN_DISTANCE);
            let direction = [
                diff[0] / distance,
                diff[1] / distance
            ];
            let force_vector = [force / distance * direction[0], force / distance * direction[1]];
            total_force[0] += force_vector[0];
            total_force[1] += force_vector[1];
        }
        particles[i].velocity[0] = (1.0-particles[i].particle_type.damping) 
            * particles[i].velocity[0] + total_force[0] * elapsed_time;
        particles[i].velocity[1] = (1.0-particles[i].particle_type.damping) 
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
            particle_type: &PARTICLE_TYPES[rng.gen_range(0..NUM_PARTICLE_TYPES)],
            position: [rng.gen_range(0..WINDOW_WIDTH).into(), rng.gen_range(0..WINDOW_HEIGHT).into()],
            velocity: [0.0, 0.0],
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
            clear([1.0; 4], graphics);

            // Draw particles
            for particle in draw_particles.lock().unwrap().iter() {
                let color = particle.particle_type.color;
                ellipse(color, [particle.position[0], particle.position[1], PARTICLE_SIZE, PARTICLE_SIZE], 
                    context.transform, graphics);
            }
        });
    }
}
