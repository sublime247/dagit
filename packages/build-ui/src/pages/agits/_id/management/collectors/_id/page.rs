use crate::{pages::agits::_id::management::collectors::controllers::Controller, routes::Route};
use bdk::prelude::{
    by_components::icons::{arrows, edit, layouts, settings, validations},
    *,
};

#[component]
#[allow(unused_variables)]
pub fn CollectorDetailPage(
    lang: Language,
    agit_id: ReadOnlySignal<i64>,
    collector_id: i64,
) -> Element {
    let mut active_tab = use_signal(|| "owned");
    let search_query = use_signal(String::new);
    let view_mode = use_signal(|| "grid");
    let filter = use_signal(|| "All");
    let ctrl = Controller::new(lang, agit_id)?;
    let assets = ctrl.asset();

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
                div { class: "absolute -bottom-1 -right-1 bg-green-500 rounded-full p-1",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-4 w-4 text-white",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M5 13l4 4L19 7"
                        }
                    }
                }
            }

            // Stats + Wallets (stacked vertically)
            div { class: "flex-1 grid grid-cols-3 gap-4",

                // Total Volume column
                div {
                    // Stat
                    div { class: "border-b border-gray-800 flex justify-between items-center mb-4",
                        p { class: "text-sm text-white", "Total Volume" }
                        p { class: "text-sm font-bold", "2,370 ETH" }
                    }
                    // Wallet
                    div { class: "pt-2 border-b border-gray-800 flex justify-between items-center pb-2 mt-4",
                        p { class: "text-sm text-white", "Wallet Address" }
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
                // Total Volume column
                div {
                    // Stat
                    div { class: "border-b border-gray-800 flex justify-between items-center mb-4",
                        p { class: "text-sm text-white", "Total Volume" }
                        p { class: "text-sm font-bold", "2,370 ETH" }
                    }
                    // Wallet
                    div { class: "pt-2 border-b border-gray-800 flex justify-between items-center pb-2 mt-4",
                        p { class: "text-sm text-white", "Wallet Address" }
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
                // Total Volume column
                div {
                    // Stat
                    div { class: "border-b border-gray-800 flex justify-between items-center mb-4",
                        p { class: "text-sm text-white", "Owned" }
                        p { class: "text-sm font-bold", "245" }
                    }
                    // Wallet
                    div { class: "pt-2 border-b border-gray-800 flex justify-between items-center pb-2 mt-4",
                        p { class: "text-sm text-white", "Wallet Address" }
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

                    // Tabs
                  div {
                        class: "border-b border-gray-800 mb-6",
                        div {
                            class: "flex space-x-8",
                            button {
                                // class: "pb-4 px-2 {active_tab.read() == owned} ? \"border-b-2 border-white font-medium\" : \"text-gray-400\"}",
                                onclick: move |_| active_tab.set("owned"),
                                "Owned "
                                span { class: "ml-1 text-gray-500", "585" }
                            }
                            button {
                                // class: "pb-4 px-2 {active_tab.read() == \"created\" ? \"border-b-2 border-white font-medium\" : \"text-gray-400\"}",
                                onclick: move |_| active_tab.set("created"),
                                "Created "
                                span { class: "ml-1 text-gray-500", "585" }
                            }
                            button {
                                // class: "pb-4 px-2 {active_tab.read() == \"trade\" ? \"border-b-2 border-white font-medium\" : \"text-gray-400\"}",
                                onclick: move |_| active_tab.set("trade"),
                                "Trade"
                            }
                            button {
                                // class: "pb-4 px-2 {active_tab.read() == \"activity\" ? \"border-b-2 border-white font-medium\" : \"text-gray-400\"}",
                                onclick: move |_| active_tab.set("activity"),
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
                            button {
                                class: "p-2 border border-border-primary text-white w-full sm:w-auto",
                                // onclick: move |_| show_filters.toggle(),
                                layouts::Window{ class: "[&>path]:stroke-white" }
                            }


                            // Filter dropdown
                            button {
                                class: "p-2 border border-border-primary text-white w-full sm:w-auto",
                                // onclick: move |_| show_filters.toggle(),
                                settings::Sliders { class: "[&>path]:stroke-white" }
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
                        table {
                            class: "w-full text-sm text-left border-collapse min-w-[800px]",
                            thead {
                                tr {

                                    th {class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                      div { class: "flex items-center",
                                         span {"Title"}
                                          arrows::UpDown { class: "[&>path]:stroke-white [&>circle]:stroke-white", height:18, width:18 }
                                       }
                                    }
                                    th {class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                      div { class: "flex items-center",
                                         span {"Attributes"}
                                        //   arrows::UpDown { class: "[&>path]:stroke-white [&>circle]:stroke-white", height:18, width:18 }
                                       }
                                    }
                                    th {class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                      div { class: "flex items-center",
                                         span {"Ways to Sell"}
                                          arrows::UpDown { class: "[&>path]:stroke-white [&>circle]:stroke-white", height:18, width:18 }
                                       }
                                    }
                                    th {class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                      div { class: "flex items-center",
                                         span {"Owner"}
                                          arrows::UpDown { class: "[&>path]:stroke-white [&>circle]:stroke-white", height:18, width:18 }
                                       }
                                    }
                                    th {class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                      div { class: "flex items-center",
                                         span {"Current Price"}
                                          arrows::UpDown { class: "[&>path]:stroke-white [&>circle]:stroke-white", height:18, width:18 }
                                       }
                                    }
                                    th {class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                      div { class: "flex items-center",
                                         span {"Average Price"}
                                          arrows::UpDown { class: "[&>path]:stroke-white [&>circle]:stroke-white", height:18, width:18 }
                                       }
                                    }
                                    th {class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                      div { class: "flex items-center",
                                         span {"Price Change"}
                                          arrows::UpDown { class: "[&>path]:stroke-white [&>circle]:stroke-white", height:18, width:18 }
                                       }
                                    }
                                    th {class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                      div { class: "flex items-center",
                                         span {"Volume"}
                                          arrows::UpDown { class: "[&>path]:stroke-white [&>circle]:stroke-white", height:18, width:18 }
                                       }
                                    }
                                    th {class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                      div { class: "flex items-center",
                                         span {"Royalty"}
                                          arrows::UpDown { class: "[&>path]:stroke-white [&>circle]:stroke-white", height:18, width:18 }
                                       }
                                    }
                                    th {class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                      div { class: "flex items-center",
                                         span {"Status"}
                                          arrows::UpDown { class: "[&>path]:stroke-white [&>circle]:stroke-white", height:18, width:18 }
                                       }
                                    }
                                   th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                    div { class: "flex items-center",
                                        span { "" }
                                        validations::Extra { class: "[&>circle]:stroke-white", height:18 }
                                    }
                                }

                                }
                            }
                            tbody {
                               { assets.iter().enumerate().map(|(index, asset)| {
                                    rsx! {
                                        tr { key:"{index}",
                                            class: "border-b border-gray-800 hover:bg-gray-900",
                                              td {
                                                class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                                div {
                                                    class: "flex items-center",
                                                    div {
                                                        class: "w-8 h-8 bg-white rounded mr-2 flex items-center justify-center",
                                                        ""
                                                    }
                                                    div {
                                                        class: "flex flex-col",
                                                        div {
                                                            class: "flex items-center",
                                                            span{ {asset.title.clone()} }
                                                                span {
                                                                    class: "ml-1 text-green-500",
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
                                                        span { class: "text-xs text-gray-500", {asset.artist_name.clone()} }
                                                    }
                                                }
                                            }
                                              td {
                                                class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",

                                                div {
                                                    class: "flex space-x-1",
                                                    {asset.attributes.iter().map(|attr| {
                                                        rsx! {
                                                            span {
                                                                class: "text-xs px-2 py-1 rounded border border-white",
                                                             {attr.clone()}
                                                            }
                                                        }
                                                    })
                                                }
                                                }
                                            }
                                              td {
                                                class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",

                                                {asset.way_to_sell.clone()}
                                            }
                                              td {
                                                class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",


                                                {asset.owner.to_string()}
                                            }
                                              td {
                                                class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",

                                                div {
                                                    class: "flex flex-col",
                                                    span { {asset.current_price.to_string()} }
                                                    span { class: "text-xs text-gray-500", {asset.current_price_usd.to_string()} }
                                                }
                                            }
                                              td {
                                                class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",

                                                div {
                                                    class: "flex flex-col",
                                                    span { {asset.average_price.to_string()} }
                                                    span { class: "text-xs text-gray-500", {asset.average_price_usd.to_string()} }
                                                }
                                            }
                                              td {
                                                class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",


                                                div {
                                                    class: "flex justify-between",
                                                    div {  class:"flex flex-col",
                                                    span {
                                                        class: "text-green-500",
                                                       "+{asset.price_change_24h.to_string()}%"
                                                     }
                                                    span { class: "pl-4 text-xs", "24h" }
                                                }
                                                div {   class:"flex flex-col",

                                                    span {
                                                        class: "text-red-500",
                                                        {asset.price_change_7d.to_string()}

                                                    }
                                                    span {class: "pr-4 text-xs", "7d" }
                                                }
                                                }
                                            }
                                              td {
                                                class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",

                                                div {
                                                    class: "flex flex-col",
                                                    span { {asset.volume.to_string()}}
                                                    span { class: "text-xs text-gray-500", {asset.volume_usd.to_string()} }
                                                }
                                            }
                                              td {
                                                class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",

                                                div {
                                                    class: "flex flex-col",
                                                    span { {asset.royalty.to_string()} }
                                                    span { class: "text-xs text-gray-500", {asset.royalty_usd.to_string()}}
                                                }
                                            }
                                              td {
                                                class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",

                                                {asset.status.to_string()}
                                            }
                                              td {
                                                class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",

                                                button {
                                                    class: "text-gray-400 hover:text-white",
                                                    validations::Extra { class: "[&>circle]:stroke-white", height:18 }
                                                }

                                            }
                                        }
                                    }
                                })}
                            }
                        }
                    }
                }
            }
        }
}
