use bdk::prelude::*;
#[component]
pub fn InputField(
    label: String,
    placeholder: String,
    onInput: EventHandler<Event<FormData>>,
    value: String,
) -> Element {
  rsx! {
        div { class: "flex items-center",
            label { class: "text-sm w-40",
                span { class: "text-red-500 mr-1", "*" }
                {label}
            }
            input {
                class: "bg-border-background border border-border-primary text-white p-2.5 flex-1",
                placeholder,
                r#type: "text",
                value,
                oninput: move |e: Event<FormData>| onInput.call(e),
            }
        }
    }
}
