use crate::elements::{button::Button, Appearance};
use common::icons::outline::Shape as Icon;
use common::icons::Icon as IconElement;
use dioxus::prelude::*;

#[allow(dead_code)]
#[derive(PartialEq, Props)]
pub struct Props {
    #[props(optional)]
    text: Option<String>,
    #[props(optional)]
    link: Option<String>,
}

// Custom window titlebar with custom window controls
#[allow(dead_code)]
#[allow(non_snake_case)]
pub fn TitleBar<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    let desktop = dioxus_desktop::use_window(cx);
    let text = cx.props.text.clone().unwrap_or_default();
    let link_opt = cx.props.text.clone();
    cx.render(rsx!(
        div {
            id: "titlebar",
            aria_label: "Title Bar",
            onmousedown: move |_| { desktop.drag(); },
            div {
                id: "titlebar-text",
                aria_label: "titlebar-text",
                IconElement {
                    icon: Icon::Beaker,
                },
                p {
                    div {
                        (cs.props.link.is_some()).then(||
                            onclick: move |_| {
                                let _ = open::that(link.clone());
                            },
                        ),
                        "{text}"
                    }
                },
            },
            div {
                class: "controls",
                Button {
                    aria_label: "minimize-button".into(),
                    icon: Icon::Minus,
                    appearance: Appearance::Transparent,
                    onpress: move |_| {
                        desktop.set_minimized(true);
                    }
                },
                Button {
                    aria_label: "square-button".into(),
                    icon: Icon::Square2Stack,
                    appearance: Appearance::Transparent,
                    onpress: move |_| {
                        desktop.set_maximized(!desktop.is_maximized());
                    }
                },
                Button {
                    aria_label: "close-button".into(),
                    icon: Icon::XMark,
                    appearance: Appearance::Transparent,
                    onpress: move |_| {
                        desktop.close();
                    }
                },
            },
        }
    ))
}
