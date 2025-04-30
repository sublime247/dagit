use bdk::prelude::*;

#[component]
pub fn RemoveArtistModal(
    show: bool,
    on_back: EventHandler<()>,
    on_remove: EventHandler<()>,
) -> Element {
    let mut receive_announcements = use_signal(|| false);

    if !show {
        return rsx!(div {});
    }

    rsx! {
       div {
        class: "fixed inset-0 bg-opacity-50 backdrop-blur-sm z-50",
        onclick: move |_| on_back.call(()),
        // Modal content
        div {
            class: "fixed inset-0 flex items-center justify-center p-4",
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

           

           div { class:"px-6 pr-10",
            p { class: "text-white",
                "Removing an artist is irreversible. Once removed, all artworks associated with the artist will be converted to `Pending Sale ` status and will no longer be available for sale from that point forward."
            } 
            p {  
                class: "text-white pt-4",
                "Are you sure you want to remove this artist?"
            }
            div {
                class: "flex items-center mb-4",
                input {
                    id: "announcements",
                    class: "mr-2 h-4 w-4 border border-btn-signin checked:bg-white checked:border-black checked:text-black text-sm",
                    type: "checkbox",
                    checked: "{receive_announcements}",
                    oninput: move |evt| receive_announcements.set(evt.value().parse().unwrap_or(false)),
                }
                label {
                    class: "text-sm text-popup-label",
                   "I acknowledge and accept the permanent removal of the artist and the legal consequences of this action, including the loss of associated rights, royalties, and future sales opportunities. I understand that this action is final and cannot be undone.",
                }
            }
           }
           

        }
        }
    }
}


