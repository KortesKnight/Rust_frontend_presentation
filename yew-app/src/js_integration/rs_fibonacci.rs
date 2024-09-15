use chrono::{Timelike, Utc};
// use gloo_console::log;

// Функция для расчёта числа Фибоначчи на Rust
pub fn fibonacci_rust(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fibonacci_rust(n - 1) + fibonacci_rust(n - 2)
}

// Функция для измерения времени выполнения на Rust с использованием web_sys
pub fn measure_rust_fibonacci(n: u32) -> (u32, i64, u32) {
    let start = Utc::now().timestamp_millis();
    let result = fibonacci_rust(n);
    let end = Utc::now().timestamp_millis();
    let duration = end - start;

    (result, duration, n)
}

// Функция для расчёта числа Фибоначчи на Rust путём иттерации
pub fn fibonacci_rust_1(n: u32) -> u128 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a: u128 = 0;
    let mut b: u128 = 1;
    for _ in 2..=n {
        let temp = match a.checked_add(b) {
            Some(temp) => temp,
            None => break,
        };

        a = b;
        b = temp;
    }
    b
}

// Функция для измерения времени выполнения на Rust с использованием web_sys
pub fn measure_rust_1_fibonacci(n: u32) -> (u128, u32, u32) {
    let start = Utc::now().nanosecond();
    let result = fibonacci_rust_1(n);
    let end = Utc::now().nanosecond();
    let duration = end - start;

    (result, duration, n)
}
