use crate::{
    context::AuthContext,
    form::{Button, Input, Label},
    router::{Link, Route},
};
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

/// Props of Login FC
#[derive(Properties, PartialEq)]
pub struct LoginProps {
    #[prop_or_default]
    pub is_register: bool,
}

/// Login form data, also for request body
#[derive(Default, Clone, Serialize)]
pub struct LoginFormData {
    pub username: String,
    pub password: String,
}

/// Login response data
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponseData {
    pub access_token: String,
}

/// Login FC
#[function_component]
pub fn Login(props: &LoginProps) -> Html {
    let form_data = use_state(LoginFormData::default);
    let auth_ctx = use_context::<AuthContext>().expect("auth context");

    let on_form_submit = {
        let form_data = form_data.clone();
        let auth_ctx = auth_ctx.clone();
        Callback::from(move |_: MouseEvent| {
            let form_data = form_data.clone();
            let auth_ctx = auth_ctx.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let res = Request::post("/api/auth")
                    .json(&(*form_data).clone())
                    .expect("request body")
                    .send()
                    .await
                    .expect("response");

                if res.ok() {
                    let res_body = res
                        .json::<LoginResponseData>()
                        .await
                        .expect("parse response");
                    auth_ctx.dispatch(Some(res_body.access_token));
                }
            });
        })
    };

    enum UpdateField {
        Username,
        Password,
    }

    let handle_input = {
        let form_data = form_data.clone();

        |field: UpdateField| {
            Callback::from(move |value: String| {
                let updated = match field {
                    UpdateField::Username => LoginFormData {
                        username: value,
                        ..(*form_data).clone()
                    },
                    UpdateField::Password => LoginFormData {
                        password: value,
                        ..(*form_data).clone()
                    },
                };
                form_data.set(updated);
            })
        }
    };

    html! {
        <div class="flex min-h-screen">
            <div class="flex flex-col min-w-[320px] p-2 m-auto">
                <div class="flex flex-col gap-4">
                    <div class="text-xl">
                        <span class="font-bold">
                            {"Achiet"}
                        </span>
                        <span class="border-r border-solid mx-3" />
                        <span>
                            if props.is_register {
                                {"Register"}
                            } else {
                                {"Login"}
                            }
                        </span>
                    </div>

                    <div class="grid">
                        <Label>{"Username"}</Label>
                        <Input
                            name="username"
                            value={(*form_data).username.clone()}
                            oninput={handle_input.clone()(UpdateField::Username)}
                        />
                    </div>

                    <div class="grid">
                        <Label>{"Password"}</Label>
                        <Input
                            name="password"
                            input_type="password"
                            value={(*form_data).password.clone()}
                            oninput={handle_input.clone()(UpdateField::Password)}
                        />
                    </div>

                    <Button onclick={on_form_submit}>
                        if props.is_register {
                            {"Register"}
                        } else {
                            {"Login"}
                        }
                    </Button>

                    if props.is_register {
                        <Link class="text-center" href={Route::Login}>{"Switch to Login"}</Link>
                    } else {
                        <Link class="text-center" href={Route::Register}>{"Switch to Register"}</Link>
                    }
                </div>
            </div>
        </div>
    }
}
