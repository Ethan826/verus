mod utils;

use wasm_bindgen::prelude::*;
use verus_validations;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(js_name = validateEmail)]
pub fn wrapper_validate_email(email: &str) -> bool {
    verus_validations::validate_email(email)
}

#[wasm_bindgen(js_name = validateDate)]
pub fn wrapper_validate_date(date: &str) -> bool {
    verus_validations::validate_date(date)
}