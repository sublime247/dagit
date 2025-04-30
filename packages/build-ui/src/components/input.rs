use bdk::prelude::*;

#[component]
pub fn Input(
    label: String,
    #[props(default = "".to_string())] placeholder: String,
    value: String,
    on_change: EventHandler<String>,
    #[props(default = false)] disabled: bool,
    #[props(default = false)] invalid: bool,
    #[props(default = "".to_string())] invalid_message: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2", ..attributes,
            label { class: "text-sm/relaxed font-semibold text-neutral-70", {label} }
            input {
                "aria-invalid": invalid,
                class: "text-[15px]/[23px] border border-neutral-80 px-4 py-3 outline-none text-white hover:border-primary focus:border-primary aria-invalid:border-pink placeholder-neutral-800 disabled:!border-neutral-80",
                placeholder,
                value,
                disabled,
                oninput: move |e| on_change(e.value().clone()),
            }
            if invalid {
                span { class: "text-[15px]/[23px] text-pink", {invalid_message} }
            }
        
        }
    }
}
