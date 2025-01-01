use crate::{
    auth::Login,
    bookmark::{BookmarkModel, BookmarkPanel},
    context::{AppAction, AppContext, AppProvider, AuthContext, AuthProvider},
    hooks::{use_query, use_request},
    icons::Icon,
    router::{Link, QueryMode, QueryPart, Route},
    tag::{TagChip, TagModel},
};
use yew::prelude::*;
use yew_router::{hooks::*, router::*, switch::Switch};

/// Route switch
fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::BookmarkNew => html! {
            <Home>
                <BookmarkPanel />
            </Home>
        },
        Route::BookmarkOf { id } => html! {
            <Home>
                <BookmarkPanel id={Some(id)} />
            </Home>
        },
        Route::Login => html! { <Login /> },
        Route::Register => html! { <Login is_register={true} /> },
    }
}

/// App FC, wrapping providers before main app.
#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <AuthProvider>
                <AppProvider>
                    <Switch<Route> render={switch} />
                </AppProvider>
            </AuthProvider>
        </BrowserRouter>
    }
}

/// Props of Home FC
#[derive(Properties, PartialEq)]
pub struct HomeProps {
    #[prop_or_default]
    pub children: Html,
}

/// Home FC
#[function_component]
pub fn Home(props: &HomeProps) -> Html {
    let auth_ctx = use_context::<AuthContext>().expect("auth context");
    let app_ctx = use_context::<AppContext>().expect("app context");
    let route = use_route::<Route>().expect("route");
    let request = use_request();
    let query = use_query();
    let current_bm_id = match route {
        Route::BookmarkOf { id } => Some(id),
        _ => None,
    };

    {
        let app_ctx = app_ctx.clone();
        let request = request.clone();
        use_effect_with((), move |_| {
            let app_ctx = app_ctx.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let res = request("GET", "/api/bookmark")
                    .send()
                    .await
                    .expect("response");
                if res.ok() {
                    let data = res
                        .json::<Vec<BookmarkModel>>()
                        .await
                        .expect("parse response");
                    app_ctx.dispatch(AppAction::SetBookmarks(data));
                }
            });
        });
    }

    {
        let app_ctx = app_ctx.clone();
        let request = request.clone();
        use_effect_with((), move |_| {
            let app_ctx = app_ctx.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let res = request("GET", "/api/tag").send().await.expect("response");
                if res.ok() {
                    let data = res.json::<Vec<TagModel>>().await.expect("parse response");
                    app_ctx.dispatch(AppAction::SetTags(data));
                }
            });
        });
    }

    let on_logout = {
        let auth_ctx = auth_ctx.clone();
        Callback::from(move |_: MouseEvent| {
            auth_ctx.dispatch(None);
        })
    };

    let render_tag = {
        let query = query.clone();
        let path = query.path_or_default();
        move |tag: TagModel| {
            if tag.prefix != path {
                return html! { <></> };
            }

            html! {
                <Link
                    key={tag.id}
                    class="block"
                    query_parts={vec![QueryPart::Path(Some(tag.path))]}
                >
                    {tag.name}
                </Link>
            }
        }
    };

    let render_bm = {
        let current_bm_id = current_bm_id.clone();
        move |bm: BookmarkModel| {
            html! {
                <div key={bm.id.clone()} class="flex gap-2 px-4 py-3 relative">
                    <div class="flex flex-col gap-1">
                        <div class="flex min-w-0">
                            if bm.url.len() > 0 {
                                <a
                                    href={bm.url.clone()}
                                    target="_blank"
                                    class="z-10 hover:underline flex items-center"
                                >
                                    if bm.title.len() > 0 {
                                        {bm.title.clone()}
                                    } else {
                                        {bm.url.clone()}
                                    }
                                </a>
                            } else {
                                <span>{bm.title.clone()}</span>
                            }
                        </div>
                        if bm.tags.len() > 0 {
                            <div class="flex flex-row gap-1 overflow-x-scroll z-10 scrollbar-hidden">
                                {bm.tags.into_iter().map(|tag| html! {
                                    <TagChip key={tag.clone()} tag_str={tag.clone()} />
                                }).collect::<Html>()}
                            </div>
                        }
                    </div>
                    if current_bm_id.clone().is_some_and(|id| bm.id == id) {
                        <div class="absolute right-0 inset-y-0 flex flex-row items-center">
                            <div class="size-5">
                                {Icon::ChevronRight}
                            </div>
                        </div>
                    }
                    <Link
                        href={Route::BookmarkOf{ id: bm.id.clone() }}
                        query_parts={vec![QueryPart::Mode(None)]}
                        class="absolute inset-0 z-0"
                    />
                </div>
            }
        }
    };

    html! {
        <div class="flex flex-row items-stretch h-screen relative max-w-[var(--container-w)] mx-auto">
            <div class="fixed left-0 inset-x pl-[calc(50%-var(--container-w)/2)] bg-[#2f2f2f]">
                <div class="px-[var(--sidebar-p)] w-[calc(var(--sidebar-w)+var(--sidebar-p)*2)] min-h-screen h-full">
                    <div class="py-3 flex items-center cursor-pointer" onclick={on_logout}>
                        <div class="grow">
                            {"Account"}
                        </div>
                        <div class="size-5">
                            {Icon::UserCircle}
                        </div>
                    </div>

                    <div class="py-2 flex items-center">
                        <div class="grow">
                            {"Tags"}
                        </div>
                    </div>
                    {app_ctx.tags.clone().into_iter().map(render_tag).collect::<Html>()}
                </div>
            </div>

            <div class="grow flex flex-col pl-[calc(var(--sidebar-w)+var(--sidebar-p)*2)]">
                <div class="shadow shadow-black">
                    <div class="px-4 py-2 flex items-center">
                        <CurrentPath />

                        <div class="grow"></div>

                        <div class="px-2 py-1 border-[#3f3f3f] border flex divide-x divide-[#383838] rounded-full mx-2">
                            <Link class="px-2" query_parts={vec![QueryPart::Mode(Some(QueryMode::View))]}>
                                <div class="size-5">
                                    {Icon::Document}
                                </div>
                            </Link>
                            <Link class="px-2" query_parts={vec![QueryPart::Mode(Some(QueryMode::Edit))]}>
                                <div class="size-5">
                                    {Icon::PencilSquare}
                                </div>
                            </Link>
                        </div>

                        <div class="px-3 py-1 border-[#3f3f3f] border-l">
                            <Link class="" href={Route::BookmarkNew} query_parts={vec![QueryPart::Mode(None)]}>
                                <div class="size-5">
                                    {Icon::Bookmark}
                                </div>
                            </Link>
                        </div>

                    </div>
                </div>

                <div class="min-h-0 grow flex flex-row">
                    <div class="overflow-auto shrink-0">
                        <div class="flex-1">
                            <div class="flex relative">
                                <div class="w-[400px] flex flex-col divide-y divide-[#383838]">
                                    {app_ctx.bookmarks.clone().into_iter().map(render_bm).collect::<Html>()}
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="grow flex flex-col shadow shadow-black overflow-auto">
                        {props.children.clone()}
                    </div>
                </div>
            </div>
        </div>
    }
}

/// CurrentPath FC
#[function_component]
pub fn CurrentPath() -> Html {
    let query = use_query();
    let path = query.path_or_default();
    let sub_paths = path.split("/").collect::<Vec<_>>();

    let render_sub_path = {
        let sub_paths = sub_paths.clone();
        move |(i, sub_path): (usize, &str)| {
            let path = sub_paths[..i].join("/");
            let query_part = if path.is_empty() {
                QueryPart::Path(None)
            } else {
                QueryPart::Path(Some(path))
            };

            html! {
                <Link
                    key={sub_path}
                    class={classes!(
                        "group", "relative", "rounded",
                        "hover:outline", "hover:outline-gray-600",
                    )}
                    query_parts={vec![query_part]}
                >
                    <div class="flex space-x-1">
                        <div>{"/"}</div>
                        <div>{sub_path}</div>
                    </div>
                </Link>
            }
        }
    };

    html! {
        <div class="flex items-center space-x-1">
            <div class="size-5">
                {Icon::Tag}
            </div>
            {sub_paths.clone().into_iter()
                .enumerate()
                .skip(1)
                .map(render_sub_path)
                .collect::<Html>()
            }
        </div>
    }
}
