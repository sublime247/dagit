#![allow(non_snake_case)]
use bdk::prelude::*;
use by_components::loaders::cube_loader::CubeLoader;

use super::components::Header;
use crate::components::popup_zone::PopupZone;
use crate::routes::Route;

#[component]
pub fn SuspenseWrapper(children: Element) -> Element {
    rsx! {
        // div { class: "w-full min-h-dvh",
        Fragment {
            PopupZone {}
            SuspenseBoundary {
                fallback: |_| {
                    rsx! {
                        div { class: "absolute w-screen h-screen top-0 left-0 flex items-center justify-center text-white",
                            CubeLoader {}
                        }
                    }
                },
                {children}
            }
        }
    }
}
//border-radius: 1100px;
//background: radial-gradient(50% 50% at 50% 50%, #30D4A0 0%, rgba(115, 115, 115, 0.00) 100%);

#[component]
pub fn HeaderLayout(lang: Language) -> Element {
    rsx! {
        SuspenseWrapper {
            div { class: "relative min-h-lvh w-screen flex flex-col items-center bg-black",
                div { class: "fixed bg-radial-[50%_50%_at_50%_50%] from-[#30D4A0] to-[#737373/0%] size-100 lg:size-[1100px] -top-50 lg:-top-[800px] -left-50 lg:-left-[480px] rounded-full" }
                div { class: "fixed bg-radial-[50%_50%_at_50%_50%] from-[#FF2990] to-[#737373/0%] size-100 lg:size-[1100px] -right-40 lg:right-[80px] -bottom-60 lg:-bottom-[900px] rounded-full" }
                div { class: "max-w-[1440px] w-full h-full flex flex-col flex-1 z-1",
                    Header { lang }
                    Outlet::<Route> {}
                }
            }
        }
    }
}
