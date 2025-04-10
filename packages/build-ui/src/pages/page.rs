use bdk::prelude::*;

use crate::{components::ServiceLogo, pages::controllers::Controller};

use super::controllers::{Blockchain, Wallet};

#[component]
pub fn RootPage(lang: Language) -> Element {
    // let popup: PopupService = use_context();
    let tr: RootPageTranslate= translate(&lang); 
    let mut ctrl  =Controller::new(lang)?;
    // let blockchain = ctrl.blockchains();
    rsx! {
        div {
            class: "flex flex-col items-center justify-center min-h-screen text-center",
            
             ServiceLogo { class:"mb-10",  width:"249.98", height:"55.31" }

            h2 {
                class: "text-2xl text-white mb-4 max-w-xl",
                {tr.description}
            }
            p {
               class:"font-light font-aleo text-white mb-8",
                {tr.sub_description}
            }

            button {
                class: "bg-teal-500 text-black py-3 px-6 text-sm cursor-pointer hover:bg-teal-600",
                onclick: {
                    // let mut popup_clone = popup.clone();
                    move |_| {
                       ctrl.open_blockchain_modal();
                    }
                },
                {tr.button_text},
            }
        }
        }
    }



#[component]
pub fn BlockchainSelectionModal(
        show:bool,
        on_back: EventHandler<()>,
        on_select: EventHandler<String>,
        blockchains: Vec<Blockchain>,
        lang: Language,
    )->Element{
      let tr: RootPageTranslate= translate(&lang);
        rsx! {
            div {
                class: "fixed inset-0 z-50",
                onclick: move |_| on_back.call(()),
                div {
                    class: "fixed inset-0 flex items-center justify-center p-4",
                    onclick: move |e| e.stop_propagation(),
                    div { 
                        class: "bg-popup-bg border border-border-primary rounded-lg w-full max-w-md shadow-[0_0_40px_10px_rgba(255,41,144,0.5)]",
                        div { 
                            class: "flex items-center justify-between px-6 pt-6 border-border-bg",
                            h2 { 
                                class: "text-xl font-semibold text-white",
                                {tr.modal_title}
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
                                
    
                              {  blockchains.into_iter().enumerate().map(|(_i, blockchain)| {
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
        lang: Language,
    ) -> Element {
        let tr: RootPageTranslate= translate(&lang);
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
                                {tr.modal_wallet_title}
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
    
                                {wallets.into_iter().enumerate().map(|(_i, wallet)| {
                                    rsx! {
                                        div {
                                            class: "flex items-center p-3 border border-border-primary rounded cursor-pointer hover:bg-[#ffffff30]",
                                            onclick: move |_| on_connect.call(wallet.name.clone()),
                                            
                                            // MetaMask logo (icon)
                                            div {
                                                class: "w-6 h-6 mr-3",
                                                // svg {
                                                //     class: "w-full h-full",
                                                //     view_box: "0 0 24 24",
                                                //     fill: "#E2761B",
                                                //     path { d: "M21.5,12.5v-1l-9-4l-9,4v1l9-4L21.5,12.5z M12.5,4.5l9,4v-1l-9-4l-9,4v1L12.5,4.5z M21.5,15.5v-1l-9,4l-9-4v1l9,4L21.5,15.5z" }
                                                // }
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
        on_submit: EventHandler<(String, String)>,
        lang: Language,
    ) -> Element {
        let tr: RootPageTranslate= translate(&lang);
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
                                {tr.modal_user_info_title}
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
                                {tr.modal_user_info_description}
                            }
                            
                            // Display name input
                            div {
                                class: "mb-4",
                                label {
                                    class: "block text-sm font-medium text-popup-label mb-2",
                                    {tr.modal_user_info_name}
                                }
                                input {
                                    class: "w-full bg-transparent border border-[1px] text-white text-sm rounded-none p-2 focus:outline-none focus:border-primary",
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
                                    {tr.modal_user_info_email}
                                }
                                
                                input {
                                    class: "w-full bg-transparent border border-[1px] text-white text-sm rounded-none p-2 focus:outline-none focus:border-primary",
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
                                    class: "mr-2 h-4 w-4  border border-btn-signin checked:bg-white checked:border-black checked:text-black text-sm",
                                    type: "checkbox",
                                    checked: "{terms_accepted}",
                                    oninput: move |evt| terms_accepted.set(evt.value().parse().unwrap_or(false)),
                                }
                                label {
                                    class: "text-sm text-popup-label",
                                    {tr.modal_user_info_terms1},
                                  
                                }
                            }
                            
                            // Announcements checkbox
                            div {
                                class: "flex items-center mb-4",
                                input {
                                    id: "announcements",
                                    class: "mr-2 h-4 w-4 border border-btn-signin checked:bg-white checked:border-black checked:text-black text-sm",
                                    type: "checkbox",
                                    checked: "{receive_announcements}",
                                    oninput: move |evt| receive_announcements.set(evt.value().parse().unwrap_or(false)),
                                }
                                label {
                                    class: "text-sm text-popup-label",
                                   {tr.modal_user_info_terms2},
                                }
                            }
                            
                            // Submit button
                            button {
                                class: "w-full bg-white text-btn-signin py-2 px-4 hover:bg-gray-600 disabled:bg-popup-btn-disabled disabled:cursor-not-allowed",
                                disabled: "{display_name.read().is_empty() || email.read().is_empty() || !*terms_accepted.read()}",
                                onclick: move |_| {
                                    if !display_name.read().is_empty() && !email.read().is_empty() && *terms_accepted.read() {
                                        on_submit.call((display_name.read().clone(), email.read().clone()));
                                    }
                                },
                                {tr.finish_signup}
                            }
                        }
                    }
                }
            }
        }
   }
#[component]
pub fn BuildYourAgitModal(
        show: bool,
        on_back: EventHandler<()>,
        on_submit: EventHandler<(String, String)>,
        lang: Language,
    ) -> Element {
        let tr: RootPageTranslate= translate(&lang);
        let mut display_name = use_signal(|| String::new());
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
                                {tr.modal_user_info_title}
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
                                {tr.modal_user_info_description}
                            }
                            
                            // Display name input
                            div {
                                class: "mb-4",
                                label {
                                    class: "block text-sm font-medium text-popup-label mb-2",
                                    {tr.modal_user_info_name}
                                }
                                input {
                                    class: "w-full bg-transparent border border-[1px] text-white text-sm rounded-none p-2 focus:outline-none focus:border-primary",
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
                                    "Short URL"
                                }
                                
                                input {
                                    class: "w-full bg-transparent border border-[1px] text-white text-sm rounded-none p-2 focus:outline-none focus:border-primary",
                                    placeholder: "Enter your email",
                                    value: "dagit.com",
                                    readonly: "readonly",
                                    // oninput: move |evt| email.set(evt.value().clone()),
                                    type: "text"
                                }
                            }
                            
                            // Terms of service checkbox
                            div {
                                class: "flex items-center mb-2",
                                input {
                                    id: "terms",
                                    class: "mr-2 h-4 w-4  border border-btn-signin checked:bg-white checked:border-black checked:text-black text-sm",
                                    type: "checkbox",
                                    checked: "{terms_accepted}",
                                    oninput: move |evt| terms_accepted.set(evt.value().parse().unwrap_or(false)),
                                }
                                label {
                                    class: "text-sm text-popup-label",
                                    {tr.modal_user_info_terms1},
                                  
                                }
                            }
                            
                            // Announcements checkbox
                            div {
                                class: "flex items-center mb-4",
                                input {
                                    id: "announcements",
                                    class: "mr-2 h-4 w-4 border border-btn-signin checked:bg-white checked:border-black checked:text-black text-sm",
                                    type: "checkbox",
                                    checked: "{receive_announcements}",
                                    oninput: move |evt| receive_announcements.set(evt.value().parse().unwrap_or(false)),
                                }
                                label {
                                    class: "text-sm text-popup-label",
                                   {tr.modal_user_info_terms2},
                                }
                            }
                            
                            // Submit button
                            button {
                                class: "w-full bg-white text-btn-signin py-2 px-4 hover:bg-gray-600 disabled:bg-popup-btn-disabled disabled:cursor-not-allowed",
                                disabled: "{display_name.read().is_empty() || !*terms_accepted.read()}",
                                onclick: move |_| {
                                    if !display_name.read().is_empty() && *terms_accepted.read() {
                                        on_submit.call((display_name.read().clone(), format!("dagit.com/{}", display_name.read())));
                                    }
                                },
                                {tr.build_your_agit}
                            }
                        }
                    }
                }
            }
        }
    }

translate!{
    RootPageTranslate;
    title: {
        en: "Agit",
        ko: "",
    },
    description: {
        en: "Blockchain-based artwork certificates, seamless data management, digital gallery solutions.",
        ko: "블록체인 기반의 작품 인증서, 원활한 데이터 관리, 디지털 갤러리 솔루션.",
    },
    sub_description: {
        en: "Let's unlock new possibilities with your own Agit today!",
        ko: "오늘 당신만의 아지트로 새로운 가능성을 열어보세요!",
    },
    button_text: {
        en: "Build your Agit",
        ko: "아지트를 만들어보세요",
    },
    modal_title: {
        en: "Choose Blockchain",
        ko: "블록체인 선택",
    },
    modal_wallet_title: {
        en: "Connect Wallet",
        ko: "지갑 연결",
    },
    modal_user_info_title: {
        en: "You are almost there!",
        ko: "거의 다 왔습니다!",
    },
    modal_user_info_description: {
        en: "Choose a display name and enter your email address",
        ko: "디스플레이 이름을 선택하고 이메일 주소를 입력하세요.",
    },
    modal_user_info_name: {
        en: "Agit Name",
        ko: "",
    },
    modal_user_info_email: {
        en: "Email",
        ko: "이메일",
    },
    modal_user_info_terms1: {
        en: "I have read and accept the Terms of Service.",
        ko: "서비스 약관을 읽고 동의합니다.",
    },
    modal_user_info_terms2: {
        en: "I want to receive announcements and news from d.AgIt.",
        ko: "",
    },
    finish_signup: {
        en: "Finished Sign-up",
        ko: "가입 완료",
    },
    build_your_agit:{
        en: "Build your Agit",
        ko: "",
    },
   your_agit_name:{
        en: "What is your Agit name",
        ko: "",
    }
}

