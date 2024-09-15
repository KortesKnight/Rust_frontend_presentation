use wasm_bindgen::prelude::*;

// Экспортируем функцию, чтобы её можно было вызвать из JavaScript
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}