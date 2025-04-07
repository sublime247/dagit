use bdk::prelude::*;

#[component]
pub fn DesignPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Design {agit_id}" }
    }
}
