use bdk::prelude::*;

#[component]
pub fn SecondaryButton(
    #[props(default = "".to_string())] class: String,
    label: String,
    #[props(default = false)] disabled: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    tracing::debug!(
        "SecondaryButton: class: {}, label: {}, disabled: {}",
        class,
        label,
        disabled
    );
    rsx! {
        button {
            class: "px-4.5 py-3 border border-white text-white bg-black  active:bg-white active:text-black hover:bg-white hover:text-black disabled:border-neutral-80 disabled:text-neutral-80 disabled:bg-btn-disabled ",
            onclick: move |e| {
                if !disabled {
                    onclick.call(e);
                }
            },
            disabled,
            {label}
        }
    }
}

#[component]
pub fn PrimaryButton(
    #[props(default = "".to_string())] class: String,
    label: String,
    #[props(default = true)] disabled: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2",
            button {
                class: "px-4.5 py-3 border border-black text-white bg-black active:bg-primary/25 active:border-primary hover:bg-primary/25 hover:border-primary disabled:border-neutral-80 disabled:text-neutral-80 disabled:bg-btn-disabled",
                onclick: move |e| {
                    if !disabled {
                        onclick.call(e);
                    }
                },
                disabled,
                {label}
            }
        }
    }
}
