
extern crate nalgebra;

use nalgebra::SVector;
use nalgebra::SMatrix;

use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter, FromRepr};

use crate::particle::constants::*;

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
        [0.3, -1.3, 0.5, 0.5],
        [-1.1, 0.2, 1.1, -0.1],
        [0.5, 1.0, 0., 1.0],
        [0.5, -0.2, 1.1, -1.0],
    ];

    pub fn distance(&self, to: &Particle) -> Vector{
        self.position - to.position
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
            ParticleTypes::Red => 0.01,
            ParticleTypes::Green => 0.02,
            ParticleTypes::Blue => 0.03,
            ParticleTypes::YELLOW => 0.04,
        }
    }
    
    fn force_coeff(&self, to: &ParticleTypes) -> f64 {
        Self::FORCE_MATRIX[self.particle_type as usize][*to as usize]
    }

    pub fn calc_force(&self, to: &Particle) -> Vector {
        let diff = self.distance(to);
        let distance = diff.magnitude().max(MIN_DISTANCE);
        let direction = diff / distance;
        let force = self.force_coeff(&to.particle_type) / distance;
        direction * force
    }

}
