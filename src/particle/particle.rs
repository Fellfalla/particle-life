
extern crate nalgebra;

use nalgebra::SVector;
use nalgebra::SMatrix;

use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter, FromRepr};

use crate::particle::constants::*;
use crate::particle::math;

pub type Vector = SVector<f64, DIMENSIONS>;

#[derive(Debug, EnumCountMacro, EnumIter, FromRepr, Copy, Clone)]
pub enum ParticleTypes {
    Red,
    Green,
    Blue,
    YELLOW,
}

pub struct Particle {
    pub particle_type: ParticleTypes,
    pub position: Vector,
    pub velocity: Vector,
}


impl Particle {


    const FORCE_MATRIX: [[f64; ParticleTypes::COUNT]; ParticleTypes::COUNT] = [
        [1.0, 0.3, 0.0, 0.0],
        [0.0, 0.4, 0.3, 0.0],
        [0.0, -0.5, 0.7, 0.3],
        [0.3, 0.0, 0.0, 1.0],
    ];

    pub fn distance(&self, to: &Particle) -> Vector{
        to.position - self.position
    }

    pub fn color (&self) -> [f32; 4] {
        match self.particle_type {
            ParticleTypes::Red => [1.0, 0.0, 0.0, 1.0],
            ParticleTypes::Green => [0.0, 1.0, 0.0, 1.0],
            ParticleTypes::Blue => [0.0, 0.0, 1.0, 1.0],
            ParticleTypes::YELLOW => [0.8, 0.8, 0.2, 1.0],
        }
    }

    pub fn damping(&self) -> f64 {
        match self.particle_type {
            ParticleTypes::Red => 0.5,
            ParticleTypes::Green => 0.5,
            ParticleTypes::Blue => 0.5,
            ParticleTypes::YELLOW => 0.5,
        }
    }
    
    fn force_coeff(&self, to: &ParticleTypes) -> f64 {
        Self::FORCE_MATRIX[self.particle_type as usize][*to as usize]
    }

    pub fn calc_force(&self, to: &Particle) -> Vector {
        let diff = self.distance(to);
        let distance = diff.magnitude().max(MIN_DISTANCE);
        let direction = diff / distance;

        // Calculate the repulsion force
        let repulsion_force = math::clipped_interpolation(
            distance, 
            R_REPULSION, 
            0., 
            0.,
            MAX_REPULSION_FORCE, 
        );

        let particle_force = math::clipped_interpolation(
            distance, 
            R_INTERACTION, 
            0., 
            0.,
            self.force_coeff(&to.particle_type), 
        );

        direction * (repulsion_force + particle_force)
    }

    // pub fn step(&mut self, particles: &Vec<Particle>, dt: f64) {
    //     let mut total_force: Vector = [0.0, 0.0].into();
    //     for j in 0..particles.len() {
    //         total_force += self.calc_force(&particles[j]);
    //     }
        
    //     self.velocity = (1.0-self.damping()) * self.velocity + total_force * dt;

    //     self.position += self.velocity * dt;

    // }

}
