use bdk::prelude::by_components::icons::{arrows, edit, layouts, settings, validations};
use bdk::prelude::*;

use crate::pages::agits::_id::management::artists::_id::components::ArtistTable;
use crate::pages::agits::_id::management::artists::controllers::Controller;
use crate::pages::agits::_id::management::artists::i18n::ArtistTranslate;
use crate::pages::agits::_id::management::components::NftTable;
use crate::routes::Route;
#[component]
pub fn ArtistDetailPage(lang: Language, agit_id: ReadOnlySignal<i64>, artist_id: i64) -> Element {
    let tr: ArtistTranslate = translate(&lang);
    let mut view_mode = use_signal(|| "table");
    let ctrl = Controller::new(lang, agit_id)?;
    let artist_assets = ctrl.artist_asset();
    rsx! {
        div {  class: "w-full min-h-screen bg-background h-full flex text-white justify-center items-center",
            div {
                class: "flex flex-col w-full h-full text-white",

                // Header with back button and collector info
                div{ class: "flex flex-col mb-6",
                div {
                    class: "flex items-center",
                    Link {
                        to: Route::ArtistPage { lang, agit_id: agit_id() },
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
                                d: "M15 19l-7-7 7-7"
                            }
                        }
                    }


                    div{
                        class: "flex items-center",

                         h1 {
                        class: "text-xl font-bold",
                        {format!("{{Artist {}}}",artist_id)}
                      }
                }
            }
            div {
                class: "text-sm text-gray-400 m-2",
                "52 Total Artworks"
            }
    }


                                  // Search and view controls
                                  div {
                                    class: "flex flex-col md:flex-row justify-between mb-6 gap-4",

                                    // View mode buttons
                                    div {
                                        class: "flex space-x-2",



                                             // Filter dropdown
                                             button {
                                                class: "p-2 border border-border-primary text-white w-full sm:w-auto",
                                                // onclick: move |_| show_filters.toggle(),
                                                settings::Sliders { class: "[&>path]:stroke-white" }
                                            }

                                            button {
                                                class: format!("p-2 border {} text-white w-full sm:w-auto",
                                                    if *view_mode.read() == "nftImages" { "border-white" } else { "border-border-primary" }
                                                ),
                                                onclick: move |_| {
                                                    if *view_mode.read() == "table" {
                                                        view_mode.set("nftImages");
                                                    } else {
                                                        view_mode.set("table");
                                                    }
                                                },
                                                layouts::Window{ class: "[&>path]:stroke-white" }
                                            }




                                        // All dropdown
                                        div { class: "relative",
                                        div { class: "absolute inset-y-0 right-0 flex items-center pr-3 pointer-events-none",
                                            arrows::ChevronDown{ class: "[&>path]:stroke-white", height: 20, width: 20 }
                                        }
                                        input {
                                            class: "bg-border-background border border-border-primary text-white text-sm rounded-none block w-full pl-3 p-2.5",
                                            placeholder: "All",
                                            r#type: "text",
                                        }
                                    }

                                    }

                                    // Search
                                    div {
                                        class: "relative flex-1",
                                        div { class: "absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none",
                                        edit::Search { class: "[&>path]:stroke-white [&>circle]:stroke-white" }
                                    }
                                    input {
                                        class: "bg-border-background border border-border-primary text-white text-sm rounded-none block w-full pl-10 p-2.5",
                                        placeholder: "Search",
                                        r#type: "text",
                                    }
                                    }
                                    button {
                                        class: "bg-border-background border border-white text-white px-4 py-2 flex items-center justify-center w-full sm:w-auto",
                                        onclick: move |_| {
                                            // ctrl.open_new_collection_popup();
                                        },
                                        validations::Add { class: "mr-3 [&>path]:stroke-white [&>circle]:stroke-white" }
                                        {tr.new_artist}
                                    }

                                }

                                    // Assets table
                    div {
                        class: "overflow-x-auto flex-1",
                       { if *view_mode.read()=="nftImages" {
                            rsx!{
                                NftTable{assets: artist_assets.clone(), lang}
                            }
                        }else{
                        rsx!{
                            ArtistTable {assets: artist_assets.clone(), lang}
                        }
                            }
                        }
                    }
             }


    }

    }
}

#[component]
pub fn NewArtistPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    let _tr: ArtistTranslate = translate(&lang);
    let ctrl = Controller::new(lang, agit_id)?;

    // Form state signals
    let mut display_name = use_signal(String::new);
    let mut social_media = use_signal(String::new);
    let mut medium = use_signal(String::new);
    let mut theme = use_signal(String::new);
    let mut art_style = use_signal(String::new);
    let mut introduction = use_signal(String::new);
    let mut biography = use_signal(String::new);
    let _profile_picture = use_signal(|| None::<String>);

    // Handle form submission
    let handle_save = move |_| {
       // todo: similate api this
    };

    rsx! {
        div { class: "w-full min-h-screen bg-background h-full flex text-white items-center",
            div { class: "flex flex-col w-full h-full max-w-4xl p-6",
                // Header with title and back button
                div { class: "flex items-center mb-6",
                    Link {
                        to: Route::ArtistPage { lang, agit_id: agit_id() },
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
                                d: "M15 19l-7-7 7-7"
                            }
                        }
                    }

                    h1 { class: "text-2xl font-bold", "Add Artist" }
                }

                p { class: "text-gray-400 mb-8",
                    "Register a new artist on the platform. Showcase their work, share exhibition history, and connect with curators and galleries."
                }

                // Artist Info Section
                div { class: "mb-8",
                div { class: "flex items-center mb-4",
                h2 { class: "text-xl font-semibold", "Artist Info" }
                arrows::ChevronDown { class: "ml-2 [&>path]:stroke-white", height: 20, width: 20 }
            }

            // Form fields
            div { class: "space-y-4",
                div { class: "flex items-center",
                    label { class: "text-sm w-40",
                        span { class: "text-red-500 mr-1", "*" }
                        "Display Name or Email"
                    }
                    input {
                        class: "bg-border-background border border-border-primary text-white p-2.5 flex-1",
                        placeholder: "Input",
                        r#type: "text",
                        oninput: move |evt| display_name.set(evt.value().clone())
                    }
                }

                div { class: "flex items-center",
                    label { class: "text-sm w-40", "Social Media" }
                    input {
                        class: "bg-border-background border border-border-primary text-white p-2.5 flex-1",
                        placeholder: "Input",
                        r#type: "text",
                        oninput: move |evt| social_media.set(evt.value().clone())
                    }
                }

                div { class: "flex items-center",
                    label { class: "text-sm w-40",
                        span { class: "text-red-500 mr-1", "*" }
                        "Medium"
                    }
                    input {
                        class: "bg-border-background border border-border-primary text-white p-2.5 flex-1",
                        placeholder: "Input",
                        r#type: "text",
                        oninput: move |evt| medium.set(evt.value().clone())
                    }
                }

                div { class: "flex items-center",
                    label { class: "text-sm w-40",
                        span { class: "text-red-500 mr-1", "*" }
                        "Theme"
                    }
                    input {
                        class: "bg-border-background border border-border-primary text-white p-2.5 flex-1",
                        placeholder: "Input",
                        r#type: "text",
                        oninput: move |evt| theme.set(evt.value().clone())
                    }
                }

                div { class: "flex items-center",
                    label { class: "text-sm w-40",
                        span { class: "text-red-500 mr-1", "*" }
                        "Art Style"
                    }
                    input {
                        class: "bg-border-background border border-border-primary text-white p-2.5 flex-1",
                        placeholder: "Input",
                        r#type: "text",
                        oninput: move |evt| art_style.set(evt.value().clone())
                    }
                }
            }


                        // Profile Picture Upload
                        div { class: "flex flex-col mt-8",
                            label { class: "mb-1 text-sm", "Profile Picture" }
                            div {
                                class: "border border-dashed border-green-500 flex flex-col items-center justify-center text-center text-center w-[340px] h-[340px]",

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
                                        d: "M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                                    }
                                }
                                p { class: "text-sm text-gray-400",
                                    "Please drag and drop your file here."
                                }
                                p { class: "text-xs text-gray-500",
                                    "One or more files (JPG, PNG)"
                                }
                            }
                            p { class: "text-xs text-gray-500 mt-1",
                                "Maximum resolution: 3000px × 3000px (square format)"
                            }
                            p { class: "text-xs text-gray-500",
                                "Uploads are allowed under 10MB"
                            }
                        }
                }

                // Introduction Section
                div { class: "mb-8 mt-8",
                    div { class: "flex items-center mb-4",
                        h2 { class: "text-xl font-semibold", "Introduction" }
                        arrows::ChevronDown { class: "ml-2 [&>path]:stroke-white", height: 20, width: 20 }
                    }

                    textarea {
                        class: "bg-border-background border border-border-primary text-white p-4 w-full h-32",
                        placeholder: "Please feel free to describe your artistic work, style, and creative philosophy. It would be wonderful if you could share the message behind your art and the unique aspects of your work.",
                        oninput: move |evt| introduction.set(evt.value().clone())
                    }
                }

                // Biography Section
                div { class: "mt-8",
                    div { class: "flex items-center mb-4",
                        h2 { class: "text-xl font-semibold", "Biography" }
                        arrows::ChevronDown { class: "ml-2 [&>path]:stroke-white", height: 20, width: 20 }
                    }

                    textarea {
                        class: "bg-border-background border border-border-primary text-white p-4 w-full h-32",
                        placeholder: "Please provide a brief introduction about yourself, including your background, career, and areas of expertise. Sharing who you are and what experiences have shaped you would be greatly appreciated.",
                        oninput: move |evt| biography.set(evt.value().clone())
                    }
                }

                // Action buttons
                div { class: "flex justify-end space-x-4 mt-8",
                    button {
                        class: "border border-white text-white px-6 py-2 flex items-center justify-center",
                        onclick: handle_save,
                        "Save"
                    }

                    button {
                        class: "border border-white text-white px-6 py-2 flex items-center justify-center",
                        onclick: move |_| {
                            tracing::debug!(
                                "Display name: {:?}, social media: {:?}, introduction: {:?}, biography: {:?}",
                                display_name.read().clone(),
                                social_media.read().clone(),
                                introduction.read().clone(),
                                biography.read().clone(),
                            );
                                ctrl.create_artist(
                                 display_name.read().clone(),
                                 display_name.read().clone(),
                                 social_media.read().clone(),
                                 introduction.read().clone(),
                                 biography.read().clone(),
                                );
                        },
                        validations::Add { class: "mr-2 [&>path]:stroke-white [&>circle]:stroke-white" }
                        "Add Artwork"
                    }
                }
            }
        }
    }
}
