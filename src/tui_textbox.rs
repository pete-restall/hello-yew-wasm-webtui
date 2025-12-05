use web_sys::{Event, HtmlInputElement, InputEvent};
use web_sys::wasm_bindgen::{JsCast, UnwrapThrowExt};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub value: AttrValue,
    pub on_changed: Option<Callback<AttrValue>>
}

#[function_component]
pub fn TuiTextbox(props: &Props) -> Html {
    let oninput = {
        let callback = props.on_changed.clone();
        Callback::from(move |event: InputEvent| if let Some(callback) = &callback {
            callback.emit(get_value_from(event));
        })
    };

    html! {
        <input type="text" value={props.value.clone()} {oninput} />
    }
}

fn get_value_from(event: InputEvent) -> AttrValue {
    let event: Event = event.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value().into()
}
