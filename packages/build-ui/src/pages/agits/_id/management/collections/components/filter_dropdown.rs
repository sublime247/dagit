use bdk::prelude::*;
use by_components::icons::arrows;

#[component]
pub fn FilterDropdown(label: String) -> Element {
    rsx! {
        div { class: "flex items-center border-b-1 border-border-primary justify-between py-2 text-sm cursor-pointer hover:bg-border-bg",
            span { "{label}" }
            arrows::ChevronDown { class: "[&>path]:stroke-white" }
        }
    }
}
