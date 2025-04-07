use bdk::prelude::*;

#[component]
pub fn ReportPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Report {agit_id}" }
    }
}
