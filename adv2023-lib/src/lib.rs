use wasm_bindgen::prelude::*;

mod day2;

#[wasm_bindgen]
pub fn day_2_part_a(input: &str) -> String {
    day2::solve_part_a(input)
}
