use crate::{components::ServiceLogo, routes::Route};
use bdk::prelude::*;

mod i18n;
use i18n::NavigationTranslate;

#[derive(PartialEq, Eq)]
enum SelectedSection {
    HomePage,
    Orders,
    Management,
    Analytics,
    Design,
    ExtensionTool,
    Hub,
}
#[derive(PartialEq, Eq)]
enum SelectedItem {
    SalesRequest,
    ShippingLabel,
    Artworks,
    Collections,
    Artist,
    Collectors,
    CollectorDetail,
    Dao,
    Oracle,
    Faq,
    Report,
    Traffic,
    Design,
    ExtensionTool,
    None,
}
fn check_route(route: Route) -> (SelectedSection, SelectedItem) {
    match route {
        Route::HomePage {
            lang: _,
            agit_id: _,
        } => (SelectedSection::HomePage, SelectedItem::None),

        Route::SalesRequestPage {
            lang: _,
            agit_id: _,
        } => (SelectedSection::Orders, SelectedItem::SalesRequest),
        Route::ShippingLabelPage {
            lang: _,
            agit_id: _,
        } => (SelectedSection::Orders, SelectedItem::ShippingLabel),
        Route::ArtworkPage {
            lang: _,
            agit_id: _,
        } => (SelectedSection::Management, SelectedItem::Artworks),
        Route::CollectionPage {
            lang: _,
            agit_id: _,
        } => (SelectedSection::Management, SelectedItem::Collections),
        Route::ArtistPage {
            lang: _,
            agit_id: _,
        } => (SelectedSection::Management, SelectedItem::Artist),
        Route::CollectorsPage {
            lang: _,
            agit_id: _,
        } => (SelectedSection::Management, SelectedItem::Collectors),
        Route::CollectorDetailPage {
            lang: _,
            agit_id: _,
            collector_id: _,
        } => (SelectedSection::Management, SelectedItem::CollectorDetail),
        Route::DaoPage {
            lang: _,
            agit_id: _,
        } => (SelectedSection::Hub, SelectedItem::Dao),
        Route::OraclePage {
            lang: _,
            agit_id: _,
        } => (SelectedSection::Hub, SelectedItem::Oracle),
        Route::FaqPage {
            lang: _,
            agit_id: _,
        } => (SelectedSection::Hub, SelectedItem::Faq),
        Route::ReportPage {
            lang: _,
            agit_id: _,
        } => (SelectedSection::Analytics, SelectedItem::Report),
        Route::TrafficPage {
            lang: _,
            agit_id: _,
        } => (SelectedSection::Analytics, SelectedItem::Traffic),
        Route::DesignPage {
            lang: _,
            agit_id: _,
        } => (SelectedSection::Design, SelectedItem::Design),
        Route::ExtensionToolPage {
            lang: _,
            agit_id: _,
        } => (SelectedSection::ExtensionTool, SelectedItem::ExtensionTool),
        _ => {
            tracing::error!("Unknown route: {}", route);
            (SelectedSection::HomePage, SelectedItem::None)
        }
    }
}
#[component]
pub fn Navigation(lang: Language, agit_id: i64) -> Element {
    let tr: NavigationTranslate = translate(&lang);
    let route = use_route::<Route>();
    let (selected_section, selected_item) = check_route(route);
    rsx! {
        div { class: "flex flex-col p-8 gap-10.5 align-start",
            ServiceLogo { width: "110", height: "24", class: "fill-white" }
            div { class: "flex flex-col gap-5 text-white text-base",

                // Standalone sections without children
                Section {
                    label: tr.home,
                    selected: selected_section == SelectedSection::HomePage,
                    to: Route::HomePage { lang, agit_id },
                }
                // Sections with children
                Section {
                    label: tr.orders,
                    selected: selected_section == SelectedSection::Orders,
                    has_children: true,
                    Item { selected: selected_item == SelectedItem::SalesRequest,
                        Link {
                            to: Route::SalesRequestPage {
                                lang,
                                agit_id,
                            },
                            {tr.sales_request}
                        }
                    }
                    Item { selected: selected_item == SelectedItem::ShippingLabel,
                        Link {
                            to: Route::ShippingLabelPage {
                                lang,
                                agit_id,
                            },
                            {tr.shipping_label}
                        }
                    }
                }
                Section {
                    label: tr.management,
                    selected: selected_section == SelectedSection::Management,
                    has_children: true,
                    Item { selected: selected_item == SelectedItem::Artworks,
                        Link {
                            to: Route::ArtworkPage {
                                lang,
                                agit_id,
                            },
                            {tr.artworks}
                        }
                    }
                    Item { selected: selected_item == SelectedItem::Collections,
                        Link {
                            to: Route::CollectionPage {
                                lang,
                                agit_id,
                            },
                            {tr.collections}
                        }
                    }
                    Item { selected: selected_item == SelectedItem::Artist,
                        Link { to: Route::ArtistPage { lang, agit_id }, {tr.artists} }
                    }
                    Item { selected: selected_item == SelectedItem::Collectors,
                        Link {
                            to: Route::CollectorsPage {
                                lang,
                                agit_id,
                            },
                            {tr.collectors}
                        }
                    }
                  
                }
                Section {
                    label: tr.hub,
                    selected: selected_section == SelectedSection::Hub,
                    has_children: true,
                    Item { selected: selected_item == SelectedItem::Dao,
                        Link { to: Route::DaoPage { lang, agit_id }, {tr.dao} }
                    }
                    Item { selected: selected_item == SelectedItem::Oracle,
                        Link { to: Route::OraclePage { lang, agit_id }, {tr.oracle} }
                    }
                    Item { selected: selected_item == SelectedItem::Faq,
                        Link { to: Route::FaqPage { lang, agit_id }, {tr.faq} }
                    }
                }
                Section {
                    label: tr.analytics,
                    selected: selected_section == SelectedSection::Analytics,
                    has_children: true,
                    Item { selected: selected_item == SelectedItem::Report,
                        Link { to: Route::ReportPage { lang, agit_id }, {tr.report} }
                    }
                    Item { selected: selected_item == SelectedItem::Traffic,
                        Link {
                            to: Route::TrafficPage {
                                lang,
                                agit_id,
                            },
                            {tr.traffic}
                        }
                    }
                }
                // Standalone sections without children
                Section {
                    label: tr.design,
                    selected: selected_section == SelectedSection::Design,
                    to: Route::DesignPage { lang, agit_id },
                }
                Section {
                    label: tr.extension_tool,
                    selected: selected_section == SelectedSection::ExtensionTool,
                    to: Route::ExtensionToolPage {
                        lang,
                        agit_id,
                    },
                }
            }
        }
    }
}

#[component]
fn Section(
    label: String,
    #[props(default = VNode::empty())] children: Element,
    selected: bool,
    #[props(default = None)] to: Option<Route>,
    #[props(default = false)] has_children: bool,
) -> Element {
    let content = rsx! {
        div { class: "relative",
            // Add a green vertical line for selected sections with children
            {
                if selected && has_children {
                    rsx! {
                        div {
                            class: "absolute left-0 top-0 bg-primary ",
                            style: "width: 1px; height:38%; padding-bottom:5", // Thinner line
                        }
                    }
                } else {
                    rsx! {}
                }
            }
            div { class: "pl-1",
                div {
                    class: format!(
                        "flex items-center py-3 gap-3 hover:text-primary cursor-pointer transition-colors duration-200 ease-in-out {}",
                        if selected { "text-primary" } else { "" },
                    ),
                    {label}
                }
                div { class: "ml-5", {children} }
            }
        }
    };

    match to {
        Some(route) => rsx! {
            Link { to: route, {content} }
        },
        None => content,
    }
}

#[component]
fn Item(selected: bool, children: Element) -> Element {
    rsx! {
        div { class: "flex items-center relative",
            {
                if selected {
                    rsx! {
                        span { class: "text-primary absolute -left-5 top-1/2 transform -translate-y-1/2", "↪" }
                    }
                } else {
                    rsx! {}
                }
            }
            div {
                class: format!(
                    "hover:text-primary cursor-pointer transition-colors duration-200 ease-in-out {}",
                    if selected { "text-primary" } else { "text-inherit" },
                ),
                {children}
            }
        }
    }
}
