use bdk::prelude::*;

#[component]
pub fn SuccessModal(
    show: bool,
    collection_name: String,
    on_confirm: EventHandler<()>
) -> Element {
    if !show {
        return rsx!(
            div {}
        );
    }

    rsx! {
        // Modal backdrop
        div {
            class: "fixed inset-0 bg-opacity-50 backdrop-blur-sm z-50",
            // Modal content
            div { class: "fixed inset-0 flex items-center justify-center p-4",
                div { class: "bg-filter-bg border border-border-primary rounded-lg  w-full max-w-md text-center p-6 shadow-[0_0_40px_10px_rgba(255,41,144,0.5)]",
                    // Modal body
                    div { class: "px-6 py-2",
                        p { class: "text-white",
                            "The collection has been successfully created. Redirecting to the collection screen."
                        }
                    }
                    // Modal footer
                    div { class: "flex  items-center p-6 border-border-primary",
                        button {
                            class: "flex-1 px-6 py-2 bg-white text-sm text-black hover:bg-gray-200 min-w[120px]",
                            onclick: move |_| on_confirm.call(()),
                            "Confirm"
                        }
                    }
                }
            }
        }
    }
}
