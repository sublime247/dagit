use bdk::prelude::*;

use crate::{components::ServiceLogo, pages::controllers::Controller};

#[component]
pub fn RootPage(lang: Language) -> Element {
    // let popup: PopupService = use_context();
    let mut ctrl  =Controller::new(lang)?;
    // let blockchain = ctrl.blockchains();
    rsx! {
        div {
            class: "flex flex-col items-center justify-center min-h-screen text-center",
            
             ServiceLogo { class:"mb-10",  width:"249.98", height:"55.31" }

            h2 {
                class: "text-2xl text-white mb-4 max-w-xl",
                "Blockchain-based artwork certificates, seamless data management, digital gallery solutions."
            }
            p {
               class:"font-light font-aleo text-white mb-8",
                "Let's unlock new possibilities with your own Agit today!"
            }

            button {
                class: "bg-teal-500 text-black py-3 px-6 text-sm cursor-pointer hover:bg-teal-600",
                onclick: {
                    // let mut popup_clone = popup.clone();
                    move |_| {
                       ctrl.open_blockchain_modal();
                    }
                },
                "Build your Agit",
            }
        }
        }
    }


