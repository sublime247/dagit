use bdk::prelude::*;

#[component]
pub fn DaoPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Dao {agit_id}" }
    }
}
