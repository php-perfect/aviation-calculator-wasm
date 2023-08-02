//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use wasm_bindgen::JsError;
use wasm_bindgen_test::*;

use aviation_calculator_wasm::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    let result: Result<TakeoffDistances, JsError> = fk9_takeoff_distance(Engine::Rotax912Uls, 520.0, 300.0, 14.0, 0.0, true, false, false, false, false, SurfaceCondition::Inconspicuous);
    let Ok(takeoff_distances) = result else { panic!("No error expected!") };
    assert_eq!(128.56, takeoff_distances.takeoff_run);
    assert_eq!(318.97, takeoff_distances.to_50_ft_height);
}

#[wasm_bindgen_test]
fn error() {
    let result: Result<TakeoffDistances, JsError> = fk9_takeoff_distance(Engine::Rotax912Uls, 10.0, 300.0, 1.0, 0.0, false, false, false, false, false, SurfaceCondition::Inconspicuous);
    assert!(result.is_err());
}
