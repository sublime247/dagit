use crate::pages::agits::_id::management::collectors::models::Activity;
use crate::pages::agits::_id::management::collectors::models::Asset;
use crate::pages::agits::_id::management::{TableHeader, collections::CollectionTranslate};
use bdk::prelude::{by_components::icons::validations, *};
#[component]
pub fn OwnedTable(assets: Vec<Asset>, lang: Language) -> Element {
    let mut active_dropdown = use_signal(|| None::<usize>);

    rsx! {
        table {
            class: "w-full text-sm text-left border-collapse min-w-[800px]",
            TableHeader { lang }
            tbody {
                { assets.iter().enumerate().map(|(index, asset)| {
                    let is_dropdown_open = active_dropdown.with(|active| active == &Some(index));
                    rsx! {
                        tr {
                            key: "{index}",
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
                                            span { {asset.title.clone()} }
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
                                    })}
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
                                    div {
                                        class: "flex flex-col",
                                        span {
                                            class: "text-green-500",
                                            "+{asset.price_change_24h.to_string()}%"
                                        }
                                        span { class: "pl-4 text-xs", "24h" }
                                    }
                                    div {
                                        class: "flex flex-col",
                                        span {
                                            class: "text-red-500",
                                            {asset.price_change_7d.to_string()}
                                        }
                                        span { class: "pr-4 text-xs", "7d" }
                                    }
                                }
                            }
                            td {
                                class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                div {
                                    class: "flex flex-col",
                                    span { {asset.volume.to_string()} }
                                    span { class: "text-xs text-gray-500", {asset.volume_usd.to_string()} }
                                }
                            }
                            td {
                                class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                div {
                                    class: "flex flex-col",
                                    span { {asset.royalty.to_string()} }
                                    span { class: "text-xs text-gray-500", {asset.royalty_usd.to_string()} }
                                }
                            }
                            td {
                                class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                {asset.status.to_string()}
                            }
                            td {
                                class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                div {
                                    class: "relative",
                                    button {
                                        class: "text-gray-400 hover:text-white",
                                        onclick: move |_| {
                                            active_dropdown.set(
                                                if is_dropdown_open { None } else { Some(index) }
                                            ); // Toggle dropdown: open if closed, close if open
                                        },
                                        validations::Extra { class: "[&>circle]:stroke-white", height: 18 }
                                    }
                                    // Render dropdown if this row's dropdown is active
                                    if is_dropdown_open {
                                        div {
                                            class: "absolute right-0 mt-2 w-48 bg-black border border-green-500 rounded shadow-lg z-10",
                                            div {
                                                class: "py-1",
                                                button {
                                                    class: "block w-full text-left px-4 py-2 text-sm text-white hover:bg-gray-800",
                                                    onclick: move |_| {
                                                        println!("Edit the artwork clicked");
                                                        active_dropdown.set(None); // Close dropdown after action
                                                    },
                                                    "Edit the artwork"
                                                }
                                                button {
                                                    class: "block w-full text-left px-4 py-2 text-sm text-white hover:bg-gray-800",
                                                    onclick: move |_| {
                                                        println!("Remove from the list clicked");
                                                        active_dropdown.set(None); // Close dropdown after action
                                                    },
                                                    "Remove from the list"
                                                }
                                                button {
                                                    class: "block w-full text-left px-4 py-2 text-sm text-white hover:bg-gray-800",
                                                    onclick: move |_| {
                                                        println!("Option clicked");
                                                        active_dropdown.set(None); // Close dropdown after action
                                                    },
                                                    "Option"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                })}
            }
        }
    }
}

#[component]
#[allow(unused_variables)]
pub fn ActivityTable(activity: Vec<Activity>, lang: Language) -> Element {
    let tr: CollectionTranslate = translate(&lang);
    rsx! {
                { activity.iter().enumerate().map(|(index, activity)| {

                    rsx!{
                        div {
                            class: "flex items-center p-4 mb-2.5 rounded bg-[#1a1a1a] hover:bg-[#0a3b2c] transition-colors duration-200 group",

                            // Art thumbnail
                            div {
                                class: "w-16 h-16 bg-[#f0c14b] mr-4 rounded flex justify-center items-center",
                            }
                            // Content
                            div { class: "flex-1",
                                // Title row
                                div { class: "flex items-center mb-1",
                                    span { class: "font-bold text-base mr-1", "Art Title" }

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
                                // Purchase info
                                div { class: "flex items-center mb-1 text-sm text-gray-400",
                                    span { "Purchased by " }
                                    // User avatar
                                    div { class: "w-4 h-4 bg-white rounded-full mx-1 inline-block" }

                                    span { class: "mr-1 text-white", "20114FWO" }

                                    span { class:"text-gray-400", "from"}

                                    // User avatar
                                    div { class: "w-4 h-4 bg-white rounded-full mx-1 inline-block" }

                                    span {class:"text-white", "20114FWO" }
                                }

                                // Timestamp
                                div { class: "text-sm text-gray-400 flex items-center",
                                    span { "30 mins ago" }

                                    // External link icon
                                    span { class: "ml-1 text-xs", "↗" }
                                }
                            }
                        }
                    }
                }
            )
        }
    }
}
