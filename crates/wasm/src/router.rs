use crate::hooks::use_query;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::{hooks::*, Routable};

/// Route
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/bookmark/new")]
    BookmarkNew,
    #[at("/bookmark/:id")]
    BookmarkOf { id: String },
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
}

/// Mode in query
#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryMode {
    #[default]
    View,
    Edit,
}

/// Query part for partial update
#[derive(Clone, PartialEq)]
pub enum QueryPart {
    Mode(Option<QueryMode>),
    Path(Option<String>),
}

/// Query parts for partial update
pub type QueryParts = Vec<QueryPart>;

/// Query of the app
#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Query {
    pub mode: Option<QueryMode>,
    pub path: Option<String>,
}

impl Query {
    pub fn mode_or_default(&self) -> QueryMode {
        self.mode.clone().unwrap_or_default()
    }

    pub fn path_or_default(&self) -> String {
        self.path.clone().map_or("/".to_string(), |s| {
            if s.is_empty() {
                "/".to_string()
            } else {
                s
            }
        })
    }
}

/// Props of Link FC
#[derive(Properties, PartialEq)]
pub struct LinkProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub href: Option<Route>,
    #[prop_or_default]
    pub query_parts: QueryParts,
    #[prop_or_default]
    pub children: Html,
}

/// Link FC
#[function_component]
pub fn Link(props: &LinkProps) -> Html {
    let navigator = use_navigator().expect("navigator");
    let current_route = use_route::<Route>().expect("current route");
    let current_query = use_query();

    let route = props.href.clone().unwrap_or(current_route);
    let query = props
        .query_parts
        .clone()
        .into_iter()
        .fold(current_query, |mut query, part| {
            match part {
                QueryPart::Mode(mode) => query.mode = mode,
                QueryPart::Path(path) => query.path = path,
            }
            query
        });

    let href = use_memo((route.clone(), query.clone()), |(route, query)| {
        let qs = serde_qs::to_string(query).expect("query to_string");
        if qs.len() > 0 {
            format!("{}?{}", route.to_path(), qs)
        } else {
            format!("{}", route.to_path())
        }
    });

    let onclick = {
        let navigator = navigator.clone();
        let route = route.clone();
        let query = query.clone();
        Callback::from(move |event: MouseEvent| {
            if event.alt_key() || event.ctrl_key() || event.shift_key() || event.meta_key() {
                return;
            }

            event.prevent_default();
            let _ = navigator.push_with_query(&route, &query);
        })
    };

    html! {
        <a
            class={props.class.clone()}
            href={(*href).clone()}
            {onclick}
        >
            {props.children.clone()}
        </a>
    }
}
