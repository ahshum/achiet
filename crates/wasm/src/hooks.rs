use crate::{context::AuthContext, router::Query};
use gloo_net::http::RequestBuilder;
use yew::prelude::*;
use yew_router::hooks::*;

/// A RequestBuilder with auth header filled
#[hook]
pub fn use_request() -> impl Clone + Fn(&str, &str) -> RequestBuilder {
    let auth_ctx = use_context::<AuthContext>().expect("auth context");

    {
        let auth_ctx = auth_ctx.clone();
        move |method: &str, url: &str| {
            let req = RequestBuilder::new(url).method(method.parse().expect("method"));

            if let Some(token) = auth_ctx.token.clone() {
                req.header("authorization", &format!("Bearer {}", token))
            } else {
                req
            }
        }
    }
}

/// Get Query from location with default values
#[hook]
pub fn use_query() -> Query {
    let location = use_location().expect("location");
    let query = location.query::<Query>().expect("location query");
    query
}
