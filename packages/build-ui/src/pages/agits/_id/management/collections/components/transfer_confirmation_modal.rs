use bdk::prelude::*;

use crate::pages::agits::_id::management::collections::i18n::TransferConfirmationModalTranslate;
#[component]
pub fn TransferConfirmationModal(
    selected_count: usize,
    on_back: EventHandler<()>,
    on_continue: EventHandler<String>, // Changed to pass the collection name
    lang: Language,
) -> Element {
    let tr: TransferConfirmationModalTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full max-w-md",
            div { class: "pr-10",
                p { class: "text-white", "The {selected_count} {tr.description} " }
            }
            // Modal footer
            div { class: "flex items-center justify-between gap-4 p-6 border-border-primary",
                button {
                    class: "px-4 py-2 text-l text-popup-text hover:text-white",
                    onclick: move |_| on_back.call(()),
                    {tr.back_btn_text}
                }
                button {
                    class: "px-10 py-3 text-l bg-white  text-black hover:bg-gray-200",
                    onclick: move |_| {
                        on_continue.call(String::new());
                    },
                    {tr.continue_btn_text}
                }
            }
        }
    }
}
