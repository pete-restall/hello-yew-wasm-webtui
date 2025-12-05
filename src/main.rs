use yew::prelude::*;

mod tui_box;
use tui_box::TuiBox;

mod tui_textbox;
use tui_textbox::TuiTextbox;

pub mod vdom;

const STATIC_STUFF_HTML: &str = include_str!("static_stuff.min.html");

#[function_component]
fn App() -> Html {
    let name = use_state(|| AttrValue::from("Yew"));
	let on_changed = {
		let name = name.clone();
		Callback::from(move |value| name.set(value))
	};

    let static_stuff = Html::from_html_unchecked(STATIC_STUFF_HTML.into());
    html! {
        <TuiBox title={format!("Hello, {} !", *name)}>
            {"Name: "}<TuiTextbox value={(*name).clone()} {on_changed} />
            {static_stuff}
        </TuiBox>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
