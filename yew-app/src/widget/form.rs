mod api;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use api::{FormData, httpbin_org_request};
use crate::features::input::InputField;

enum SubmitState {
    Idle,
    Loading,
    Success(FormData),
    Error(String),
}

#[function_component(ContactForm)]
pub fn contact_form() -> Html {
    let name = use_state(String::new);
    let email = use_state(String::new);
    let message = use_state(String::new);
    let submit_state = use_state(|| SubmitState::Idle); // Используем состояние отправки

    let submit_form = {
        let name = name.clone();
        let email = email.clone();
        let message = message.clone();
        let submit_state = submit_state.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); // Отключаем стандартное поведение формы
            submit_state.set(SubmitState::Loading); // Устанавливаем состояние ожидания

            let form_data = FormData {
                name: (*name).clone(),
                email: (*email).clone(),
                message: (*message).clone(),
            };

            // Отправляем запрос асинхронно
            let submit_state = submit_state.clone();
            spawn_local(async move {
                match httpbin_org_request(&form_data).await {
                    Ok(response_data) => submit_state.set(SubmitState::Success(response_data)),
                    Err(error) => submit_state.set(SubmitState::Error(error)),
                }
            });
        })
    };

    match &*submit_state {
        SubmitState::Idle => html! {
            <form class="flex flex-col justify-center items-center space-y-4" onsubmit={submit_form}>
                <InputField label="Имя" value={(*name).clone()} on_change={Callback::from(move |v| name.set(v))} />
                <InputField label="Email" value={(*email).clone()} on_change={Callback::from(move |v| email.set(v))} />
                <InputField label="Сообщение" value={(*message).clone()} on_change={Callback::from(move |v| message.set(v))} />
                <button type="submit">{ "Отправить" }</button>
            </form>
        },
        SubmitState::Loading => html! {
            <p>{ "Ожидаем ответ от сервера..." }</p>
        },
        SubmitState::Success(data) => html! {
            <div class="flex flex-col justify-center items-center space-y-4">
                <h2>{ "Спасибо за ваше сообщение!" }</h2>
                <p>{ format!("Имя: {}", data.name) }</p>
                <p>{ format!("Email: {}", data.email) }</p>
                <p>{ format!("Сообщение: {}", data.message) }</p>
            </div>
        },
        SubmitState::Error(error) => html! {
            <div class="flex flex-col justify-center items-center space-y-4">
                <h2>{ "Ошибка!" }</h2>
                <p>{ format!("Произошла ошибка: {}", error) }</p>
                <button onclick={Callback::from(move |_| submit_state.set(SubmitState::Idle))}>
                    { "Попробовать снова" }
                </button>
            </div>
        },
    }
}
