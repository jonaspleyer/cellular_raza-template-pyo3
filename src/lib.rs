#![deny(missing_docs)]
//! This package is an example of how to construct python bindings with documentation with
//! `cellular_raza <https://cellular-raza.com/>`_.

use pyo3::prelude::*;

use cellular_raza::building_blocks::{BoundLennardJonesF32, CartesianCuboid, NewtonDamped2DF32};
use cellular_raza::concepts::{
    CalcError, CellAgent, Interaction, Mechanics, Position, RngError, Velocity,
};

use cellular_raza::core::backend::chili;

use nalgebra::Vector2;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};

/// Contains settings needed to specify the simulation
#[pyclass(get_all, set_all)]
pub struct SimulationSettings {
    /// Number of agents to initially put into the system
    pub n_agents: usize,
    /// Overall domain size
    pub domain_size: f32,
    /// Number of voxels to create subdivisions
    pub n_voxels: usize,
    /// Number of threads used
    pub n_threads: usize,
    /// Time increment used to solve the simulation
    pub dt: f32,
}

#[pymethods]
impl SimulationSettings {
    /// Creates a new :class:`SimulationSettings` class.
    #[new]
    fn new() -> Self {
        Self {
            n_agents: 200,
            domain_size: 30.0,
            n_voxels: 3,
            n_threads: 4,
            dt: 0.002,
        }
    }
}

/// A cellular agent which is used in our simulation.
#[pyclass]
#[derive(CellAgent, Clone, Deserialize, Serialize)]
pub struct Agent {
    /// Used to integrate position and velocity of our agent
    #[Mechanics]
    pub mechanics: NewtonDamped2DF32,
    /// Calculate interactions between agents
    #[Interaction]
    pub interaction: BoundLennardJonesF32,
}

/// Performs a complete numerical simulation of our system.
///
/// Args:
///     simulation_settings(SimulationSettings): The settings required to run the simulation
#[pyfunction]
pub fn run_simulation(
    simulation_settings: &SimulationSettings,
) -> Result<(), chili::SimulationError> {
    use rand::Rng;
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);

    // Agents setup
    let agent = Agent {
        mechanics: NewtonDamped2DF32 {
            pos: Vector2::from([0.0, 0.0]),
            vel: Vector2::from([0.0, 0.0]),
            damping_constant: 1.0,
            mass: 1.0,
        },
        interaction: BoundLennardJonesF32 {
            epsilon: 0.01,
            sigma: 1.0,
            bound: 0.1,
            cutoff: 1.0,
        },
    };

    let domain_size = simulation_settings.domain_size;
    let agents = (0..simulation_settings.n_agents).map(|_| {
        let mut new_agent = agent.clone();
        new_agent.set_pos(&Vector2::from([
            rng.random_range(0.0..domain_size),
            rng.random_range(0.0..domain_size),
        ]));
        new_agent
    });

    // Domain Setup
    let domain = CartesianCuboid::from_boundaries_and_n_voxels(
        [0.0; 2],
        [simulation_settings.domain_size; 2],
        [simulation_settings.n_voxels; 2],
    )?;

    // Storage Setup
    let storage_builder = cellular_raza::prelude::StorageBuilder::new().location("out");

    // Time Setup
    let t0: f32 = 0.0;
    let dt = simulation_settings.dt;
    let save_points = vec![5.0, 10.0, 15.0, 20.0];
    let time_stepper = cellular_raza::prelude::time::FixedStepsize::from_partial_save_points(
        t0,
        dt,
        save_points.clone(),
    )?;

    let settings = chili::Settings {
        n_threads: simulation_settings.n_threads.try_into().unwrap(),
        time: time_stepper,
        storage: storage_builder,
        show_progressbar: true,
    };

    chili::run_simulation!(
        domain: domain,
        agents: agents,
        settings: settings,
        aspects: [Mechanics, Interaction],
    )?;
    Ok(())
}

#[pymodule]
fn cellular_raza_template_pyo3_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SimulationSettings>()?;
    m.add_function(wrap_pyfunction!(run_simulation, m)?)?;
    Ok(())
}
