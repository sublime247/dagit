use bdk::prelude::*;
use by_components::icons::{ settings, folder, edit };
use dioxus_popup::PopupService;

use crate::pages::collection::{ i18n::CollectionTranslate, Artwork, Collection, CollectionNameModal, FilterSidebar, NewCollectionModal, SuccessModal, TransferConfirmationModal};



#[allow(unused_variables)]
#[component]
pub fn CollectionsPage(lang: Language, agit_id:ReadOnlySignal<i64>) -> Element {
    let tr: CollectionTranslate = translate(&lang);
    let mut show_filters = use_signal(|| false);
    let mut selected_artworks = use_signal(|| Vec::<usize>::new());
    let mut new_collection_name = use_signal(|| String::new());
    let mut popup: PopupService=use_context();
    
    let collections = use_signal(|| {
        (1..15).map(|id| Collection {
            id,
            name: "(Collection Name)".to_string(),
            verified: true,
            floor_price_eth: 2.370,
            floor_price_usd: 8147.63,
            floor_change_eth: 2.370,
            floor_change_usd: 8147.63,
            volume_change_24h: 12.0,
            volume_change_7d: -8.0,
            volume_eth: 2.370,
            volume_usd: 8147.63,
            owners: "Num".to_string(),
            stock: "Num".to_string(),
            status: "Active".to_string(),
        }).collect::<Vec<_>>()
    });

    let artwork = use_signal(|| {
        (0..4).map(|id| Artwork {
            id,
            title: "(Art Title)".to_string(),
            artist_name: "Artist Name".to_string(),
            verified: true,
            collection: Some("Happy".to_string()),
            attributes: vec!["Paid".to_string(), "Verified".to_string()],
            ways_to_sell: "Bid".to_string(),
            volume_eth: 2.370,
            volume_usd: 8147.63,
            status: "Active".to_string(),
        }).collect::<Vec<_>>()
    });

    let collections_data = collections.read();

    // Function to simulate API call for creating a collection
    let mut create_collection = move |name: String| {
        // This would be an API call
        // log::info!("Creating collection: {}", name);
        
        // Store the collection name for the success modal
        new_collection_name.set(name);
        
        // Show success modal after "API call"
        popup.open(rsx!{
            SuccessModal {
                show: true,
                collection_name: new_collection_name.read().clone(),
                on_confirm: move |_| { popup.close() },
            }
        }).with_id("success-modal");
    };

    rsx! {
        div { class: "w-full min-h-screen bg-background h-full flex text-white justify-center items-center",
            // Main content
            div { class: "flex flex-col w-full h-full",
                // Header
                div { class: "",
                    h1 { class: "text-2xl sm:text-2xl font-bold font-Pretendard",
                        "Collections {agit_id}"
                    }
                    p { class: "text-sm  sm:text-sm text-gray-400", "1,120 Total Collections" }
                }
                // Search and filters
                div { class: "p-4 flex flex-col sm:flex-row sm:items-center gap-4",
                    button {
                        class: "p-2 border border-border-primary text-white w-full sm:w-auto",
                        onclick: move |_| show_filters.toggle(),
                        settings::Sliders {}
                    }
                    div { class: "relative flex-1 mr-4",
                        div { class: "absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none",
                            edit::Search {}
                        }
                        input {
                            class: "bg-border-background border border-border-primary text-white text-sm rounded-none block w-full pl-10 p-2.5",
                            placeholder: tr.search_by_title,
                            r#type: "text",
                        }
                    }
                    button {
                        class: "bg-border-background border border-border-primary text-white px-4 py-2 flex items-center justify-center w-full sm:w-auto",
                        onclick: move |_| {
                            popup.open(rsx! {
                                NewCollectionModal {
                                    show: true,
                                    on_close: move |_| {
                                        popup.close();
                                    },
                                    artworks: artwork.clone(),
                                    on_select_artworks: move |selected| {
                                        selected_artworks.set(selected);
                                        popup
                                            .open(rsx! {
                                                TransferConfirmationModal {
                                                    show: true,
                                                    selected_count: selected_artworks.read().len(),
                                                    on_back: move |_| popup.close(),
                                                    on_continue: move |_| {
                                                        popup.open(rsx! {
                                                            CollectionNameModal {
                                                                show: true,
                                                                on_back: move |_| popup.close(),
                                                                on_add: move |name| {
                                                                    create_collection(name);
                                                                },
                                                            }
                                                        }).with_id("collection-name-modal");
                                                    },
                                                }
                                            })
                                            .with_id("transfer-confirmation-modal");
                                    },
                                }
                            }).with_id("new-collection-modal");
                        },
                        folder::UploadFolder { class: "mr-3" }
                        {tr.new_collection}
                    }
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
                            FilterSidebar {}
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
                                            span { class: "ml-1 text-gray-500", "%" }
                                        }
                                    }
                                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            span { {tr.collection} }
                                            span { class: "ml-1 text-gray-500", "%" }
                                        }
                                    }
                                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            span { {tr.floor_price} }
                                            span { class: "ml-1 text-gray-500", "%" }
                                        }
                                    }
                                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            span { {tr.floor_change} }
                                            span { class: "ml-1 text-gray-500", "%" }
                                        }
                                    }
                                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            span { {tr.volume_change} }
                                            span { class: "ml-1 text-gray-500", "%" }
                                        }
                                    }
                                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            span { {tr.volume} }
                                            span { class: "ml-1 text-gray-500", "%" }
                                        }
                                    }
                                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            span { {tr.owners} }
                                            span { class: "ml-1 text-gray-500", "%" }
                                        }
                                    }
                                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            span { {tr.stock} }
                                            span { class: "ml-1 text-gray-500", "%" }
                                        }
                                    }
                                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            span { {tr.status} }
                                            span { class: "ml-1 text-gray-500", "%" }
                                        }
                                    }
                                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        ""
                                    } // For the actions column
                                }
                            }
                            // Table body
                            tbody {
                                {
                                    collections_data
                                        .iter()
                                        .enumerate()
                                        .map(|(index, collection)| {
                                            rsx! {
                                                tr { key: "{index}", class: "border-b border-border-primary",
                                                    // Table row content...
                                                    // (Keeping the existing table row code)
                                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", "{collection.id}" }
                                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                                        div { class: "flex items-center",
                                                            div { class: "w-6 h-6 sm:w-8 sm:h-8 bg-[#333] mr-2" }
                                                            span { "{collection.name}" }
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
                                                        div { class: "text-xs text-gray-400", "$ {collection.floor_price_usd}" }
                                                    }
                                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                                        div { "{collection.floor_change_eth} ETH" }
                                                        div { class: "text-xs text-gray-400", "$ {collection.floor_change_usd}" }
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
                                                        div { class: "text-xs text-gray-400", "$ {collection.volume_usd}" }
                                                    }
                                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", "{collection.owners}" }
                                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", "{collection.stock}" }
                                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", "{collection.status}" }
                                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                                        button { class: "text-gray-400 hover:text-white",
                                                            svg {
                                                                view_box: "0 0 24 24",
                                                                width: "18",
                                                                height: "18",
                                                                stroke: "currentColor",
                                                                stroke_width: "2",
                                                                fill: "none",
                                                                path { d: "M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z" }
                                                            }
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
