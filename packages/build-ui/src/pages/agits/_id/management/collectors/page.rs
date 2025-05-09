#![allow(unused)]
use bdk::prelude::{
    by_components::icons::{arrows, edit, validations},
    svg_attributes::to,
    *,
};

use crate::routes::Route;

use super::controllers::Controller;
use super::i18n::CollectorsTranslate;
#[component]
pub fn CollectorsPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    let ctrl = Controller::new(lang, agit_id)?;
    let tr: CollectorsTranslate = translate(&lang);
    let mut search_query = use_signal(String::new);
    let collectors = ctrl.collector();

    rsx! {
        div { class: "w-full min-h-screen bg-background h-full flex text-white justify-center items-center",
            div { class: "flex flex-col w-full h-full",
                div { class: "flex flex-col mb-6",
                    h1 { class: "text-2xl font-bold", "Collectors" }
                    p { class: "text-sm text-gray-400", "1,201 Total Collectors" }
                }

                div { class: "mb-6",
                    div { class: "relative",
                        div { class: "absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none",
                            edit::Search { class: "[&>path]:stroke-white [&>circle]:stroke-white" }
                        }
                        input {
                            class: "bg-border-background border border-border-primary text-white text-sm rounded-none block w-full pl-10 p-2.5",
                            placeholder: tr.search_by_title,
                            r#type: "text",
                        }
                    }
                }

                div { class: " flex-1 overflow-x-auto",
                    table { class: "w-full text-sm text-left border-collapse min-w-[800px]",
                        thead { class: "text-xs text-white capitalize bg-table-background border-b border-border-primary",
                            tr {

                                th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                    div { class: "flex items-center",
                                        span { {tr.collector_id} }
                                        arrows::UpDown {
                                            class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                            height: 18,
                                            width: 18,
                                        }
                                    }
                                }
                                th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                    div { class: "flex items-center",
                                        span { {tr.total_volume} }
                                        arrows::UpDown {
                                            class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                            height: 18,
                                            width: 18,
                                        }
                                    }
                                }
                                th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                    div { class: "flex items-center",
                                        span { {tr.owned} }
                                        arrows::UpDown {
                                            class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                            height: 18,
                                            width: 18,
                                        }
                                    }
                                }
                                th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                    "Token ID"
                                }
                                th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                    "Wallet Address"
                                }
                                th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                    div { class: "flex items-center",
                                        span { {tr.last_activity} }
                                        arrows::UpDown {
                                            class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                            height: 18,
                                            width: 18,
                                        }
                                    }
                                }
                                th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                    div { class: "flex items-end",
                                        span { "" }
                                        validations::Extra {
                                            class: "[&>circle]:stroke-white",
                                            height: 18,
                                        }
                                    }
                                }
                            }
                        }
                        tbody {
                            {
                                collectors
                                    .into_iter()
                                    .enumerate()
                                    .map(|(_index, collector)| {
                                        let collector_id = collector.id.clone();
                                        rsx! {
                                            tr {
                                                key: "{collector_id}",
                                                class: "hover:bg-gray-900 cursor-pointer",
                                                onclick: move |_| {
                                                    let collector_id = collector.id.clone();
                                                    use_navigator()
                                                        .push(Route::CollectorDetailPage {
                                                            lang: lang,
                                                            agit_id: agit_id(),
                                                            collector_id,
                                                        });
                                                },
                                                td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                                    div { class: "flex items-center",
                                                        div { class: "w-8 h-8 bg-white rounded-full mr-2 flex items-center justify-center" }
                                                        div { class: "flex items-center",
                                                            span { "10FEO!20" }
                                                            span {
                                                                class: "ml-1 text-green-500 hidden aria-verified:block",
                                                                "aria-verified": collector.verified,
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
                                                    }
                                                }
                                                td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                                    div { class: "flex flex-col",
                                                        span { "{collector.total_volume} ETH" }
                                                        span { class: "text-xs text-gray-500", "$ {collector.total_volume}" }
                                                    }
                                                }
                                                td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", {collector.owned.to_string()} }
                                                td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                                    div { class: "flex space-x-1",
                                                        {
                                                            collector
                                                                .token_ids
                                                                .iter()
                                                                .map(|token| {
                                                                    rsx! {
                                                                        span { class: "text-xs px-2 py-1 rounded border border-white", "{token}" }
                                                                    }
                                                                })
                                                        }
                                                    }
                                                }
                                                td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", {collector.wallet_address.clone()} }
                                                td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", {collector.last_activity.clone()} }
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
