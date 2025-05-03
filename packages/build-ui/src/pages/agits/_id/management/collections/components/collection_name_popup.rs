use bdk::prelude::*;

#[component]
pub fn CollectionNameModal(
    show: bool,
    on_back: EventHandler<()>,
    on_add: EventHandler<String>, // Changed to pass the collection name
) -> Element {
    let mut collection_name = use_signal(|| String::new());

    if !show {
        return rsx!(
            div {}
        );
    }

    rsx! {

        div {
            class: "fixed inset-0 bg-opacity-50 backdrop-blur-sm z-50",

            onclick: move |_| on_back.call(()),
            // Modal content
            div {
                class: "fixed inset-0 flex items-center justify-center p-4",
                onclick: move |e| e.stop_propagation(),
                div { class: "bg-popup-bg border border-border-primary rounded-lg  w-full max-w-md shadow-[0_0_40px_10px_rgba(255,41,144,0.5)]",
                    // Modal header
                    div { class: "flex items-center justify-between px-6 pt-6 border-border-bg",
                        h2 { class: "text-xl font-semibold text-white",
                            "What is the name of collection?"
                        }
                        button {
                            class: "text-gray-400 hover:text-white",
                            onclick: move |_| on_back.call(()),
                            svg {
                                class: "w-6 h-6",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                stroke_width: "2",
                                fill: "none",
                                path { d: "M6 18L18 6M6 6l12 12" }
                            }
                        }
                    }
                    // Modal body
                    div { class: "px-6 pb-5",
                        p { class: "text-sm text-white mb-4", "Choose a collection name." }
                        // Collection name input
                        div { class: "mb-4",
                            label { class: "block text-sm font-medium text-popup-label mb-2",
                                "Collection Name"
                            }
                            input {
                                class: "w-full bg-transparent border border-popup-border text-white text-sm rounded-none p-2",
                                placeholder: "Collection name",
                                value: "{collection_name}",
                                oninput: move |evt| collection_name.set(evt.value().clone()),
                            }
                        }
                        // Short URL input
                        div {
                            label { class: "block text-sm font-medium text-popup-label mb-2",
                                "Short URL"
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
                            "Back"
                        }
                        button {
                            class: "px-4 py-2 text-l bg-white text-black hover:bg-gray-200",
                            onclick: move |_| {
                                on_add.call(collection_name.read().clone());
                            },
                            "Add Collection"
                        }
                    }
                }
            }
        }
    }
}
