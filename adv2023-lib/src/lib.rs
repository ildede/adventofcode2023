use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn day_2_part_a(input: &str) -> String {
    String::from(format!("not solved yet"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = day_2_part_a("test");
        assert_eq!(result, "not solved yet");
    }
}
