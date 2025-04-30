use bdk::prelude::*;

use crate::pages::agits::_id::management::artists::i18n::ArtistTranslate;

#[component]
pub fn ConfirmRemoveArtistModal(
    show: bool,
    on_back: EventHandler<()>,
    on_remove: EventHandler<()>,
    lang: Language,
) -> Element {
    let mut terms_accepted = use_signal(|| false);
    let tr: ArtistTranslate = translate(&lang);
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
                    div { class: "bg-black border border-border-primary rounded-lg  w-full max-w-[550px] shadow-[0_0_40px_10px_rgba(255,41,144,0.5)]",
                        // Modal header
                        div { class: "flex items-center justify-between px-6 pt-6  pb-2 border-border-primary",
                            h2 { class: "text-xl font-semibold text-white", {tr.remove_popup_title} }
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
                        div { class: "px-6 pr-10 py-6",
                            p { class: "text-white",
                                {tr.remove_popup_description_1}
                            }
                            p { class: "text-white pt-1",
                                {tr.remove_popup_description_2}
                            }

                        }
                        div { class: "p-6 pt-0 flex items-start",
                        input {
                            id: "announcements",
                            class: "mr-2  h-4 w-4 border border-btn-signin checked:bg-white checked:border-black checked:text-black text-sm",
                            type: "checkbox",
                            checked: "{terms_accepted}",
                            oninput: move |evt| terms_accepted.set(evt.value().parse().unwrap_or(false)),
                        }
                            label { class: "text-sm text-popup-label mb-2",
                                {tr.remove_popup_confirm_text}
                            }
                        }

                        // Modal footer
                        div { class: "flex items-center justify-between gap-4 p-6 border-border-primary",
                            button {
                                class: "px-10 py-2 text-l text-popup-text hover:text-white border border-white",
                                onclick: move |_| on_back.call(()),
                               { tr.cancel_btn}
                            }
                            button {
                                class: "px-10 py-3 text-l bg-white  text-black hover:bg-gray-200",
                                onclick: move |_| {
                                   on_remove.call(());
                                },
                               {tr.confirm_btn}

                            }
                        }
                    }
                }
            }
    }
}

#[component]
pub fn RemoveArtistModal(
    show: bool,
    on_back: EventHandler<()>,
    on_remove: EventHandler<()>,
    lang: Language,
) -> Element {
    let mut collection_name = use_signal(|| String::new());
    let tr: ArtistTranslate = translate(&lang);
    let mut terms_accepted = use_signal(|| false);

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
                onclick: move |e| e.stop_propagation(),
                div { class: "bg-popup-bg border border-border-primary rounded-lg  w-full max-w-md shadow-[0_0_40px_10px_rgba(255,41,144,0.5)]",
                    // Modal header
                    div { class: "flex items-center justify-between px-6 pt-6 pb-5 border-border-bg",
                        h2 { class: "text-xl font-semibold text-white",
                            {tr.remove_popup_2}
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
                        p { class: "text-sm text-white mb-4", {tr.remove_popup_2_description} }
                        // Collection name input
                        div { class: "mb-4",
                            input {
                                class: "w-full bg-transparent border border-[1px] text-white text-sm rounded-none p-2 focus:outline-none focus:border-primary",
                                    placeholder: "Artist name",
                                value: "{collection_name}",
                                oninput: move |evt| collection_name.set(evt.value().clone()),
                            }
                        }
                        // Short URL input
                        div { class: "flex items-start",
                        input {
                            id: "announcements",
                            class: "mr-2  h-4 w-4 border border-btn-signin checked:bg-white checked:border-black checked:text-black text-sm",
                            type: "checkbox",
                            checked: "{terms_accepted}",
                            oninput: move |evt| terms_accepted.set(evt.value().parse().unwrap_or(false)),
                        }
                            label { class: "text-sm text-popup-label mb-2",
                                {tr.remove_popup_confirm_text_2}
                            }
                        }
                    }


                      // Modal footer
                      div { class: "flex items-center justify-between gap-4 p-6 pt-0 border-border-primary",
                      button {
                          class: "px-10 py-2 text-l text-popup-text hover:text-white border border-white",
                          onclick: move |_| on_back.call(()),
                         { tr.cancel_btn}
                      }
                      button {
                          class: "px-10 py-3 text-l bg-white  text-black hover:bg-gray-200",
                          onclick: move |_| {
                             on_remove.call(());
                          },
                         {tr.confirm_btn}

                      }
                  }
                }
            }
        }
    }
}

#[component]
pub fn SuccessModal(
    show: bool,
    on_back: EventHandler<()>,
    on_confirm: EventHandler<()>,
    lang: Language,
) -> Element {
    let tr: ArtistTranslate = translate(&lang);
    if !show {
        return rsx!(div {});
    }

    rsx! {
        // Modal backdrop
        div {
            class: "fixed inset-0 bg-opacity-50 backdrop-blur-sm z-50",
            // Modal content
            div { class: "fixed inset-0 flex items-center justify-center p-4",
                div { class: "bg-filter-bg border border-border-primary rounded-lg  w-full max-w-md text-center shadow-[0_0_40px_10px_rgba(255,41,144,0.5)]",
                          // Modal header
                          div { class: "flex items-center justify-between px-6 pt-6  pb-2 border-border-primary",
                          h2 { class: "text-xl font-semibold text-white", {tr.remove_popup_title} }
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
                    div { class: "px-6 py-2",
                        p { class: "text-white",
                            {tr.success_popup_description}
                        }
                    }
                    // Modal footer
                    div { class: "flex items-center p-6 border-border-primary",
                        button {
                            class: "flex-1 px-6 py-2 bg-white text-sm text-black hover:bg-gray-200 min-w[120px]",
                            onclick: move |_| on_confirm.call(()),
                            {tr.confirm_btn}
                        }
                    }
                }
            }
        }
    }
}
