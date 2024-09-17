use gloo_console::log;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::js_integration::{
    js_integration::{greet, JsFibonacciResult, Method},
    rs_fibonacci::*,
};

// Компонент Yew
#[function_component]
pub fn Fibonacci() -> Html {
    let name = "Yew Developer";
    let greeting = greet(name);

    let n_state = use_state(|| 30);

    let js_state =
        use_state(|| JsFibonacciResult::measure_js_fibonacci(*n_state, Method::Recursive));

    let js_1_state =
        use_state(|| JsFibonacciResult::measure_js_fibonacci(*n_state, Method::Iterative));

    let rust_state = use_state(|| measure_rust_fibonacci(*n_state));

    let rust_1_state = use_state(|| measure_rust_1_fibonacci(*n_state));

    let add = {
        let n_state = n_state.clone();
        Callback::from(move |_| {
            n_state.set((*n_state) + 1);
        })
    };

    let remove = {
        let n_state = n_state.clone();
        Callback::from(move |_| {
            n_state.set((*n_state) - 1);
        })
    };

    let recalculate_js = {
        let js_state = js_state.clone();
        let n_state = n_state.clone();

        Callback::from(move |_| {
            js_state.set(JsFibonacciResult::measure_js_fibonacci(
                *n_state,
                Method::Recursive,
            ))
        })
    };

    let recalculate_js_1 = {
        let js_1_state = js_1_state.clone();
        let n_state = n_state.clone();

        Callback::from(move |_| {
            js_1_state.set(JsFibonacciResult::measure_js_fibonacci(
                *n_state,
                Method::Iterative,
            ))
        })
    };

    let recalculate_rust = {
        let rust_state = rust_state.clone();
        let n_state = n_state.clone();

        Callback::from(move |_| rust_state.set(measure_rust_fibonacci(*n_state)))
    };

    let recalculate_rust_1 = {
        let rust_1_state = rust_1_state.clone();
        let n_state = n_state.clone();

        Callback::from(move |_| rust_1_state.set(measure_rust_1_fibonacci(*n_state)))
    };

    let node_ref = use_node_ref();

    let oninput = {
        let n_state = n_state.clone();
        let node_ref = node_ref.clone();

        Callback::from(move |_| {
            if let Some(input) = node_ref.cast::<HtmlInputElement>() {
                let num = match input.value().parse::<u32>() {
                    Ok(num) => num,
                    Err(err) => {
                        log!(err.to_string());
                        u32::default()
                    }
                };

                n_state.set(num);
            }
        })
    };

    html! {
        <>
            <div class="bg-slate-600 p-4 mb-8">
                <h1>{ "Rust & Yew with JavaScript" }</h1>
                <p>{ greeting }</p> // Отображаем результат вызова JS функции
            </div>

            <div class="bg-slate-600 p-4 flex flex-col space-y-3">
                <h1>{ "Rust vs JavaScript Fibonacci Calculation" }</h1>
                <div style="display: flex; height: fit-content; margin-bottom: 1rem;">
                    <button class="size-6 bg-neutral-500 text-white hover:bg-pink-600 hover:text-cyan-400" onclick={remove}>{"-"}</button>

                    <p style="margin: 0;">
                        {"Actual N state: "}
                        <input ref={node_ref} oninput={oninput} value={format!("{}", *n_state)} type="number" />
                    </p>

                    <button class="size-6 bg-neutral-500 text-white hover:bg-pink-600 hover:text-cyan-400" onclick={add}>{"+"}</button>
                </div>

                <button class="p-2 bg-neutral-500 text-white hover:bg-pink-600 hover:text-cyan-400" onclick={recalculate_js}>{"Recalculate JS"}</button>
                <p class="p-2">{ format!("JavaScript: Result = {}, Time = {} ms, N = {:?}", (*js_state).result, (*js_state).time, (*js_state).n) }</p>

                <button class="p-2 bg-neutral-500 text-white hover:bg-pink-600 hover:text-cyan-400" onclick={recalculate_js_1}>{"Recalculate JS 1"}</button>
                <p class="p-2">{ format!("JavaScript: Result = {}, Time = {} ms, N = {:?}", (*js_1_state).result, (*js_1_state).time, (*js_1_state).n) }</p>

                <button class="p-2 bg-neutral-500 text-white hover:bg-pink-600 hover:text-cyan-400" onclick={recalculate_rust}>{"Recalculate Rust"}</button>
                <p class="p-2">{ format!("Rust: Result = {}, Time = {} ms, N = {:?}", (*rust_state).0, (*rust_state).1, (*rust_state).2) }</p>

                <button class="p-2 bg-neutral-500 text-white hover:bg-pink-600 hover:text-cyan-400" onclick={recalculate_rust_1}>{"Recalculate Rust 1"}</button>
                <p class="p-2">{ format!("Rust: Result = {}, Time = {} ns, N = {:?}", (*rust_1_state).0, (*rust_1_state).1, (*rust_1_state).2) }</p>

            </div>

        </>

    }
}
