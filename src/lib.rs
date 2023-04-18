use aviation_calculator::*;
use wasm_bindgen::prelude::*;

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, aviation-calculator-wasm!");
}

#[wasm_bindgen]
pub fn ground_speed(course: f64, tas: f64, wd: f64, ws: f64) -> f64 {
    return calculate_ground_speed(course, tas, wd, ws);
}
