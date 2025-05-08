#[allow(unused_imports)]
use crate::{
    components::{search_filter_bar::SearchFilterBar, filter_sidebar::FilterSidebar},
    routes::Route,
};


use super::i18n::CollectionTranslate;
//FIXME: Use Collection in "packages/models/table/collection.rs"
use super::controllers::Controller;
use bdk::prelude::*;
use by_components::icons::{arrows, validations};
#[allow(unused_variables)]
#[component]
pub fn CollectionPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    let tr: CollectionTranslate = translate(&lang);
    //FIXME: Logics Should be implemented in Controller
    let ctrl = Controller::new(lang, agit_id)?;

    //FIXME: Use PopupService(ex. popup.open)
    let mut show_filters = use_signal(|| false);

    let collections = ctrl.collections();
    // Function to simulate API call for creating a collection

    rsx! {
        div { class: "w-full min-h-screen bg-background h-full flex text-white justify-center items-center",
            // Main content
            div { class: "flex flex-col w-full h-full",
                h1 { class: "text-2xl sm:text-2xl font-bold font-Pretendard",
                    "{tr.collections} {agit_id}"
                }
                p { class: "text-sm  sm:text-sm text-gray-400", "1,120 Total Collections" }
                // Search and Filter Bar
                SearchFilterBar {
                    show_filter_btn: true,
                    on_filter_click: move |_| {
                        show_filters.toggle();
                    },
                    placeholder: tr.search_by_title,
                    on_add_click: move |_| {
                        ctrl.open_new_collection_modal();
                    },
                    // Fixme:
                    on_search_change: move |search_text| {},
                    on_search: move |search_text| {},
                    show_add_btn: true,
                    add_btn_text: tr.new_collection,
                }

                // Content area (FilterSidebar and Table)
                div { class: "flex flex-col md:flex-row flex-1 w-full",
                    // FilterSidebar (hidden on small screens unless toggled)
                    if *show_filters.read() {
                        div {
                            class: format!(
                                "w-64 bg-background border-r border-border-primary fixed inset-y-0 left-0 z-40 transform {} md:relative md:z-auto md:translate-x-0 transition-transform duration-300",
                                if *show_filters.read() { "translate-x-0" } else { "-translate-x-full" },
                            ),
                            FilterSidebar {
                                on_artist_change: move |artist| {},
                                value: "".to_string(),
                                attribute_value: "".to_string(),
                            }
                        }
                    }
                    // Table body
                    div { class: "flex-1 overflow-auto",
                        table { class: "w-full text-sm text-left border-collapse min-w-[800px]",
                            // Table header
                            thead { class: "text-xs uppercase bg-table-background border-b border-border-primary",
                                tr {
                                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            span { "#" }
                                            arrows::UpDown {
                                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                                height: 18,
                                                width: 18,
                                            }
                                        }
                                    }
                                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            span { {tr.collection} }
                                            arrows::UpDown {
                                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                                height: 18,
                                                width: 18,
                                            }
                                        }
                                    }
                                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            span { {tr.floor_price} }
                                            arrows::UpDown {
                                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                                height: 18,
                                                width: 18,
                                            }
                                        }
                                    }
                                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            span { {tr.floor_change} }
                                            arrows::UpDown {
                                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                                height: 18,
                                                width: 18,
                                            }
                                        }
                                    }
                                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            span { {tr.volume_change} }
                                            arrows::UpDown {
                                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                                height: 18,
                                                width: 18,
                                            }
                                        }
                                    }
                                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            span { {tr.volume} }
                                            arrows::UpDown {
                                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                                height: 18,
                                                width: 18,
                                            }
                                        }
                                    }
                                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            span { {tr.owners} }
                                            arrows::UpDown {
                                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                                height: 18,
                                                width: 18,
                                            }
                                        }
                                    }
                                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            span { {tr.stock} }
                                            arrows::UpDown {
                                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                                height: 18,
                                                width: 18,
                                            }
                                        }
                                    }
                                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            span { {tr.status} }
                                            arrows::UpDown {
                                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                                height: 18,
                                                width: 18,
                                            }
                                        }
                                    }
                                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            span { "" }
                                            validations::Extra {
                                                class: "[&>circle]:stroke-white",
                                                height: 18,
                                            }
                                        }
                                    } // For the actions column
                                }
                            }
                            // Table body
                            tbody {
                                {
                                    collections
                                        .into_iter()
                                        .enumerate()
                                        .map(|(index, collection)| {
                                            rsx! {
                                                tr {
                                                    key: "{index}",
                                                    class: "border-b border-border-primary",
                                                    onclick: move |_| {
                                                        use_navigator()
                                                            .push(Route::CollectionDetailPage {
                                                                lang: lang,
                                                                agit_id: *agit_id.read(),
                                                                collection_id: collection.id as i64,
                                                            });
                                                    },
                                                    // Table row content...
                                                    // (Keeping the existing table row code)
                                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", "{collection.id}" }
                                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                                        div { class: "flex items-center",
                                                            div { class: "w-6 h-6 sm:w-8 sm:h-8 bg-border-primary mr-2" }
                                                            span { "{collection.title}" }
                                                            // Verified icon
                                                            svg {
                                                                view_box: "0 0 24 24",
                                                                width: "16",
                                                                height: "16",
                                                                fill: "#10b981",
                                                                class: "ml-1",
                                                                path { d: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" }
                                                            }
                                                        }
                                                    }
                                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                                        div { "{collection.floor_price_eth} ETH" }
                                                        div { class: "text-xs text-gray-400", "$ {collection.floor_price_eth}" }
                                                    }
                                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                                        div { "{collection.floor_change_eth} ETH" }
                                                        div { class: "text-xs text-gray-400", "$ {collection.floor_change_eth}" }
                                                    }
                                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                                        div { class: "flex items-center space-x-4 p-l-4",
                                                            div { class: "flex flex-col",
                                                                div { class: "text-green-500", "+ {collection.volume_change_24h}%" }
                                                                div { class: "text-xs text-gray-400", "24h" }
                                                            }
                                                            div { class: "flex flex-col",
                                                                div { class: "text-red-500", "{collection.volume_change_7d}%" }
                                                                div { class: "text-xs text-gray-400", "7d" }
                                                            }
                                                        }
                                                    }
                                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                                        div { "{collection.volume_eth} ETH" }
                                                        div { class: "text-xs text-gray-400", "$ {collection.volume_eth}" }
                                                    }
                                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", "{collection.owners}" }
                                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", "{collection.owners}" }
                                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", "{collection.status}" }
                                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                                        button { class: "text-gray-400 hover:text-white",
                                                            validations::Extra { class: "[&>circle]:stroke-white", height: 18 }
                                                        }
                                                    }
                                                }
                                            }
                                        })
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
