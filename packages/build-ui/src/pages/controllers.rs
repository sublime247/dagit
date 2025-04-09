#![allow(unused)]
use bdk::prelude::{dioxus_popup::PopupService, *};

use crate::pages::page::{BlockchainSelectionModal, ConnectWalletModal, UserInfoModal};
#[derive(Clone, Debug, PartialEq)]
pub struct  Blockchain{
   pub name: String,
    id:usize,
 }
 #[derive(Clone, Debug, PartialEq)]
pub struct  Wallet{
    pub name: String,
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
            (0..4).map(|id| Wallet{
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


    pub fn open_blockchain_modal(&mut self) {
        let mut popup_clone = self.popup.clone();
        let ctrl_clone = self.clone();
        
        popup_clone.open(
            rsx!{
                BlockchainSelectionModal {
                    show: true,
                    on_back: move |_| {
                        popup_clone.close();
                    },
                    on_select: {
                        let mut ctrl = ctrl_clone.clone();
                        move |blockchain| {
                            tracing::debug!("Selected blockchain: {}", blockchain);
                            popup_clone.close();
                            
                            // Store selected blockchain if needed
                            // Then open wallet modal
                            ctrl.open_wallet_modal_with_blockchain(blockchain);
                        }
                    },
                    blockchains: self.blockchains.read().clone(),
                }
            }
        ).with_id("select_blockchain_modal");
    }

    // New method that takes the selected blockchain
    pub fn open_wallet_modal_with_blockchain(&mut self, selected_blockchain: String) {
        let mut popup_clone = self.popup.clone();
        let ctrl_clone = self.clone();
        
        popup_clone.open(
            rsx!{
                ConnectWalletModal {
                    show: true,
                    on_back: {
                        let mut ctrl = ctrl_clone.clone();
                        move |_| {
                            popup_clone.close();
                            // Go back to blockchain selection
                            ctrl.open_blockchain_modal();
                        }
                    },
                    on_connect: {
                        let mut ctrl = ctrl_clone.clone();
                        let blockchain = selected_blockchain.clone();
                        move |wallet| {
                            tracing::debug!("Selected wallet: {}", wallet);
                            popup_clone.close();
                            
                            // Store selected wallet if needed
                            // Then open user info modal
                            ctrl.open_user_info_modal_with_data(blockchain.clone(), wallet);
                        }
                    },
                    wallets: self.wallets.read().clone(),
                }
            }
        ).with_id("select_wallet_modal");
    }

    // New method that takes the selected blockchain and wallet
    pub fn open_user_info_modal_with_data(&mut self, selected_blockchain: String, selected_wallet: String) {
        let mut popup_clone = self.popup.clone();
        let ctrl_clone = self.clone();
        
        popup_clone.open(
            rsx!{
                UserInfoModal {
                    show: true,
                    on_back: {
                        let mut ctrl = ctrl_clone.clone();
                        let blockchain = selected_blockchain.clone();
                        move |_| {
                            popup_clone.close();
                            // Go back to wallet selection
                            ctrl.open_wallet_modal_with_blockchain(blockchain.clone());
                        }
                    },
                    on_submit: {
                        let blockchain = selected_blockchain.clone();
                        let wallet = selected_wallet.clone();
                        move |(name, email)| {
                            tracing::debug!("User info submitted: {} - {}", name, email);
                            tracing::debug!("Selected blockchain: {}, wallet: {}", blockchain, wallet);
                            popup_clone.close();
                            
                            // Here you can handle the final submission with all collected data
                            // For example, call an API or navigate to another page
                        }
                    }
                }
            }
        ).with_id("user_info_modal");
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


pub fn open_user_info_modal(&mut self){
    let mut popup_clone = self.popup.clone();
    popup_clone.open(
        rsx!{
            UserInfoModal {
                show: true,
                on_back: move |_| {
                    popup_clone.close();
                },
                on_submit: move |(name, email)| {
                    tracing::debug!("User info submitted: {} - {}", name, email);
                    popup_clone.close();
                }
            }
        }
    ).with_id("user_info_modal");
}
}









