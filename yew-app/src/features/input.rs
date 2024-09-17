use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub label: String,
    pub value: String,
    pub on_change: Callback<String>,
}

#[function_component(InputField)]
pub fn input_field(props: &InputProps) -> Html {
    let input_ref = use_node_ref();
    let onchange = {
        let input_ref = input_ref.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |_: InputEvent| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                on_change.emit(input.value());
            }
        })
    };

    html! {
        <div class="grid grid-cols-8 gap-4">
            <label class="col-span-3">{ &props.label }</label>
            <input ref={input_ref} class="col-span-5 text-black" type="text" value={props.value.clone()} oninput={onchange} />
        </div>
    }
}
