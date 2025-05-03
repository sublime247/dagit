use bdk::prelude::*;

use crate::pages::agits::_id::management::collections::i18n::SuccessModalTranslate;

#[component]
pub fn SuccessModal(collection_name: String, on_confirm: EventHandler<()>, lang:Language) -> Element {
    let tr:SuccessModalTranslate = translate(&lang);

    rsx! {
        div { class: "max-w-md flex flex-col w-full",
            // Modal body
            div { class: "py-2",
                p { class: "text-white", {tr.description} }
            }
            // Modal footer
            div { class: "flex  items-center p-6 border-border-primary",
                button {
                    class: "flex-1 px-6 py-2 bg-white text-sm text-black hover:bg-gray-200 min-w[120px]",
                    onclick: move |_| on_confirm.call(()),
                    {tr.confirm_btn_text}
                }
            }
        }
    }
    }   
