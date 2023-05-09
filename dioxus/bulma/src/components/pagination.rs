//! Provides a responsive, usable and flexible pagination component
//!
use crate::state::*;
use dioxus::prelude::*;
use fermi::UseAtomRef;

#[allow(non_snake_case)]
#[derive(Props)]
pub struct PaginationProps<'a> {
    #[props(!optional)]
    state: &'a UseAtomRef<GlobalState>,

    #[props(!optional)]
    route: String,

    #[props(!optional)]
    total_pages: usize,

    #[props(default = 3)]
    links_per_side: usize,
}

/// Pagination is the parent of all the pagination components and must be used
/// as the outside container for them to work correctly
///
/// ### Warning
/// * must be used inside the Router component to work correctly
///
/// ### Properties
/// * `route: String` routing key used for page lookup
/// * `total_pages: usize` total number of pages to paginate over
/// * `links_per_side: usize` number of links to show to the left and right of the current page
#[allow(non_snake_case)]
pub fn Pagination<'a>(cx: Scope<'a, PaginationProps<'a>>) -> Element {
    let state = cx.props.state;

    let (route1, route2) = (cx.props.route.clone(), cx.props.route.clone());
    let page = state.read().pagination.get_current_page(&route1);
    let max_links = cx.props.links_per_side;

    let pages_left = page.checked_sub(1).unwrap_or_default();
    let pages_right = cx.props.total_pages - page;
    let mut links_left = max_links.min(pages_left);
    // If not all left links were displayed then add them to the right side
    let links_right = max_links.min(pages_right) + max_links.checked_sub(links_left).unwrap_or_default();
    // If not all right links were displayed then add them to the left side
    links_left = links_left + max_links.checked_sub(links_right).unwrap_or_default();

    let mut prev_css = "".to_string();
    cx.render(rsx! {
        nav { class: "pagination is-right",
            if page == 1 {
                prev_css = "is-disabled".to_string();
            }
            a { class: "pagination-previous {prev_css}",
                onclick: move |_| {
                    if page - 1 > 0 {
                        state.write().pagination.set_current_page(&route1, page - 1);
                    }
                },
                "Previous"
            }
            a { class: "pagination-next",
                onclick: move |_| {
                    if page + 1 <= cx.props.total_pages {
                        state.write().pagination.set_current_page(&route2, page + 1);
                    }
                },
                "Next Page"
            }
            ul {
                class: "pagination-list",
                PaginationRange(cx, (1..=pages_left).collect(), links_left, true)
                li { a { class: "pagination-link is-current", "{page}"} }
                PaginationRange(cx, (page+1..=page+pages_right).collect(), links_right, false)
            }
        }
    })
}

/// The range may be to the left or the right of the current page.
/// * `pages` is the page range to potentially display as links for this pagination range
/// * `max` is the max number of pages to display as links for this pagination range
/// * `left` signals the optional ellipsis would be to the left
#[allow(non_snake_case)]
fn PaginationRange<'a>(
    cx: Scope<'a, PaginationProps<'a>>, mut pages: Vec<usize>, max: usize, left: bool,
) -> Element {
    cx.render(if pages.len() > max {
        if left {
            // Split off everything at index max and beyond
            // also taking into account 2 less for the last and ellipsis
            let offset = pages.len().checked_sub(max.checked_sub(2).unwrap_or_default()).unwrap_or_default();
            let right = pages.split_off(offset);
            rsx! {
                PaginationLink(cx, *pages.first().unwrap())
                PaginationEllipsis {}
                for i in (right) {
                    PaginationLink(cx, i)
                }
            }
        } else {
            // Split off everything at index max and beyond
            // also taking into account 2 less for the last and ellipsis
            let right = pages.split_off(max - 2);
            rsx! {
                for i in (pages) { PaginationLink(cx, i) }
                PaginationEllipsis {}
                PaginationLink(cx, *right.last().unwrap())
            }
        }
    } else {
        rsx! {
            for i in (pages) {
                PaginationLink(cx, i)
            }
        }
    })
}

/// PaginationLink provides a clickable pagination button
///
/// ### Properties
/// * `route: String` routing key used for page lookup
#[allow(non_snake_case)]
fn PaginationLink<'a>(cx: Scope<'a, PaginationProps<'a>>, page: usize) -> Element<'a> {
    let state = cx.props.state;

    cx.render(rsx! {
        li {
            a { class: "pagination-link",
                onclick: move |_| {
                    state.write().pagination.set_current_page(&cx.props.route, page);
                },
                format!("{page}")
            }
        }
    })
}

/// PaginationEllipsis provides an ellipsis place holder for pagination buttons
#[allow(non_snake_case)]
fn PaginationEllipsis(cx: Scope) -> Element {
    cx.render(rsx! {
        li {
            span { class: "pagination-ellipsis",
                "..."
            }
        }
    })
}
