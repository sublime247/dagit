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

