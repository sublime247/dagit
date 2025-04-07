use bdk::prelude::*;

#[component]
pub fn FaqPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Faq {agit_id}" }
    }
}
