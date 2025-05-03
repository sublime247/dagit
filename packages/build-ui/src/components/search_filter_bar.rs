use bdk::prelude::{
    by_components::icons::{arrows, edit, folder, layouts, settings, validations},
    *,
};
#[component]
pub fn SearchFilterBar(
    #[props(default = false)] show_filter_btn: bool, // Controls whether the filter button is shown
    #[props(default = None)] on_filter_click: Option<EventHandler<()>>, // Callback for filter button click
    placeholder: String, // Placeholder for the search input // Callback for search input change
    #[props(default = None)] on_add_click: Option<EventHandler<()>>,
    #[props(default = None)] on_remove_click: Option<EventHandler<()>>,
    on_search_change: EventHandler<String>, // Callback for "New Collection" button click
    on_search: EventHandler<String>,        // Callback for search input change
    #[props(default = false)] show_add_btn: bool, // Controls whether the "New Collection" button is shown
    #[props(default = false)] show_remove_btn: bool, // Controls whether the "Remove" button is shown
    #[props(default = None)] on_view_mode_click: Option<EventHandler<()>>,
    #[props(default = false)] show_art_btn: bool,
    #[props(default = false)] show_all_filter_field: bool,
    #[props(default = "".to_string())] add_btn_text: String,
    #[props(default = "".to_string())] remove_btn_text: String, // Text for the "New Collection" button
                                                                // Controls whether the search input is shown
) -> Element {
    rsx! {
        div { class: "p-4 flex flex-col sm:flex-row sm:items-center gap-4",
            if show_filter_btn {
                button {
                    class: "p-2 border border-border-primary text-white w-full sm:w-auto",
                    onclick: move |_| {
                        if let Some(on_filter_click) = &on_filter_click {
                            on_filter_click(());
                        }
                    },
                    settings::Sliders { class: "[&>path]:stroke-white" }
                }
            }
            if show_art_btn {
                button {
                    class: "p-2 border border-border-primary text-white w-full sm:w-auto",
                    onclick: move |_| {
                        if let Some(on_view_mode_click) = &on_view_mode_click {
                            on_view_mode_click(());
                        }
                    },
                    layouts::Window { class: "[&>path]:stroke-white" }
                }
            }
            if show_all_filter_field {
                // All dropdown
                div { class: "relative",
                    div { class: "absolute inset-y-0 right-0 flex items-center pr-3 pointer-events-none",
                        arrows::ChevronDown {
                            class: "[&>path]:stroke-white",
                            height: 20,
                            width: 20,
                        }
                    }
                    input {
                        class: "bg-border-background border border-border-primary text-white text-sm rounded-none block w-full pl-3 p-2.5",
                        placeholder: "All",
                        r#type: "text",
                    }
                }
            }

            div { class: "relative flex-1 mr-4",
                div { class: "absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none",
                    edit::Search { class: "[&>path]:stroke-white [&>circle]:stroke-white" }
                }
                input {
                    class: "bg-border-background border border-border-primary text-white text-sm rounded-none block w-full pl-10 p-2.5",
                    placeholder: "{placeholder}",
                    r#type: "text",
                    oninput: move |e| on_search_change(e.value().clone()),
                }
            }
            if show_add_btn {
                button {
                    class: "bg-border-background border border-border-primary text-white px-4 py-2 flex items-center justify-center w-full sm:w-auto",
                    onclick: move |_| {
                        if let Some(on_add_click) = &on_add_click {
                            on_add_click(());
                        }
                    },
                    if remove_btn_text.is_empty() {
                        folder::UploadFolder { class: "mr-3 [&>path]:stroke-white [&>circle]:stroke-white" }
                    } else {
                        validations::Add { class: "mr-3 [&>path]:stroke-white [&>circle]:stroke-white" }
                    }
                    "{add_btn_text}"
                }
            }
            if show_remove_btn {
                button {
                    class: "bg-border-background border border-border-primary text-white px-4 py-2 flex items-center justify-center w-full sm:w-auto",
                    onclick: move |_| {
                        if let Some(on_remove_click) = &on_remove_click {
                            on_remove_click(());
                        }
                    },
                    validations::Remove { class: "mr-3 [&>path]:stroke-white [&>circle]:stroke-white" }
                    "{remove_btn_text}"
                }
            }
        }
    }
}
