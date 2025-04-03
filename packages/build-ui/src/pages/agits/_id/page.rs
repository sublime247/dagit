use bdk::prelude::*;

#[component]
pub fn HomePage(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Home {agit_id}" }
    }
}

#[component]
pub fn SalesRequest(lang:Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "SalesRequest {agit_id}" }
    }
}


#[component]
pub fn ShippingLabel(lang:Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "ShippingLabel {agit_id}" }
    }
}
#[component]
pub fn Artworks(lang:Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Artworks {agit_id}" }
    }
}
#[component]
pub fn Collections(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Collections {agit_id}" }
    }
}
#[component]
pub fn Dao(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Dao {agit_id}" }
    }
}
#[component]
pub fn Oracle(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Oracle {agit_id}" }
    }
}
#[component]
pub fn Faq(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Faq {agit_id}" }
    }
}
#[component]
pub fn Report(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Report {agit_id}" }
    }
}
#[component]
pub fn Traffic(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Traffic {agit_id}" }
    }
}
#[component]
pub fn Customers(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Customers {agit_id}" }
    }
}
#[component]
pub fn Design(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Design {agit_id}" }
    }
}



#[component]
pub fn ExtensionTool(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "ExtensionTool {agit_id}" }
    }
}



#[component]
pub fn Collectors(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Collectors {agit_id}" }
    }
}


#[component]
pub fn Artist(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Artist {agit_id}" }
    }
}