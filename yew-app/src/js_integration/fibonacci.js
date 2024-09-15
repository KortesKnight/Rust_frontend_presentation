// Функция для расчёта числа Фибоначчи на JavaScript
export function fibonacci_js(n) {
    if (n <= 1) {
        return n;
    }
    return fibonacci_js(n - 1) + fibonacci_js(n - 2);
}

// Функция для измерения времени выполнения на JavaScript
export function measure_js_fibonacci(n) {
    const start = performance.now();
    const result = fibonacci_js(n);
    const end = performance.now();
    return {
        result: result,
        time: end - start,
        n: n
    };
}

// Оптимизированная функция для расчёта числа Фибоначчи на JavaScript
export function fibonacci_js_1(n) {
    if (n === 0) {
        return 0;
    } else if (n === 1) {
        return 1;
    }

    let a = 0;
    let b = 1;
    for (let i = 2; i <= n; i++) {
        let temp = a + b;
        a = b;
        b = temp;
    }
    return b;
}

// Функция для измерения времени выполнения на JavaScript
export function measure_js_1_fibonacci(n) {
    const start = performance.now();
    const result = fibonacci_js(n);
    const end = performance.now();
    return {
        result: result,
        time: end - start,
        n: n
    };
}