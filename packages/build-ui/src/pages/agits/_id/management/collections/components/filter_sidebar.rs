use bdk::prelude::*;

use super::FilterDropdown;
use by_components::icons::edit;

#[component]
pub fn FilterSidebar() -> Element {
    rsx! {
        div { class: "w-64 bg-filter-bg border-r border-border-primary p-4 flex flex-col gap-6",
            // Artist Section
            div { id: "artist-section", class: "flex flex-col gap-2",
                div { class: "text-sm font-semibold", "Artist" }
                div { class: "relative",
                    input {
                        class: "w-full bg-border-bg border border-border-primary text-white text-sm rounded-none p-2 pl-8",
                        placeholder: "Search",
                        r#type: "text",
                    }
                    div { class: "absolute inset-y-0 right-2 flex items-center pointer-events-none",
                        edit::Search { class: "[&>path]:stroke-white [&>circle]:stroke-white" }
                    }
                }
            }

            // Price Section
            div { id: "price-section", class: "flex flex-col gap-2",
                div { class: "text-sm font-semibold", "Price" }
                select { class: "bg-border-bg border border-border-primary text-white text-sm p-2",
                    // placeholder: "Currency",
                    option { "Currency" }
                }
                div { class: "flex gap-2",
                    button { class: "flex-1 bg-border-bg border border-border-primary text-white text-sm p-2 rounded-3xl",
                        "Lowest"
                    }
                    button { class: "flex-1 bg-border-bg border border-border-primary text-white text-sm p-2 rounded-3xl",
                        "Highest"
                    }
                }
            }

            // Attributes Section
            div { id: "attributes-section", class: "flex flex-col gap-2",
                div { class: "text-sm font-semibold", "Attributes" }
                div { class: "relative",
                    input {
                        class: "w-full bg-border-bg border border-border-primary text-white text-sm rounded-none p-2 pl-8",
                        placeholder: "Search",
                        r#type: "text",
                    }
                    div { class: "absolute inset-y-0 right-2 flex items-center pointer-events-none",
                        edit::Search { class: "[&>path]:stroke-white [&>circle]:stroke-white" }
                    
                    }
                }

                // Attribute dropdowns
                FilterDropdown { label: "Medium" }
                FilterDropdown { label: "Theme" }
                FilterDropdown { label: "Art style" }
                FilterDropdown { label: "Material" }
                FilterDropdown { label: "Color" }
                FilterDropdown { label: "Size" }
                FilterDropdown { label: "Year" }
            }

            // Tags Section
            div { class: "flex flex-col gap-2",
                div { class: "flex items-center justify-between text-sm text-gray-400",
                    label { class: "flex items-center gap-2",
                        input {
                            r#type: "checkbox",
                            class: "form-checkbox bg-transparent border-border-primary text-primary rounded-none accent-border-primary",
                        }
                        span { "Tag" }
                    }
                    span { "50" }
                }
                // Repeat for more tags...
                div { class: "flex items-center justify-between text-sm text-gray-400",
                    label { class: "flex items-center gap-2",
                        input {
                            r#type: "checkbox",
                            class: "form-checkbox bg-transparent border-border-primary text-primary rounded-none accent-border-primary",
                        }
                        span { "Tag" }
                    }
                    span { "50" }
                }
            }
        }
    }
}
