use bdk::prelude::*;

#[component]
pub fn ExtensionToolPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Extension tool {agit_id}" }
    }
}
