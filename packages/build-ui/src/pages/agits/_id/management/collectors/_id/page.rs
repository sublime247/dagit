use crate::{
    pages::agits::_id::management::collectors::{
        _id::components::{
            render_activity_table, render_created_table, render_nft_images, render_owned_table, render_trade_table
        },
        controllers::Controller, i18n::CollectorsTranslate,
    },
    routes::Route,
};
use bdk::prelude::{
    by_components::icons::{arrows, edit, layouts, settings},
    *,
};
#[derive(Clone, Debug, PartialEq)]
pub enum AssetTab {
    Owned,
    Created,
    Trade,
    Activity,
}
#[component]
#[allow(unused_variables)]
pub fn CollectorDetailPage(
    lang: Language,
    agit_id: ReadOnlySignal<i64>,
    collector_id: i64,
) -> Element {
    let search_query = use_signal(String::new);
    let mut view_mode = use_signal(|| "table"); 
    let filter = use_signal(|| "All");
    let tr: CollectorsTranslate = translate(&lang);
    let ctrl = Controller::new(lang, agit_id)?;
    let assets = ctrl.asset();
    let activities = ctrl.activity();
    let mut active_tab = use_signal(|| AssetTab::Owned);

    rsx! {
             div{
                class: "w-full min-h-screen bg-background h-full flex text-white justify-center items-center",

                div {
                    class: "flex flex-col w-full h-full text-white",

                    // Header with back button and collector info
                    div{ class: "flex flex-col mb-6",
                    div {
                        class: "flex items-center",
                        Link {
                            to: Route::CollectorsPage { lang, agit_id: agit_id() },
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
                            span {
                                class: "text-green-500 mr-2",
                                   // Verified icon
                                svg {
                                    view_box: "0 0 24 24",
                                    width: "20",
                                    height: "20",
                                    fill: "#10b981",
                                    class: "ml-1",
                                    path { d: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" }
                                }
                            }

                             h1 {
                            class: "text-xl font-bold",
                            {format!("{{Collector {}}}",collector_id)}
                          }
                    }
                }
                div {
                    class: "text-sm text-gray-400 m-2",
                    "Last Activity 2mins ago / Joined Feb 01, 2025"
                }
        }
        div { class: "flex flex-col text-white space-y-4 mb-20",

        // Top: Avatar + Stats
        div { class: "flex items-start",
            // Avatar section
            div { class: "relative mr-8",
                div { class: "w-20 h-20 bg-gray-300 rounded-full" }
                div { class: "absolute -bottom-1 -right-1 text-green-500 mr-2",
                svg {
                    view_box: "0 0 24 24",
                    width: "28",
                    height: "28",
                    fill: "#10b981",
                    class: "ml-1",
                    path { d: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" }
                }
                }
            }

            // Stats + Wallets (stacked vertically)
            div { class: "flex-1 grid grid-cols-3 gap-4",

                // Total Volume and created  column
                div {
                    // Stat
                    div { class: "border-b-2 border-border-primary flex justify-between items-center pb-2  mb-4",
                        p { class: "text-sm text-white", {tr.total_volume} }
                        p { class: "text-sm font-bold", "2,370 ETH" }
                    }
                    // Wallet
                    div { class: "pt-2 border-b-2 border-border-primary flex justify-between items-center pb-2 mt-4",
                    p { class: "text-sm text-white", "Created" }
                    p { class: "text-sm font-bold", "2,370 ETH" }
                    }
                }
                // Total owned  and wallet column
                div {
                    // Stat
                    div { class: "border-b-2 border-border-primary flex justify-between items-center pb-2 mb-4",
                        p { class: "text-sm text-white", {tr.owned} }
                        p { class: "text-sm font-bold", "2,370 ETH" }
                    }
                    // Wallet
                    div { class: "pt-2 border-b-2 border-border-primary flex justify-between items-center pb-2 mt-4",
                        p { class: "text-sm text-white", {tr.wallet_address} }
                        div { class: "flex items-center",
                            p { class: "font-mono text-sm", "0x1234...abcd" }
                            button { class: "ml-2 text-white",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    class: "h-4 w-4",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke: "currentColor",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M19 9l-7 7-7-7"
                                    }
                                }
                            }
                        }
                    }
                }
                // Total owned and wallet column
                div {
                    // Stat
                    div { class: "border-b-2 border-border-primary flex justify-between items-center pb-2 mb-4",
                        p { class: "text-sm text-white", {tr.owned} }
                        p { class: "text-sm font-bold", "2,370 ETH" }
                    }
                    // Wallet
                    div { class: "pt-2 border-b-2 border-border-primary flex justify-between items-center pb-2 mt-4",
                        p { class: "text-sm text-white", {tr.wallet_address} }
                        div { class: "flex items-center",
                            p { class: "font-mono text-sm", "0x1234...abcd" }
                            button { class: "ml-2 text-white",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    class: "h-4 w-4",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke: "currentColor",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M19 9l-7 7-7-7"
                                    }
                                }
                            }
                        }
                    }
                }

            }
        }
    }

    div {
        class: "mb-10",
        div {
            class: "flex space-x-8 text-sm font-medium",
            button {
                class: format!("pb-2 px-5 flex items-center space-x-1 {}",
                    if *active_tab.read() == AssetTab::Owned {
                        "border-b border-white font-semibold text-white"
                    } else {
                        "text-gray-400 hover:text-white"
                    }
                ),
                onclick: move |_| active_tab.set(AssetTab::Owned),
                span { "Owned" }
                span {
                    class: "ml-1 px-1.5 py-0.5 rounded text-xs bg-white text-black font-semibold",
                    "585"
                }
            }

            button {
                class: format!("pb-2 px-5 flex items-center space-x-1 {}",
                    if *active_tab.read() == AssetTab::Created {
                        "border-b border-white font-semibold text-white"
                    } else {
                        "text-gray-400 hover:text-white"
                    }
                ),
                onclick: move |_| active_tab.set(AssetTab::Created),
                span { "Created" }
                span {
                    class: "ml-1 px-1.5 py-0.5 rounded text-xs bg-white text-black font-semibold",
                    "585"
                }
            }

            button {
                class: format!("pb-2 px-5 flex items-center space-x-1 {}",
                if *active_tab.read() == AssetTab::Trade {
                    "border-b border-white font-semibold text-white"
                } else {
                    "text-gray-400 hover:text-white"
                }
            ),
                onclick: move |_| active_tab.set(AssetTab::Trade),
                "Trade"
            }

            button {
                class: format!("pb-2 px-5 flex items-center space-x-1 {}",
                if *active_tab.read() == AssetTab::Activity {
                    "border-b border-white font-semibold text-white"
                } else {
                    "text-gray-400 hover:text-white"
                }
            ),
                onclick: move |_| active_tab.set(AssetTab::Activity),
                "Activity"
            }
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
                            class: "relative flex-grow",
                            div { class: "absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none",
                            edit::Search { class: "[&>path]:stroke-white [&>circle]:stroke-white" }
                        }
                        input {
                            class: "bg-border-background border border-border-primary text-white text-sm rounded-none block w-full pl-10 p-2.5",
                            placeholder: "Search",
                            r#type: "text",
                        }
                        }
                    }

                    // Assets table
                    div {
                        class: "overflow-x-auto flex-1",


                            {match active_tab.with(|tab| tab.clone()) {
                                AssetTab::Owned=> {
                                    if *view_mode.read()=="nftImages" {
                                        rsx!{
                                            render_nft_images {assets: assets.clone(), lang}
                                        }
                                    }else {
                                    rsx!{ render_owned_table {assets: assets.clone(), lang}} 
                               } 
                            }
                            AssetTab::Created => {
                                if *view_mode.read() == "nftImages" {
                                    rsx!{ render_nft_images { assets: assets.clone(), lang} }
                                } else {
                                    rsx!{ render_created_table { assets: assets.clone(), lang } }
                                }
                            },
                            AssetTab::Trade => {
                                if *view_mode.read() == "nftImages" {
                                    rsx!{ render_nft_images { assets: assets.clone(), lang } }
                                } else {
                                    rsx!{ render_trade_table { assets: assets.clone(), lang } }
                                }
                            },
                            AssetTab::Activity => {
                                // Activity is always shown as a table
                                rsx!{ render_activity_table { activity: activities.clone(), lang  } }
                            },
                            }}



                    }
                }
            }
        }
}
