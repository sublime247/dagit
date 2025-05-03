use bdk::prelude::by_components::icons::{arrows, other_devices, settings};
use bdk::prelude::*;

use crate::components::search_filter_bar::SearchFilterBar;
use crate::pages::agits::_id::management::artists::_id::components::{ArtistTable, InputField};
use crate::pages::agits::_id::management::artists::controllers::Controller;
use crate::pages::agits::_id::management::artists::i18n::ArtistTranslate;
use crate::pages::agits::_id::management::components::NftTable;
use crate::routes::Route;
#[component]
#[allow(unused_variables)]
pub fn ArtistDetailPage(lang: Language, agit_id: ReadOnlySignal<i64>, artist_id: i64) -> Element {
    let tr: ArtistTranslate = translate(&lang);
    let mut view_mode = use_signal(|| "table");
    let ctrl = Controller::new(lang, agit_id)?;
    let artist_assets = ctrl.artist_asset();
    rsx! {
        div { class: "w-full min-h-screen bg-background h-full flex text-white justify-center items-center",
            div { class: "flex flex-col w-full h-full text-white",

                // Header with back button and collector info
                div { class: "flex flex-col mb-6",
                    div { class: "flex items-center",
                        Link {
                            to: Route::ArtistPage {
                                lang,
                                agit_id: agit_id(),
                            },
                            class: "text-gray-400 hover:text-white ",
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


                        div { class: "flex items-center",

                            h1 { class: "text-xl font-bold", {format!("{{Artist {}}}", artist_id)} }
                        }
                    }
                    div { class: "text-sm text-gray-400 m-2", "52 Total Artworks" }
                }


                // Search and view controls

                SearchFilterBar {
                    placeholder: tr.search_by_title,
                    show_filter_btn: true,
                    on_filter_click: move |_| {},
                    on_search_change: move |search_text| {},
                    on_search: move |search_text| {},
                    show_all_filter_field: true,
                    show_art_btn: true,
                    on_view_mode_click: move |_| {
                        view_mode.set(if *view_mode.read() == "table" { "nftImages" } else { "table" });
                    },
                    show_add_btn: true,
                    add_btn_text: tr.new_artist,
                    on_add_click: move |_| {},
                    remove_btn_text: tr.remove_artist,
                }

                // Assets table
                div { class: "overflow-x-auto flex-1",
                    {
                        if *view_mode.read() == "nftImages" {
                            rsx! {
                                NftTable { assets: artist_assets.clone(), lang }
                            }
                        } else {
                            rsx! {
                                ArtistTable {
                                    assets: artist_assets.clone(),
                                    lang,
                                    agit_id: agit_id(),
                                    artist_id,
                                }
                            }
                        }
                    }
                }
            }
        

        }
    }
}

#[component]
pub fn EditArtistPage(
    lang: Language,
    agit_id: ReadOnlySignal<i64>,
    artist_id: ReadOnlySignal<i64>,
) -> Element {
    let tr: ArtistTranslate = translate(&lang);
    let mut ctrl = Controller::new(lang, agit_id)?;
    let _profile_picture = use_signal(|| None::<String>);
    let mut is_dropdown_open = use_signal(|| false);

    // Handle form submission
    let handle_save = move |_| {
        // todo: similate api this
    };

    rsx! {
        div { class: "w-full min-h-screen bg-background h-full flex text-white items-center",
            div { class: "flex flex-col w-full h-full p-6",
                // Header with title and back button
                div { class: "flex items-center mb-6",
                    Link {
                        to: Route::ArtistDetailPage {
                            lang,
                            agit_id: agit_id(),
                            artist_id: artist_id(),
                        },
                        class: "text-gray-400 hover:text-white mr-4",
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
                    div { class: "flex justify-between w-full",
                        h1 { class: "text-2xl font-bold", "Edit {artist_id}'s Info" }

                        div { class: "relative",
                            button {
                                onclick: move |_| {
                                    is_dropdown_open.toggle();
                                },
                                settings::Settings2 {
                                    class: "ml-2 [&>path]:stroke-white",
                                    height: 20,
                                    width: 20,
                                }
                            }
                            div {
                                class: "absolute right-0 mt-2 w-48 bg-background border border-border-primary rounded-md shadow-lg z-1 hidden aria-dropdown-open:block",
                                "aria-dropdown-open": is_dropdown_open,
                                div { class: "py-1",
                                    a {
                                        // to:Route::ArtistPage { lang, agit_id: agit_id() },
                                        onclick: move |_| {
                                            ctrl.remove_artist_popup();
                                        },
                                        class: "block px-4 py-2 text-sm text-white hover:bg-gray-700 hover:text-white",
                                        "Remove Artist"
                                    }
                                    Link {
                                        to: Route::ArtistPage {
                                            lang,
                                            agit_id: agit_id(),
                                        },
                                        class: "block px-4 py-2 text-sm text-white hover:bg-gray-700 hover:text-white",
                                        "Edit Artist Info"
                                    }
                                    Link {
                                        to: Route::ArtistPage {
                                            lang,
                                            agit_id: agit_id(),
                                        },
                                        class: "block px-4 py-2 text-sm text-white hover:bg-gray-700 hover:text-white",
                                        "Delete Artist"
                                    }
                                }
                            }
                        
                        }
                    



                    }
                }

                p { class: "text-gray-400 mb-8", "Joined Feb 20, 2025" }

                // Artist Info Section
                div { class: "mb-8",
                    div { class: "flex items-center mb-4",
                        h2 { class: "text-xl font-semibold", "Artist Info" }
                        arrows::ChevronDown {
                            class: "ml-2 [&>path]:stroke-white",
                            height: 20,
                            width: 20,
                        }
                    }

                    // Form fields
                    div { class: "space-y-4 max-w-4xl",

                        InputField {
                            label: tr.input_artist_name,
                            placeholder: tr.input_artist_name_placeholder,
                            value: ctrl.artist_input_field().display_name.clone(),
                            onInput: move |evt: Event<FormData>| {
                                ctrl.update_artist_field("display_name".to_string(), evt.value().clone())
                            },
                        }

                        InputField {
                            label: tr.social_media_label,
                            placeholder: tr.social_media_label,
                            value: ctrl.artist_input_field().social_media.clone(),
                            onInput: move |evt: Event<FormData>| {
                                ctrl.update_artist_field("social_media".to_string(), evt.value().clone())
                            },
                        }

                        InputField {
                            label: tr.medium,
                            placeholder: tr.medium,
                            value: ctrl.artist_input_field().medium.clone(),
                            onInput: move |evt: Event<FormData>| {
                                ctrl.update_artist_field("medium".to_string(), evt.value().clone())
                            },
                        }

                        InputField {
                            label: tr.theme,
                            placeholder: tr.theme,
                            value: ctrl.artist_input_field().theme.clone(),
                            onInput: move |evt: Event<FormData>| {
                                ctrl.update_artist_field("theme".to_string(), evt.value().clone())
                            },
                        }

                        InputField {
                            label: tr.art_style,
                            placeholder: tr.art_style,
                            value: ctrl.artist_input_field().art_style.clone(),
                            onInput: move |evt: Event<FormData>| {
                                ctrl.update_artist_field("art_style".to_string(), evt.value().clone())
                            },
                        }
                    }
                    // Profile Picture Upload
                    div { class: "flex flex-col mt-8",
                        label { class: "mb-1 text-sm", "Profile Picture" }
                        div { class: "border border-dashed border-green-500 flex flex-col items-center justify-center text-center text-center w-[340px] h-[340px]",

                            // You would implement file upload functionality here
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                class: "h-12 w-12 text-gray-400 mb-2",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "1",
                                    d: "M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z",
                                }
                            }
                            p { class: "text-sm text-gray-400", {tr.upload_img_label_1} }
                            p { class: "text-xs text-gray-500", {tr.upload_img_label_2} }
                        }
                        p { class: "text-xs text-gray-500 mt-1", {tr.max_file_size} }
                        p { class: "text-xs text-gray-500", {tr.max_file_size_2} }
                    }
                }

                // Introduction Section
                div { class: "mb-8 mt-8 max-w-4xl",
                    div { class: "flex items-center mb-4",
                        h2 { class: "text-xl font-semibold", {tr.introduction_label} }
                        arrows::ChevronDown {
                            class: "ml-2 [&>path]:stroke-white",
                            height: 20,
                            width: 20,
                        }
                    }

                    textarea {
                        class: "bg-border-background border border-border-primary text-white p-4 w-full h-32",
                        placeholder: tr.introduction_placeholder,
                        value: "{ctrl.artist_input_field().introduction.clone()}",

                        oninput: move |evt| {
                            ctrl.update_artist_field("introduction".to_string(), evt.value().clone())
                        },
                    
                    }
                }

                // Biography Section
                div { class: "mt-8 max-w-4xl",
                    div { class: "flex items-center mb-4",
                        h2 { class: "text-xl font-semibold", {tr.biography_label} }
                        arrows::ChevronDown {
                            class: "ml-2 [&>path]:stroke-white",
                            height: 20,
                            width: 20,
                        }
                    }

                    textarea {
                        class: "bg-border-background border border-border-primary text-white p-4 w-full h-32",
                        placeholder: tr.biography_placeholder,
                        value: "{ctrl.artist_input_field().biography.clone()}",
                        oninput: move |evt| {
                            ctrl.update_artist_field("biography".to_string(), evt.value().clone())
                        },
                    }
                }

                // Action buttons
                div { class: "flex justify-end space-x-4 mt-8",
                    button {
                        class: "border border-white text-white px-6 py-2 flex items-center justify-center",
                        onclick: handle_save,
                        other_devices::Save {
                            class: "mr-2 [&>path]:stroke-white",
                            height: 20,
                            width: 20,
                        }
                        "Save"
                    }
                }
            }
        }
    }
}
