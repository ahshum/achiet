use crate::{bookmark::BookmarkModel, router::Route, tag::TagModel};
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;

/// App state
#[derive(Clone, PartialEq, Default)]
pub struct AppState {
    pub bookmarks: Vec<BookmarkModel>,
    pub tags: Vec<TagModel>,
}

impl Reducible for AppState {
    type Action = AppAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_state = match action {
            AppAction::SetBookmarks(bms) => Self {
                bookmarks: bms,
                ..(*self).clone()
            },
            AppAction::SetTags(tags) => Self {
                tags,
                ..(*self).clone()
            },
        };
        next_state.into()
    }
}

/// App state action
pub enum AppAction {
    SetBookmarks(Vec<BookmarkModel>),
    SetTags(Vec<TagModel>),
}

/// App context
pub type AppContext = UseReducerHandle<AppState>;

/// Props of AppProvider FC
#[derive(Properties, PartialEq)]
pub struct AppProviderProps {
    pub children: Html,
}

/// AppProvider FC
#[function_component]
pub fn AppProvider(props: &AppProviderProps) -> Html {
    let state = use_reducer(|| AppState::default());

    html! {
        <ContextProvider<AppContext> context={state}>
            {props.children.clone()}
        </ContextProvider<AppContext>>
    }
}

/// Auth key for local storage
const AUTH_KEY: &str = "auth";

/// Auth state
#[derive(Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthState {
    pub token: Option<String>,
}

impl Reducible for AuthState {
    type Action = Option<String>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_state = match action {
            Some(token) => Self { token: Some(token) },
            None => Self { token: None },
        };
        LocalStorage::set(AUTH_KEY, next_state.clone()).expect("local storage set auth");
        next_state.into()
    }
}

/// Auth context
pub type AuthContext = UseReducerHandle<AuthState>;

/// Props of AuthProvider FC
#[derive(Properties, PartialEq)]
pub struct AuthProviderProps {
    pub children: Html,
}

/// AuthProvider FC
#[function_component]
pub fn AuthProvider(props: &AuthProviderProps) -> Html {
    let state = use_reducer(|| LocalStorage::get::<AuthState>(AUTH_KEY).unwrap_or_default());
    let location = use_location().expect("location");
    let navigator = use_navigator().expect("navigator");

    {
        let location = location.clone();
        let navigator = navigator.clone();
        let state = state.clone();
        use_effect_with(
            (
                (*state).token.clone(),
                Route::recognize(location.path()).expect("routes"),
            ),
            move |(token, route)| {
                let is_guest_routes = match route {
                    Route::Login | Route::Register => true,
                    _ => false,
                };

                match (token, is_guest_routes) {
                    (Some(_), true) => navigator.replace(&Route::Home),
                    (None, false) => navigator.replace(&Route::Login),
                    _ => (),
                };
            },
        );
    }

    html! {
        <ContextProvider<AuthContext> context={state}>
            {props.children.clone()}
        </ContextProvider<AuthContext>>
    }
}
