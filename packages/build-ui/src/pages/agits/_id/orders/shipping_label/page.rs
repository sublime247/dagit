use bdk::prelude::*;

#[component]
pub fn ShippingLabelPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "ShippingLabelPage {agit_id}" }
    }
}
