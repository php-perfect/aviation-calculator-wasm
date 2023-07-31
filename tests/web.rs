//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;

use aviation_calculator_wasm::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    let result: TakeoffDistances = fk9_takeoff_distance(Engine::Rotax912Uls, 520.0, 300.0, 1.0, 0.0, false, false, false);
    assert_eq!(130.38, result.takeoff_run);
    assert_eq!(323.48, result.to_50_ft_height);
}
