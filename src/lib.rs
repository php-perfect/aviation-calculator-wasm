use aviation_calculator::fk9::{calculate_takeoff_distance, Engine as LibEngine, GrassSurface, SurfaceCondition as LibSurfaceCondition, TakeoffCalculationError, TakeoffResult};
use aviation_calculator::meteorology::*;
use aviation_calculator::utils::*;
use serde_json::json;
use wasm_bindgen::prelude::*;

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Engine {
    Rotax912Ul,
    Rotax912Uls,
}

impl Into<LibEngine> for Engine {
    fn into(self) -> LibEngine {
        match self {
            Engine::Rotax912Ul => LibEngine::Rotax912Ul,
            Engine::Rotax912Uls => LibEngine::Rotax912Uls,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct TakeoffDistances {
    pub takeoff_run: f64,
    pub to_50_ft_height: f64,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum SurfaceCondition {
    Inconspicuous,
    Slush,
    Snow,
    PowderSnow,
}

impl Into<LibSurfaceCondition> for SurfaceCondition {
    fn into(self) -> LibSurfaceCondition {
        match self {
            SurfaceCondition::Inconspicuous => LibSurfaceCondition::Inconspicuous,
            SurfaceCondition::Slush => LibSurfaceCondition::Slush,
            SurfaceCondition::Snow => LibSurfaceCondition::Snow,
            SurfaceCondition::PowderSnow => LibSurfaceCondition::PowderSnow,
        }
    }
}

#[wasm_bindgen]
pub fn fk9_takeoff_distance(engine: Engine, mass: f64, pressure_altitude: f64, temperature: f64, slope: f64, gras: bool, wet: bool, soft_ground: bool, damaged_turf: bool, high_grass: bool, surface_condition: SurfaceCondition) -> Result<TakeoffDistances, JsError> {
    utils::set_panic_hook();

    let gras_surface: Option<GrassSurface> = if gras {
        Some(GrassSurface { wet, soft_ground, damaged_turf, high_grass })
    } else {
        None
    };

    let result = calculate_takeoff_distance(engine.into(), mass, pressure_altitude, temperature, slope, gras_surface, surface_condition.into());

    if result.is_err() {
        return Err(convert_error(result));
    }

    let (takeoff_run, to_50_ft_height): (f64, f64) = result.unwrap();

    Ok(TakeoffDistances {
        takeoff_run,
        to_50_ft_height,
    })
}

fn convert_error(result: TakeoffResult) -> JsError {
    let error: TakeoffCalculationError = result.unwrap_err();
    let (input, kind, message, value, threshold): (String, String, String, f64, f64) = match error {
        TakeoffCalculationError::MassTooLow { min, mass } => ("mass".to_string(), "MassTooLow".to_string(), error.to_string(), mass, min),
        TakeoffCalculationError::MassTooHigh { max, mass } => ("mass".to_string(), "MassTooHigh".to_string(), error.to_string(), mass, max),
        TakeoffCalculationError::InvalidPressureAltitude { source } => match source {
            UndefinedPressureAltitudeError::AboveMaximum { max, pressure_altitude } => ("pressure_altitude".to_string(), "PressureAltitudeAboveMaximum".to_string(), source.to_string(), pressure_altitude, max),
            UndefinedPressureAltitudeError::BelowMinimum { min, pressure_altitude } => ("pressure_altitude".to_string(), "PressureAltitudeBelowMinimum".to_string(), source.to_string(), pressure_altitude, min),
        },
        TakeoffCalculationError::TemperatureTooLow { min, temperature } => ("temperature".to_string(), "TemperatureTooLow".to_string(), error.to_string(), temperature, min),
        TakeoffCalculationError::TemperatureTooHigh { max, temperature } => ("temperature".to_string(), "TemperatureTooHigh".to_string(), error.to_string(), temperature, max),
        TakeoffCalculationError::SlopeTooSteep { max, slope } => ("slope".to_string(), "SlopeTooSteep".to_string(), error.to_string(), slope, max),
    };

    let json = json!({
        "input": input,
        "kind": kind,
        "message": message,
        "value": value,
        "threshold": threshold,
    });

    JsError::new(&json.to_string())
}

#[wasm_bindgen]
pub fn pressure_altitude_meter_to_feet(qnh: f64, field_elevation: f64) -> f64 {
    utils::set_panic_hook();
    round(meter_to_feet(pressure_altitude_by_qnh(qnh, field_elevation)), 2)
}

#[wasm_bindgen]
pub fn pressure_altitude_feet_to_feet(qnh: f64, field_elevation: f64) -> f64 {
    utils::set_panic_hook();
    round(meter_to_feet(pressure_altitude_by_qnh(qnh, feet_to_meter(field_elevation))), 2)
}
