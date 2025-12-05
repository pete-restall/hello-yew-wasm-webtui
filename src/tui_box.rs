use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

use crate::vdom::try_set_attributes_for_tag;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: AttrValue,
    pub children: Html
}

#[derive(Properties, PartialEq)]
struct HeaderProps {
    pub title: AttrValue,
}

#[function_component]
fn Header(props: &HeaderProps) -> Html {
    let mut span = html_nested! {
        <span>{props.title.clone()}</span>
    };

    try_set_attributes_for_tag(&mut span, &[
        ("is-", "badge"),
        ("variant-", "yellow"),
        ("cap-", "square triangle")
    ]).unwrap_throw();

    span.into()
}

#[function_component]
pub fn TuiBox(props: &Props) -> Html {
    let mut div = html_nested! {
        <div>
            <div class="header"><Header title={props.title.clone()} /></div>
            <div class="content">{props.children.clone()}</div>
        </div>
    };

    try_set_attributes_for_tag(&mut div, &[
        ("box-", "round"),
        ("shear-", "top"),
        ("class", "box")
    ]).unwrap_throw();

    div.into()
}
