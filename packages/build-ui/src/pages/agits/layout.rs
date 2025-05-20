#![allow(non_snake_case)]
use crate::{pages::layout::SuspenseWrapper, routes::Route, services::user_service::UserService};
use bdk::prelude::*;

use super::Navigation;

#[component]
pub fn NavigationLayout(lang: Language, agit_id: i64) -> Element {
    let nav = use_navigator();
    let user: UserService = use_context();
    use_effect(move || {
        if user.user_info().is_none() {
            nav.push(Route::RootPage { lang });
        }
    });

    rsx! {
        SuspenseWrapper {
            div { class: "flex flex-row bg-background min-h-svh",
                Navigation { lang, agit_id }
                div { class: "py-10 px-20 w-full", Outlet::<Route> {} }
            }
        }
    }
}
