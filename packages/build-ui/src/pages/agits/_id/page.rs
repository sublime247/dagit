use bdk::prelude::*;

#[component]
pub fn HomePage(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Home {agit_id}" }
    }
}

#[component]
pub fn SalesRequestPage(lang:Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "SalesRequest {agit_id}" }
    }
}


#[component]
pub fn ShippingLabelPage(lang:Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "ShippingLabel {agit_id}" }
    }
}
#[component]
pub fn ArtworksPage(lang:Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Artworks {agit_id}" }
    }
}

#[component]
pub fn DaoPage(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Dao {agit_id}" }
    }
}
#[component]
pub fn OraclePage(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Oracle {agit_id}" }
    }
}
#[component]
pub fn FaqPage(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Faq {agit_id}" }
    }
}
#[component]
pub fn ReportPage(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Report {agit_id}" }
    }
}
#[component]
pub fn TrafficPage(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Traffic {agit_id}" }
    }
}
#[component]
pub fn CustomersPage(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Customers {agit_id}" }
    }
}
#[component]
pub fn DesignPage(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Design {agit_id}" }
    }
}



#[component]
pub fn ExtensionToolPage(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "ExtensionTool {agit_id}" }
    }
}



#[component]
pub fn CollectorsPage(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Collectors {agit_id}" }
    }
}


#[component]
pub fn ArtistPage(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Artist {agit_id}" }
    }
}