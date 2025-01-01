use crate::{
    context::AppContext,
    form::{Input, Label, Textarea},
    icons::Icon,
    router::{Link, Query, QueryMode, QueryPart, Route},
    tag::TagChip,
};
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::hooks::*;

/// Bookmark model
#[derive(Clone, PartialEq, Serialize, Deserialize, Default, Debug)]
pub struct BookmarkModel {
    pub id: String,
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

/// Props of BookmarkPanel FC
#[derive(Properties, PartialEq)]
pub struct BookmarkPanelProps {
    #[prop_or_default]
    pub id: Option<String>,
}

/// BookmarkPanel FC
#[function_component]
pub fn BookmarkPanel(props: &BookmarkPanelProps) -> Html {
    let app_ctx = use_context::<AppContext>().expect("app context");
    let location = use_location().expect("location");
    let current_query = location.query::<Query>().expect("current query");
    let current_mode = current_query.mode_or_default();
    let is_new = props.id.is_none();
    let data = use_state(BookmarkModel::default);

    {
        let data = data.clone();
        let app_bms = app_ctx.bookmarks.clone();
        let current_id = props.id.clone();
        use_effect_with((current_id, app_bms), move |deps| {
            let (current_id, app_bms) = deps.clone();
            let bm = match current_id {
                Some(id) => app_bms
                    .into_iter()
                    .find(|bm| bm.id == id)
                    .unwrap_or_default(),
                None => BookmarkModel::default(),
            };
            data.set(bm);
        });
    }

    let render_view = {
        let data = data.clone();
        move || {
            html! {
                <div class="flex flex-col px-8 space-y-4">
                    <div>
                        <div class="text-xl">
                            {(*data).title.clone()}
                        </div>
                        if !(*data).url.is_empty() {
                            <a
                                class="flex flex-row items-center underline"
                                href={(*data).url.clone()}
                                target="_blank"
                            >
                                <span>{(*data).url.clone()}</span>
                            </a>
                        }
                    </div>
                    if (*data).tags.len() > 0 {
                        <div class="flex flex-row flex-wrap">
                            {(*data).tags.clone().into_iter().map(|tag| html! {
                                <TagChip key={tag.clone()} tag_str={tag.clone()} />
                            }).collect::<Html>()}
                        </div>
                    }
                    if let Some(desc) = (*data).description.clone() {
                        <div>{desc}</div>
                    }
                </div>
            }
        }
    };

    let render_edit = {
        let data = data.clone();

        enum UpdateField {
            Title,
            Url,
            Description,
        }

        let handle_input = {
            let data = data.clone();
            |field: UpdateField| {
                Callback::from(move |value: String| {
                    let updated = match field {
                        UpdateField::Title => BookmarkModel {
                            title: value,
                            ..(*data).clone()
                        },
                        UpdateField::Url => BookmarkModel {
                            url: value,
                            ..(*data).clone()
                        },
                        UpdateField::Description => BookmarkModel {
                            description: Some(value),
                            ..(*data).clone()
                        },
                    };
                    data.set(updated);
                })
            }
        };

        move || {
            html! {
                <div class="flex flex-col px-8 gap-4">

                    <div class="grid">
                        <Label>{"Title"}</Label>
                        <Input
                            name="title"
                            value={(*data).title.clone()}
                            oninput={handle_input.clone()(UpdateField::Title)}
                        />
                    </div>
                    <div class="grid">
                        <Label>{"Url"}</Label>
                        <Input
                            name="url"
                            value={(*data).url.clone()}
                            oninput={handle_input.clone()(UpdateField::Url)}
                        />
                    </div>
                    <div class="grid">
                        <Label>{"Description"}</Label>
                        <Textarea
                            name="description"
                            value={(*data).description.clone().unwrap_or_default()}
                            oninput={handle_input.clone()(UpdateField::Description)}
                        />
                    </div>

                </div>
            }
        }
    };

    html! {
        <div class="flex flex-col py-8 relative">
            {match (is_new, current_mode) {
                (true, _)|(false, QueryMode::Edit) => render_edit(),
                _ => render_view(),
            }}

            <Link
                href={Route::Home}
                query_parts={vec![QueryPart::Mode(None)]}
                class="absolute right-2 top-2"
            >
                <div class="size-6">
                    {Icon::XMark}
                </div>
            </Link>
        </div>
    }
}
