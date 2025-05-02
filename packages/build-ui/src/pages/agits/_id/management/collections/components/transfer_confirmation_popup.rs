use bdk::prelude::*;
#[component]
pub fn TransferConfirmationModal(
    show: bool,
    selected_count: usize,
    on_back: EventHandler<()>,
    on_continue: EventHandler<String>, // Changed to pass the collection name
) -> Element {
    if !show {
        return rsx!(div {});
    }

    rsx! {
        // Modal backdrop with purple glow effect
        div {
            class: "fixed inset-0 bg-opacity-50 backdrop-blur-sm z-50",

            onclick: move |_| on_back.call(()),
            // Modal content
            div {
                class: "fixed inset-0 flex items-center justify-center p-4",
                onclick: move |e| e.stop_propagation(),
                div { class: "bg-black border border-border-primary rounded-lg  w-full max-w-md shadow-[0_0_40px_10px_rgba(255,41,144,0.5)]",
                    // Modal header
                    div { class: "flex items-center justify-between px-6 pt-6  pb-2 border-border-primary",
                        h2 { class: "text-xl font-semibold text-white", "Transfer Artwork" }
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
                    div { class: "px-6 pr-10",
                        p { class: "text-white",
                            "The {selected_count} selected artworks are already included in another collection. Would you like to transfer the artworks to the designated collection?"
                        }
                    }
                    // Modal footer
                    div { class: "flex items-center justify-between gap-4 p-6 border-border-primary",
                        button {
                            class: "px-4 py-2 text-l text-popup-text hover:text-white",
                            onclick: move |_| on_back.call(()),
                            "Back"
                        }
                        button {
                            class: "px-10 py-3 text-l bg-white  text-black hover:bg-gray-200",
                            onclick: move |_| {
                                on_continue.call(String::new());
                            },
                            "Continue"
                        }
                    }
                }
            }
        }
    }
}
