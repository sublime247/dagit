use crate::pages::agits::_id::management::collectors::models::Activity;
use bdk::prelude::*;

#[component]
#[allow(unused_variables)]
pub fn render_activity_table(activity: Vec<Activity>) -> Element {
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
