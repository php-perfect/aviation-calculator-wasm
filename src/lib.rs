use aviation_calculator::*;
use aviation_calculator::fk9::{calculate_start_distance, Engine as LibEngine};
use wasm_bindgen::prelude::*;

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub enum Engine {
    Rotax912Ul,
    Rotax912Uls,
}

#[wasm_bindgen]
pub struct TakeoffDistances {
    pub takeoff_run: f64,
    pub to_50_ft_height: f64,
}

#[wasm_bindgen]
pub fn fk9_takeoff_distance(engine: Engine, mass: f64, pressure_altitude: f64, temperature: f64, slope: f64, wet_surface: bool, soft_surface: bool, high_gras: bool) -> TakeoffDistances {
    utils::set_panic_hook();

    let distances: (f64, f64) = calculate_start_distance(match engine {
        Engine::Rotax912Ul => LibEngine::Rotax912Ul,
        Engine::Rotax912Uls => LibEngine::Rotax912Uls,
    }, mass, pressure_altitude, temperature, slope, wet_surface, soft_surface, high_gras).unwrap_throw();

    TakeoffDistances {
        takeoff_run: distances.0,
        to_50_ft_height: distances.1,
    }
}

#[wasm_bindgen]
pub fn ground_speed(course: f64, tas: f64, wd: f64, ws: f64) -> f64 {
    utils::set_panic_hook();
    calculate_ground_speed(course, tas, wd, ws)
}

#[cfg(test)]
mod tests {}
