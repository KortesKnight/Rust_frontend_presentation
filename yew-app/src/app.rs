use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use crate::{processes::switch::switch, shared::Route};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="h-screen w-screen
                            flex justify-center items-center flex-col
                            bg-slate-400 text-orange-500
                            text-base">
                <Switch<Route> render={switch} />
            </div>
        </BrowserRouter>
    }
}