use wasm_bindgen::prelude::*;

mod day2;
mod day3;

#[wasm_bindgen]
pub fn day_2_part_a(input: &str) -> String {
    day2::solve_part_a(input)
}

#[wasm_bindgen]
pub fn day_2_part_b(input: &str) -> String {
    day2::solve_part_b(input)
}

#[wasm_bindgen]
pub fn day_3_part_a(input: &str) -> String {
    day3::solve_part_a(input)
}
