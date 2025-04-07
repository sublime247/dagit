use bdk::prelude::*;

#[component]
pub fn CollectionPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Collection {agit_id}" }
    }
}
