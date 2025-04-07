use bdk::prelude::*;

#[component]
pub fn CollectorPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Collector {agit_id}" }
    }
}
