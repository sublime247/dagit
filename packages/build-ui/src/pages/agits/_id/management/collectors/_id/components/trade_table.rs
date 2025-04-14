use crate::pages::agits::_id::management::collectors::models::Asset;
use bdk::prelude::{
    by_components::icons::{arrows, validations},
    *,
};

#[component]
pub fn render_trade_table(assets: Vec<Asset>) -> Element {
    rsx! {
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
