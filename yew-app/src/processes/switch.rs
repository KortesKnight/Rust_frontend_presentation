use yew::prelude::*;
use crate::shared::Route;
use crate::widget::{fibonacci::Fibonacci, form::ContactForm, header::Header};

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Main => html! {
            <>
                <Header/>
                <div class="flex justify-center items-center flex-col
                            bg-slate-600
                            p-4
                            gap-4">
                    <h1 class="text-3xl">{ "Форма обратной связи" }</h1>
                    <ContactForm />
                </div>
            </>
        },
        Route::Fibonacci => html! {
            <>
                <Header/>
                <Fibonacci/>
            </>
        },
        Route::NotFound => html! {
            <>
                <Header/>
                <div class="h-screen w-screen bg-black flex justify-center items-center">
                    <p class="text-center align-middle text-white text-7xl">{"Nothing is here. 404"}</p>
                </div>
            </>
        },
    }
}
