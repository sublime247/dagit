use bdk::prelude::*;

use crate::pages::agits::_id::management::collections::i18n::CollectionNameInputModalTranslate;

#[component]
pub fn CollectionNameInputModal(
    on_back: EventHandler<()>,
    on_add: EventHandler<String>, // Changed to pass the collection name
    lang: Language,
) -> Element {
    let mut collection_name = use_signal(|| String::new());
    let tr: CollectionNameInputModalTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col",
            // Modal body
            div { class: "pb-5",
                p { class: "text-sm text-white mb-4", {tr.sub_title} }
                // Collection name input
                div { class: "mb-4",
                    label { class: "block text-sm font-medium text-popup-label mb-2",
                        {tr.collection_name}
                    }
                    input {
                        class: "w-full bg-transparent border border-popup-border text-white text-sm rounded-none p-2",
                        placeholder: tr.collection_name_placeholder,
                        value: "{collection_name}",
                        oninput: move |evt| collection_name.set(evt.value().clone()),
                    }
                }
                // Short URL input
                div {
                    label { class: "block text-sm font-medium text-popup-label mb-2",
                        {tr.short_url}
                    }
                    div { class: "flex",
                        span { class: "inline-flex items-center px-3 text-sm text-gray-400 border-popup-border border border-r-0",
                            "dagit.com/"
                        }
                        input {
                            class: "flex-1 bg-transparent border border-popup-border text-popup-border text-sm rounded-none p-2",
                            placeholder: "(collection)",
                            readonly: true,
                            value: {
                                let name = collection_name.read();
                                name.to_lowercase().replace(" ", "-")
                            },
                        }
                    }
                }
            }
            // Modal footer
            div { class: "flex items-center justify-end gap-4 p-6 border-t border-border-primary",
                button {
                    class: "px-5 py-2 text-l text-gray-400 hover:text-white",
                    onclick: move |_| on_back.call(()),
                    {tr.back_btn_text}
                }
                button {
                    class: "px-4 py-2 text-l bg-white text-black hover:bg-gray-200",
                    onclick: move |_| {
                        on_add.call(collection_name.read().clone());
                    },
                    {tr.add_btn_text}
                }
            }
        }
    }
    }
