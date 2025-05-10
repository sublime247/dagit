use bdk::prelude::{by_components::icons::{arrows, security},  *};
use crate::{components::{dropdown_input::DropdownInput, input::{BottomBorderInput, Input2}, tab_button::TabButton}, pages::agits::_id::management::artworks::{controllers::Controller, new::i18n::NewArtworkPageTranslate}};



#[derive(Clone, Debug, PartialEq)]
pub enum ActiveTab{
    ItemDetails,
    ArtInfo
}
#[component]
pub fn NewArtworkPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    let mut ctrl = Controller::new(lang, agit_id)?;
    let mut active_tab = use_signal(||ActiveTab::ItemDetails);
    let tr: NewArtworkPageTranslate = translate(&lang);

    rsx! {
        div { class: "w-full min-h-screen bg-background h-full flex text-white items-center",
            div { class: "flex flex-col w-full h-full  p-6",
                div { class: "flex items-center mb-6",
                    div { onclick: move |_| ctrl.open_artwork_page(),
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "h-6 w-6",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M15 19l-7-7 7-7",
                            }
                        }
                    }
                    BottomBorderInput {
                        label: "Artwork name".to_string(),
                        placeholder: "Input Title".to_string(),
                        value: "".to_string(),
                        on_change: move |_value| {},
                        disabled: false,
                        invalid: false,
                        invalid_message: "".to_string(),
                    }
                    div {
                        security::Lock2 {
                            class: "ml-2 [&>path]:stroke-white [&>circle]:stroke-white",
                            height: 20,
                            width: 20,
                        }
                    }
                }
                div { class: "mt-4 mb-8",
                    div { class: "flex space-x-8 text-sm font-medium",
                        TabButton {
                            label: "Item Details".to_string(),
                            is_active: *active_tab.read() == ActiveTab::ItemDetails,
                            onclick: move |_| active_tab.set(ActiveTab::ItemDetails),
                        }
                        TabButton {
                            label: "Artwork Info".to_string(),
                            is_active: *active_tab.read() == ActiveTab::ArtInfo,
                            onclick: move |_| active_tab.set(ActiveTab::ArtInfo),
                        }
                    }
                }

                // Item detail Section

                div { class: "mb-8",
                    div { class: "flex items-center mb-4",
                        h2 { class: "text-xl font-semibold", {tr.artist_info} }
                        arrows::ChevronDown {
                            class: "ml-2 [&>path]:stroke-white",
                            height: 20,
                            width: 20,
                        }
                    }
                    hr { class: "mt-4 mb-4 text-neutral-80" }
                    div { class: "space-y-4 max-w-4xl",
                        Input2 {
                            label: tr.display_name_label,
                            placeholder: tr.display_name_label,
                            value: ctrl.artwork_input_field().display_name.clone(),
                            on_change: move |evt: String| {
                                ctrl.update_artwork_field("display_name".to_string(), evt.clone())
                            },
                        }
                    }
                }
                div { class: "mb-8 mt-8",
                    div { class: "flex items-center mb-4",
                        h2 { class: "text-xl font-semibold", {tr.sale_info} }
                        arrows::ChevronDown {
                            class: "ml-2 [&>path]:stroke-white",
                            height: 20,
                            width: 20,
                        }
                    }
                    hr { class: "mt-4 mb-4 text-neutral-80" }
                    div { class: "space-y-4",
                        DropdownInput {
                            label: tr.ways_to_sell,
                            placeholder: tr.ways_to_sell,
                            value: ctrl.artwork_input_field().ways_to_sell.clone(),
                            options: vec!["Trans".to_string(), "Offer".to_string()],
                            on_change: move |evt: String| {
                                ctrl.update_artwork_field("ways_to_sell".to_string(), evt.clone())
                            },
                        }
                                        //  Input2 {
                    //         label: tr.ways_to_sell,
                    //         placeholder: tr.ways_to_sell,
                    //         value: ctrl.artwork_input_field().ways_to_sell.clone(),
                    //         on_change: move |evt: String| {
                    //             ctrl.update_artwork_field("ways_to_sell".to_string(), evt.clone())
                    //         },
                    //     }
                    }
                }
            
            }
        }
    }
}