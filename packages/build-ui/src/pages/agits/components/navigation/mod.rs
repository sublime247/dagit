use crate::{components::ServiceLogo, routes::Route};
use bdk::prelude::*;

mod i18n;
use i18n::NavigationTranslate;

#[derive(PartialEq, Eq)]
enum SelectedSection {
    HomePage,
    Orders,
    Management,
    Community,
    Customers,
    Analytics,
    Design,
    ExtensionTool,
}
#[derive(PartialEq, Eq)]
enum SelectedItem {
    SalesRequest,
    ShippingLabel,
    Artworks,
    Collections,
    Artist,
    Collectors,
    Dao,
    Oracle,
    Faq,
    Report,
    Traffic,
    Design,
    ExtensionTool,
    Customers,
    None,
}
fn check_route(route: Route) -> (SelectedSection, SelectedItem) {
    match route {
        Route::HomePage {
            lang: _,
            agit_id: _,
        } => (SelectedSection::HomePage, SelectedItem::None),

        Route::SalesRequest { lang: _, agit_id: _ } => (SelectedSection::Orders, SelectedItem::SalesRequest),
        Route::ShippingLabel { lang: _, agit_id: _ } => (SelectedSection::Orders, SelectedItem::ShippingLabel),
        Route::Artworks { lang: _, agit_id: _ } => (SelectedSection::Management, SelectedItem::Artworks),
        Route::Collections { lang: _, agit_id: _ } => (SelectedSection::Management, SelectedItem::Collections),
        Route::Artist { lang: _, agit_id: _ } => (SelectedSection::Management, SelectedItem::Artist),
        Route::Collectors { lang: _, agit_id: _ } => (SelectedSection::Community, SelectedItem::Collectors),
        Route::Dao { lang: _, agit_id: _ } => (SelectedSection::Community, SelectedItem::Dao),
        Route::Oracle { lang: _, agit_id: _ } => (SelectedSection::Community, SelectedItem::Oracle),
        Route::Faq { lang: _, agit_id: _ } => (SelectedSection::Community, SelectedItem::Faq),
        Route::Report { lang: _, agit_id: _ } => (SelectedSection::Analytics, SelectedItem::Report),
        Route::Traffic { lang: _, agit_id: _ } => (SelectedSection::Analytics, SelectedItem::Traffic),
        Route::Customers { lang: _, agit_id: _ } => (SelectedSection::Customers, SelectedItem::Customers),
        Route::Design { lang: _, agit_id: _ } => (SelectedSection::Design, SelectedItem::Design),
        Route::ExtensionTool { lang: _, agit_id: _ } => (SelectedSection::ExtensionTool, SelectedItem::ExtensionTool),
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
        div { class: "flex flex-col p-10 gap-12.5",
            ServiceLogo {width: "110", height: "24", class: "fill-white" },
            div { class: "flex flex-col gap-5 text-white text-base",
                // Standalone sections without children
                StandaloneSection {
                    label: tr.home,
                    selected: selected_section == SelectedSection::HomePage,
                    to: Route::HomePage { lang, agit_id },
                }
                // Sections with children
                SectionWithChildren {
                    label: tr.orders,
                    selected: selected_section == SelectedSection::Orders,
                    Item { selected: selected_item == SelectedItem::SalesRequest,
                        Link { to: Route::SalesRequest{ lang, agit_id }, {tr.sales_request} }
                    }
                    Item { selected: selected_item == SelectedItem::ShippingLabel,
                        Link { to: Route::ShippingLabel { lang, agit_id }, {tr.shipping_label} }
                    }
                }
                SectionWithChildren {
                    label: tr.management,
                    selected: selected_section == SelectedSection::Management,
                    Item { selected: selected_item == SelectedItem::Artworks,
                        Link { to: Route::Artworks { lang, agit_id }, {tr.artworks} }
                    }
                    Item { selected: selected_item == SelectedItem::Collections,
                        Link { to: Route::Collections { lang, agit_id }, {tr.collections} }
                    }
                    Item { selected: selected_item == SelectedItem::Artist,
                        Link { to: Route::Artist { lang, agit_id }, {tr.artists} }
                    }
                }
                SectionWithChildren {
                    label: tr.community,
                    selected: selected_section == SelectedSection::Community,
                    Item { selected: selected_item == SelectedItem::Collectors,
                        Link { to: Route::Collectors { lang, agit_id }, {tr.collectors} }
                    }
                    Item { selected: selected_item == SelectedItem::Dao,
                        Link { to: Route::Dao { lang, agit_id }, {tr.dao} }
                    }
                    Item { selected: selected_item == SelectedItem::Oracle,
                        Link { to: Route::Oracle { lang, agit_id }, {tr.oracle} }
                    }
                    Item { selected: selected_item == SelectedItem::Faq,
                        Link { to: Route::Faq { lang, agit_id }, {tr.faq} }
                    }
                }
                // Standalone section without children
                StandaloneSection {
                    label: tr.customers,
                    selected: selected_section == SelectedSection::Customers,
                    to: Route::Customers { lang, agit_id },
                }
                // Section with children
                SectionWithChildren {
                    label: tr.analytics,
                    selected: selected_section == SelectedSection::Analytics,
                    Item { selected: selected_item == SelectedItem::Report,
                        Link { to: Route::Report { lang, agit_id }, {tr.report} }
                    }
                    Item { selected: selected_item == SelectedItem::Traffic,
                        Link { to: Route::Traffic { lang, agit_id }, {tr.traffic} }
                    }
                }
                // Standalone sections without children
                StandaloneSection {
                    label: tr.design,
                    selected: selected_section == SelectedSection::Design,
                    to: Route::Design { lang, agit_id },
                }
                StandaloneSection {
                    label: tr.extension_tool,
                    selected: selected_section == SelectedSection::ExtensionTool,
                    to: Route::ExtensionTool { lang, agit_id },
                }
            }
        }
    }
}




// Component for standalone sections without children
#[component]
fn StandaloneSection(
    label: String,
    selected: bool,
    to: Route,
) -> Element {
    rsx! {
        Link {
            to: to,
            div {
                class: format!(
                    "hover:text-primary cursor-pointer transition-colors duration-200 ease-in-out",
                ),
                div {
                    class: format!(
                        "flex items-center py-2.5 gap-2 {}",
                        if selected { "text-primary" } else { "" },
                    ),
                    {label}
                }
            }
        }
    }
}




// Component for sections with children
#[component]
fn SectionWithChildren(
    label: String,
    #[props(default = VNode::empty())] children: Element,
    selected: bool,
) -> Element {
    rsx! {
        div {
            class: "relative",
            {
                if selected {
                    rsx! {
                        div {
                            class: "absolute left-0 top-0 bg-primary",
                            style: "width: 1px; height: 38%;",
                        }
                    }
                } else {
                    rsx! {}
                }
            }
            div {
                class: format!(
                    "pl-5 hover:text-primary cursor-pointer transition-colors duration-200 ease-in-out",
                ),
                div {
                    class: format!(
                        "flex items-center py-2.5 gap-2 {}",
                        if selected { "text-primary" } else { "" },
                    ),
                    {label}
                }
                div { class: "ml-5", {children} }
            }
        }
    }
}

#[component]
fn Item(selected: bool, children: Element) -> Element {
    rsx! {
        div { 
            class: "flex items-center relative",
            {
                if selected {
                    rsx! {
                        span { 
                            class: "text-primary absolute -left-10  top-1/2 transform -translate-y-1/2",
                            "↪" 
                        }
                    }
                } else {
                    rsx! {}
                }
            }
            div { 
                class: format!("{}", if selected { "text-primary" } else { "text-inherit" }),
                {children}
            }
        }
    }
}