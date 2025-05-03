use bdk::prelude::*;
#[component]
pub fn Table(header: Element, #[props(default = None)] body: Option<Element>) -> Element {
    rsx! {
        table { class: "w-full text-sm text-left min-w-[800px]",
            thead { class: "text-xs uppercase bg-table-background border-b border-border-primary",
                tr { class: "text-gray-400", {header} }
            }
            tbody {
                if let Some(body) = body {
                    {body}
                }
            }
        }
    }
}
