use crate::shared::shared::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Header() -> Html {
    html! {

        <div class="fixed w-full flex space-x-4 top-0 left-0 bg-neutral-500">
                <Link<Route> classes={"p-2 text-white hover:bg-pink-600 hover:text-cyan-400"} to={Route::Main}>{"Main page!"}</Link<Route>>
                <Link<Route> classes={"p-2 text-white hover:bg-pink-600 hover:text-cyan-400"} to={Route::Fibonacci}>{"Fibonacci page!"}</Link<Route>>
                <Link<Route> classes={"p-2 text-white hover:bg-pink-600 hover:text-cyan-400"} to={Route::NotFound}>{"Not Found page!"}</Link<Route>>
            </div>
    }
}
