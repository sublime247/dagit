#![allow(unused)]
use bdk::prelude::{dioxus_popup::PopupService, *};
#[derive(Clone, Debug, PartialEq)]
pub struct  Blockchain{
    name: String,
    id:usize,
 }
 #[derive(Clone, Debug, PartialEq)]
pub struct  Wallet{
    name: String,
    id:usize,
 }



#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller{
    popup: PopupService,
    lang: Language,
    blockchains: Signal<Vec<Blockchain>>,
    wallets: Signal<Vec<Wallet>>,
}


impl Controller{
    pub fn new(lang: Language) -> Result<Self, RenderError> {
        let mut popup: PopupService = use_context();
        let blockchains = use_signal(||{
            (0..5).map(|id| Blockchain{
                id,
                name: "Ethereum".to_string(),
                
            }).collect::<Vec<_>>()

        });
       
       let wallets = use_signal(||{
            (0..5).map(|id| Wallet{
                id,
                name: "Metamask".to_string(),
                
            }).collect::<Vec<_>>()

        

       });

        let ctrl:Controller= Self {
            lang,
            popup,
            blockchains,
            wallets,
        };
         use_context_provider(||ctrl);
        Ok(ctrl)
    }


pub fn open_blockchain_modal(&mut self){
    let mut popup_clone = self.popup.clone();
    popup_clone.open(
        rsx!{
            BlockchainSelectionModal {
                show: true,
                on_back: move |_| {
                    popup_clone.close();
                },
                on_select: move |blockchain| {
                    tracing::debug!("Selected blockchain: {}", blockchain);
                    popup_clone.close();
                },
                blockchains: self.blockchains.read().clone(),
            }
        }
    ).with_id("select_blockchain_modal");
}

pub fn open_wallet_modal(&mut self){
    let mut popup_clone = self.popup.clone();
    popup_clone.open(
        rsx!{
            ConnectWalletModal {
                show: true,
                on_back: move |_| {
                    popup_clone.close();
                },
                on_connect: move |wallet| {
                    tracing::debug!("Selected wallet: {}", wallet);
                    popup_clone.close();
                },
                wallets: self.wallets.read().clone(),
            }
        }
    ).with_id("select_wallet_modal");
}




}






#[component]
pub fn BlockchainSelectionModal(
    show:bool,
    on_back: EventHandler<()>,
    on_select: EventHandler<String>,
    blockchains: Vec<Blockchain>,

)->Element{

    
    rsx! {
        div {
            class: "fixed inset-0 z-50",
            onclick: move |_| on_back.call(()),
            
            // Modal content
            div {
                class: "fixed inset-0 flex items-center justify-center p-4",
                onclick: move |e| e.stop_propagation(),
                div { 
                    class: "bg-popup-bg border border-border-primary rounded-lg w-full max-w-md shadow-[0_0_40px_10px_rgba(255,41,144,0.5)]",
                    
                    // Modal header
                    div { 
                        class: "flex items-center justify-between px-6 pt-6 border-border-bg",
                        h2 { 
                            class: "text-xl font-semibold text-white",
                            "Choose Blockchain"
                        }
                        button {
                            class: "text-gray-400 hover:text-white",
                            onclick: move |_| on_back.call(()),
                            svg {
                                class: "w-6 h-6",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                stroke_width: "2",
                                fill: "none",
                                path { d: "M6 18L18 6M6 6l12 12" }
                            }
                        }
                    }
                    
                    // Modal body - blockchain options
                    div { 
                        class: "px-6 py-5",
                        
                        // List of blockchain options
                        div {
                            class: "space-y-2",
                            
                            // Ethereum options (5 of them as shown in screenshot)

                          {  blockchains.into_iter().enumerate().map(|(i, blockchain)| {
                                rsx! {
                                    div {
                                        class: "flex items-center p-3 border border-border-primary rounded cursor-pointer hover:bg-opacity-25 hover:bg-primary",
                                        onclick: move |_| on_select.call(blockchain.name.clone()),
                                        
                                        // Ethereum logo
                                        div {
                                            class: "w-6 h-6 mr-3",
                                            svg {
                                                class: "w-full h-full",
                                                view_box: "0 0 24 24",
                                                fill: "#627EEA",
                                                path { d: "M11.944 17.97L4.58 13.62 11.943 24l7.37-10.38-7.372 4.35h.003zM12.056 0L4.69 12.223l7.365 4.354 7.365-4.35L12.056 0z" }
                                            }
                                        }
                                        
                                        // Blockchain name
                                        span {
                                            class: "text-white",
                                            "{blockchain.name}"
                                        }
                                    }
                                }
                            })}

                        }
                    }
                }
            }
        }
    }
}


#[component]
pub fn ConnectWalletModal(
    show: bool,
    on_back: EventHandler<()>,
    on_connect: EventHandler<String>,
    wallets: Vec<Wallet>,
) -> Element {
    if !show {
        return rsx!(div {});
    }

    rsx! {
        div {
            class: "fixed inset-0 z-50",
            onclick: move |_| on_back.call(()),
            
            // Modal content
            div {
                class: "fixed inset-0 flex items-center justify-center p-4",
                onclick: move |e| e.stop_propagation(),
                div { 
                    class: "bg-popup-bg border border-border-primary rounded-lg w-full max-w-md shadow-[0_0_40px_10px_rgba(255,41,144,0.5)]",
                    
                    // Modal header
                    div { 
                        class: "flex items-center justify-between px-6 pt-6 border-border-bg",
                        h2 { 
                            class: "text-xl font-semibold text-white",
                            "Connect Wallet"
                        }
                        button {
                            class: "text-gray-400 hover:text-white",
                            onclick: move |_| on_back.call(()),
                            svg {
                                class: "w-6 h-6",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                stroke_width: "2",
                                fill: "none",
                                path { d: "M6 18L18 6M6 6l12 12" }
                            }
                        }
                    }
                    
                    // Modal body - wallet options
                    div { 
                        class: "px-6 py-5",
                        
                        // List of wallet options
                        div {
                            class: "space-y-2",
                            
                            // MetaMask options (3 of them as shown in screenshot)

                            {wallets.into_iter().enumerate().map(|(i, wallet)| {
                                rsx! {
                                    div {
                                        class: "flex items-center p-3 border border-border-primary rounded cursor-pointer hover:bg-opacity-20 hover:bg-white",
                                        onclick: move |_| on_connect.call(wallet.name.clone()),
                                        
                                        // MetaMask logo (fox-like icon)
                                        div {
                                            class: "w-6 h-6 mr-3",
                                            svg {
                                                class: "w-full h-full",
                                                view_box: "0 0 24 24",
                                                fill: "#E2761B",
                                                path { d: "M21.5,12.5v-1l-9-4l-9,4v1l9-4L21.5,12.5z M12.5,4.5l9,4v-1l-9-4l-9,4v1L12.5,4.5z M21.5,15.5v-1l-9,4l-9-4v1l9,4L21.5,15.5z" }
                                            }
                                        }
                                        
                                        // Wallet name
                                        span {
                                            class: "text-white",
                                            "{wallet.name}"
                                        }
                                    }
                                }
                            })}
                           
                        }
                    }
                }
            }
        }
    }
}




#[component]
pub fn UserInfoModal(
    show: bool,
    on_back: EventHandler<()>,
    on_submit: EventHandler<(String, String)>
) -> Element {
    let mut display_name = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut terms_accepted = use_signal(|| false);
    let mut receive_announcements = use_signal(|| false);

    if !show {
        return rsx!(div {});
    }

    rsx! {
        div {
            class: "fixed inset-0 z-50",
            onclick: move |_| on_back.call(()),
            
            // Modal content
            div {
                class: "fixed inset-0 flex items-center justify-center p-4",
                onclick: move |e| e.stop_propagation(),
                div { 
                    class: "bg-popup-bg border border-border-primary rounded-lg w-full max-w-md shadow-[0_0_40px_10px_rgba(255,41,144,0.5)]",
                    
                    // Modal header
                    div { 
                        class: "flex items-center justify-between px-6 pt-6 border-border-bg",
                        h2 { 
                            class: "text-xl font-semibold text-white",
                            "You are almost there!"
                        }
                        button {
                            class: "text-gray-400 hover:text-white",
                            onclick: move |_| on_back.call(()),
                            svg {
                                class: "w-6 h-6",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                stroke_width: "2",
                                fill: "none",
                                path { d: "M6 18L18 6M6 6l12 12" }
                            }
                        }
                    }
                    
                    // Modal body - user info form
                    div { 
                        class: "px-6 py-5",
                        
                        p {
                            class: "text-sm text-white mb-4",
                            "Choose a display name and enter your email address"
                        }
                        
                        // Display name input
                        div {
                            class: "mb-4",
                            label {
                                class: "block text-sm font-medium text-popup-label mb-2",
                                "Agit Name"
                            }
                            input {
                                class: "w-full bg-transparent border border-popup-border text-white text-sm rounded-none p-2",
                                placeholder: "Display name",
                                value: "{display_name}",
                                oninput: move |evt| display_name.set(evt.value().clone()),
                            }
                        }
                        
                        // Email input
                        div {
                            class: "mb-4",
                            label {
                                class: "block text-sm font-medium text-popup-label mb-2",
                                "Email"
                            }
                            input {
                                class: "w-full bg-transparent border border-popup-border text-white text-sm rounded-none p-2",
                                placeholder: "Enter your email",
                                value: "{email}",
                                oninput: move |evt| email.set(evt.value().clone()),
                                type: "email"
                            }
                        }
                        
                        // Terms of service checkbox
                        div {
                            class: "flex items-center mb-2",
                            input {
                                id: "terms",
                                class: "mr-2",
                                type: "checkbox",
                                checked: "{terms_accepted}",
                                oninput: move |evt| terms_accepted.set(evt.value().parse().unwrap_or(false)),
                            }
                            label {
                                class: "text-sm text-white",
                                "I have read and accept the ",
                                span {
                                    class: "underline",
                                    "Terms of Service"
                                },
                                "."
                            }
                        }
                        
                        // Announcements checkbox
                        div {
                            class: "flex items-center mb-4",
                            input {
                                id: "announcements",
                                class: "mr-2",
                                type: "checkbox",
                                checked: "{receive_announcements}",
                                oninput: move |evt| receive_announcements.set(evt.value().parse().unwrap_or(false)),
                            }
                            label {
                                class: "text-sm text-white",
                                "I want to receive announcements and news from d.AgIt."
                            }
                        }
                        
                        // Submit button
                        button {
                            class: "w-full bg-gray-700 text-white py-2 px-4 hover:bg-gray-600 disabled:opacity-50 disabled:cursor-not-allowed",
                            disabled: "{display_name.read().is_empty() || email.read().is_empty() || !*terms_accepted.read()}",
                            onclick: move |_| {
                                if !display_name.read().is_empty() && !email.read().is_empty() && *terms_accepted.read() {
                                    on_submit.call((display_name.read().clone(), email.read().clone()));
                                }
                            },
                            "Finished Sign-up"
                        }
                    }
                }
            }
        }
    }
}