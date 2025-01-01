use yew::prelude::*;

/// Icon types
#[derive(Clone, PartialEq)]
pub enum Icon {
    UserCircle,
    Document,
    PencilSquare,
    Bookmark,
    Tag,
    XMark,
    ChevronRight,
}

impl Icon {
    fn as_svg_str(&self) -> &'static str {
        match self {
            Icon::UserCircle => include_str!("../node_modules/heroicons/16/solid/user-circle.svg"),
            Icon::Document => include_str!("../node_modules/heroicons/16/solid/document.svg"),
            Icon::PencilSquare => {
                include_str!("../node_modules/heroicons/16/solid/pencil-square.svg")
            }
            Icon::Bookmark => include_str!("../node_modules/heroicons/16/solid/bookmark.svg"),
            Icon::Tag => include_str!("../node_modules/heroicons/16/solid/tag.svg"),
            Icon::XMark => include_str!("../node_modules/heroicons/16/solid/x-mark.svg"),
            Icon::ChevronRight => {
                include_str!("../node_modules/heroicons/16/solid/chevron-right.svg")
            }
        }
    }
}

impl ToHtml for Icon {
    fn to_html(&self) -> Html {
        Html::from_html_unchecked(AttrValue::from(self.as_svg_str()))
    }
}
