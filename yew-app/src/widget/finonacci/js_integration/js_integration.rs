use gloo_console::log;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

// Объявляем функцию из JavaScript
#[wasm_bindgen(module = "/src/widget/finonacci/js_integration/hellow_world.js")]
extern "C" {
    pub fn greet(name: &str) -> String;
}

// Объявляем функцию из JavaScript
#[wasm_bindgen(module = "/src/widget/finonacci/js_integration/fibonacci.js")]
extern "C" {
    fn measure_js_fibonacci(n: u32) -> JsValue;

    fn measure_js_1_fibonacci(n: u32) -> JsValue;

}

// Объявляем структуру, которая соответствует объекту из JavaScript
#[derive(Deserialize, Default)]
pub struct JsFibonacciResult {
    pub result: u32,
    pub time: f64,
    pub n: u32,
}

pub enum Method {
    Recursive,
    Iterative,
}

impl JsFibonacciResult {
    fn new(value: JsValue) -> Self {
        match serde_wasm_bindgen::from_value(value) {
            Ok(res) => res,
            Err(err) => {
                log!(err.to_string());
                JsFibonacciResult {
                    result: 0,
                    time: 0.0,
                    n: 0,
                }
            }
        }
    }

    pub fn measure_js_fibonacci(n: u32, method: Method) -> Self {
        match method {
            Method::Recursive => JsFibonacciResult::new(measure_js_fibonacci(n)),
            Method::Iterative => JsFibonacciResult::new(measure_js_1_fibonacci(n)),
        }
    }
}
