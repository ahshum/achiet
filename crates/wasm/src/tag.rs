use crate::context::AppContext;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

/// Tag model
#[derive(Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TagModel {
    pub id: String,
    pub path: String,
    pub prefix: String,
    pub name: String,
}

/// Props of TagChip FC
#[derive(Properties, PartialEq)]
pub struct TagChipProps {
    #[prop_or_default]
    pub class: Classes,
    pub tag_str: String,
}

/// TagChip FC
#[function_component]
pub fn TagChip(props: &TagChipProps) -> Html {
    html! {
        <div
            class={classes!(
                "flex-none", "rounded-full", "border", "border-white",
                "px-2", "flex", "flex-row", "text-sm",
                props.class.clone(),
            )}
        >
            {props.tag_str.clone()}
        </div>
    }
}

/// Props of TagInput FC
#[derive(Properties, PartialEq)]
pub struct TagInputProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub value: String,
}

/// TagInput FC
#[function_component]
pub fn TagInput(props: &TagInputProps) -> Html {
    let app_ctx = use_context::<AppContext>().expect("app context");
    let input_value = use_state(|| String::new());

    let on_input = {
        let input_value = input_value.clone();
        Callback::from(move |event: InputEvent| {
            let target = event
                .target_dyn_into::<web_sys::HtmlInputElement>()
                .expect("target");
            input_value.set(target.value());
        })
    };

    html! {
        <>
            <input
                class={classes!(
                    "w-auto",
                    "outline-none",
                )}
                value={(*input_value).clone()}
                oninput={on_input}
            />
            <div></div>
        </>
    }
}

/// Props of TagSelect FC
#[derive(Properties, PartialEq)]
pub struct TagSelectProps {
    #[prop_or_default]
    pub class: Classes,
    pub value: Vec<String>,
}

/// TagSelect FC
#[function_component]
pub fn TagSelect(props: &TagSelectProps) -> Html {
    html! {
        <div>
        </div>
    }
}
