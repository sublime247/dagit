use bdk::prelude::*;
use by_components::icons::edit;
#[component]
pub fn SearchInput(placeholder: String, on_change: EventHandler<String>) -> Element {
    rsx! {
        div { class: "relative flex-1 mr-4",
            div { class: "absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none",
                edit::Search { class: "[&>path]:stroke-white [&>circle]:stroke-white" }
            }
            input {
                class: "text-[15px]/[23px]  border border-neutral-80 outline-none text-white hover:border-primary focus:border-primary aria-invalid:border-pink placeholder-neutral-800 disabled:!border-neutral-80 pl-10 p-2.5 w-full",
                placeholder: "{placeholder}",
                r#type: "text",
                oninput: move |e| on_change(e.value().clone()),
            }
        }
    }
}
