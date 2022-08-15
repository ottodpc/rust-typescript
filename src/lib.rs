use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen] // mark to able to export in wasm
pub fn person(name: &str) {
    println!("{}", name);
}
