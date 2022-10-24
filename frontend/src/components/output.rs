use std::rc::Rc;
use gloo_net::http::QueryParams;
use yew::prelude::*;
use crate::{ActionButtonState, ActionButtonStateContext};

#[derive(Properties, PartialEq)]
pub struct OutputContainerProps {
    pub value: Rc<str>,
}

#[function_component]
pub fn OutputContainer(props: &OutputContainerProps) -> Html {
    let action_button_state = use_context::<ActionButtonStateContext>().unwrap();
    let loading = use_state(|| true);
    let src = use_state(|| AttrValue::from("about:black"));

    {
        let loading = loading.clone();
        let src = src.clone();
        use_effect_with_deps(move |value| {
            src.set({
                let query = QueryParams::new();
                query.append("code", &*value);
                AttrValue::from(format!("/api/run/?{}", query))
            });
            loading.set(false);
        }, Rc::clone(&props.value))
    };

    let fallback = html! { <div class="h-full bg-gray-600">{"Loading"}</div> };

    let onload = move |_| {
        action_button_state.dispatch(ActionButtonState::Enabled);
    };
    let classes = classes!("w-full", "h-full", "border-t-[10px]", "border-gray-400", "border-solid", if *loading { "invisible" } else { "visible" });
    html! {
        <>
            if *loading {
                {fallback}
            }
            <iframe src={AttrValue::clone(&*src)} {onload} class={classes} />
        </>
    }
}
