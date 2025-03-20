use bdk::prelude::*;

#[component]
pub fn RootPage(lang: Language) -> Element {
    rsx! {
        div { class: "text-white", "Home" }
    }
}
