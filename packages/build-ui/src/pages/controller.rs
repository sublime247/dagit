use bdk::prelude::{dioxus_popup::PopupService, *};

use crate::pages::components::{
    BuildAgitModal, BuildAgitResult, ConnectWalletModal, UserInfoModal, UserInfoResult,
};
use crate::services::user_service::{Chain, Status, UserService, Wallet};

use super::components::BlockchainSelectionModal;
use super::i18n::{
    BlockchainSelectionModalTranslate, ConnectWalletModalTranslate, UserInfoModalTranslate,
};

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    popup: PopupService,
    user: UserService,
}

impl Controller {
    pub fn new(lang: Language) -> Result<Self, RenderError> {
        let popup: PopupService = use_context();
        let user: UserService = use_context();
        let ctrl: Controller = Self { lang, popup, user };
        Ok(ctrl)
    }

    pub fn open_select_blockchain_modal(&self) {
        let mut popup = self.popup.clone();
        let tr: BlockchainSelectionModalTranslate = translate(&self.lang);
        let ctrl = self.clone();
        popup
            .open(rsx! {
                BlockchainSelectionModal {
                    lang: self.lang,
                    on_select: move |chain| {
                        tracing::debug!("Selected blockchain: {:?}", chain);
                        ctrl.open_connect_wallet_modal(chain);
                    },
                }
            })
            .with_id("select_blockchain_modal")
            .with_title(tr.title);
    }

    pub fn open_connect_wallet_modal(&self, chain: Chain) {
        let mut popup = self.popup.clone();
        let tr: ConnectWalletModalTranslate = translate(&self.lang);
        let mut ctrl = self.clone();

        popup
            .open(rsx! {
                ConnectWalletModal {
                    lang: self.lang,
                    on_select: move |wallet: Wallet| {
                        tracing::debug!("Selected wallet: {:?}", wallet);
                        spawn(async move {
                            if let Ok(status) = ctrl.user.login(chain, wallet).await {
                                tracing::debug!("Login status: {:?}", status);
                                match status {
                                    Status::Login => {
                                        popup.close();
                                    }
                                    Status::Signup { principal: _, profile_url: _, nickname, email } => {
                                        ctrl.open_user_info_modal(nickname, email);
                                    }
                                    _ => {
                                        popup.close();
                                    }
                                }
                            }
                        });
                    },
                }
            })
            .with_id("connect_wallet_modal")
            .with_title(tr.title);
    }

    // // New method that takes the selected blockchain and wallet
    pub fn open_user_info_modal(&self, nickname: Option<String>, email: Option<String>) {
        let mut popup = self.popup.clone();
        let tr: UserInfoModalTranslate = translate(&self.lang);
        let mut ctrl = self.clone();
        popup
            .open(rsx! {
                UserInfoModal {
                    nickname,
                    email,
                    lang: self.lang,
                    on_button_click: move |result: UserInfoResult| {
                        tracing::debug!("Selected wallet: {:?}", result);
                        spawn(async move {
                            if let Ok(status) = ctrl
                                .user
                                .signup(
                                    result.email,
                                    result.nickname,
                                    result.terms_agreed_at,
                                    result.ads_agreed_at,
                                )
                                .await
                            {
                                match status {
                                    Status::Login => {
                                        popup.close();
                                    }
                                    _ => {
                                        btracing::error!("{} : {:?}", tr.signup_failed, status);
                                        popup.close();
                                    }
                                }
                            }
                        });
                        popup.close();
                    },
                }
            })
            .with_id("user_info_modal")
            .with_title(tr.title);
    }

    #[allow(dead_code)]
    pub fn open_build_agit_modal(&self) {
        let mut popup = self.popup.clone();
        let tr: UserInfoModalTranslate = translate(&self.lang);
        // let ctrl = self.clone();

        popup
            .open(rsx! {
                BuildAgitModal {
                    lang: self.lang,
                    on_button_click: move |result: BuildAgitResult| {
                        tracing::debug!("Result: {:?}", result);
                        popup.close();
                    },
                }
            })
            .with_id("user_info_modal")
            .with_title(tr.title);
    }
}
