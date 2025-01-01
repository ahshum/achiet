use yew::prelude::*;

/// Props of Label FC
#[derive(Properties, PartialEq)]
pub struct LabelProps {
    #[prop_or_default]
    pub class: Classes,
    pub children: Html,
}

/// Label FC
#[function_component]
pub fn Label(props: &LabelProps) -> Html {
    html! {
        <label
            class={classes!(props.class.clone())}
        >
            {props.children.clone()}
        </label>
    }
}

/// Props of Input FC
#[derive(Properties, PartialEq)]
pub struct InputProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or("text".to_string())]
    pub input_type: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub oninput: Callback<String>,
}

/// Input FC
#[function_component]
pub fn Input(props: &InputProps) -> Html {
    let oninput = {
        let cb = props.oninput.clone();
        Callback::from(move |event: InputEvent| {
            let target = event
                .target_dyn_into::<web_sys::HtmlInputElement>()
                .expect("target");
            cb.emit(target.value());
        })
    };

    html! {
        <input
            class={classes!(
                "outline-none", "rounded", "border", "px-2", "py-1",
                "bg-[var(--color-input-bg)]",
                "border-[var(--color-input-border)]",
                "focus:border-[var(--color-input-border-focus)]",
                props.class.clone(),
            )}
            name={props.name.clone()}
            type={props.input_type.clone()}
            value={props.value.clone()}
            oninput={oninput}
        />
    }
}

/// Props of Textarea FC
#[derive(Properties, PartialEq)]
pub struct TextareaProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub oninput: Callback<String>,
}

/// Textarea FC
#[function_component]
pub fn Textarea(props: &TextareaProps) -> Html {
    let oninput = {
        let cb = props.oninput.clone();
        Callback::from(move |event: InputEvent| {
            let target = event
                .target_dyn_into::<web_sys::HtmlTextAreaElement>()
                .expect("target");
            cb.emit(target.value());
        })
    };

    html! {
        <textarea
            class={classes!(
                "outline-none", "rounded", "border", "px-2", "py-1",
                "bg-[var(--color-input-bg)]",
                "border-[var(--color-input-border)]",
                "focus:border-[var(--color-input-border-focus)]",
                props.class.clone(),
            )}
            name={props.name.clone()}
            value={props.value.clone()}
            oninput={oninput}
        />
    }
}

/// Props of Button FC
#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

/// Button FC
#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    html! {
        <button
            class={classes!(
                "outline-none", "border", "p-1", "rounded",
                "border-[var(--color-input-border)]",
                "focus:border-[var(--color-input-border-focus)]",
                props.class.clone(),
            )}
            onclick={props.onclick.clone()}
        >
            {props.children.clone()}
        </button>
    }
}
