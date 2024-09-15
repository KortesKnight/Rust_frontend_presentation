use yew::prelude::*;
use web_sys;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub label: String,
    pub value: String,
    pub on_change: Callback<String>,
}

#[function_component(InputField)]
pub fn input_field(props: &InputProps) -> Html {
    let onchange = {
        let on_change = props.on_change.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            on_change.emit(input.value());
        })
    };

    html! {
        <div class="grid grid-cols-8 gap-4">
            <label class="col-span-3">{ &props.label }</label>
            <input class="col-span-5 text-black" type="text" value={props.value.clone()} oninput={onchange} />
        </div>
    }
}
