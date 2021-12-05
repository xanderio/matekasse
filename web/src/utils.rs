use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let class = classes!("button", "is-fullwidth", "is-medium", "is-primary", "mb-2");
    html! {
        <button {class} onclick={props.onclick.clone()}>
            {props.children.clone()}
        </button>
    }
}
