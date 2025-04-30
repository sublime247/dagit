use crate::config;
use bdk::prelude::*;
use common::error::ServiceError;
use common::{Result, tables::users::User};
use google_wallet::{FirebaseWallet, WalletEvent};

#[derive(Debug, Eq, PartialEq)]
pub enum Status {
    Signup {
        principal: String,
        profile_url: Option<String>,
        nickname: Option<String>,
        email: Option<String>,
    },
    Login,
    Logout,
}

#[derive(Debug, Copy, Clone, Translate)]
pub enum Chain {
    InternetComputer,
}

#[derive(Debug, Copy, Clone)]
pub enum Wallet {
    Google,
}

#[derive(Debug, Clone)]
pub enum WalletSigner {
    Firebase(FirebaseWallet),
}

pub fn get_firebase_wallet() -> google_wallet::FirebaseWallet {
    #[cfg(not(feature = "web"))]
    let firebase = google_wallet::FirebaseWallet::default();

    #[cfg(feature = "web")]
    let firebase = {
        let conf = config::get();
        let mut firebase = google_wallet::FirebaseWallet::new(
            conf.firebase.api_key.clone(),
            conf.firebase.auth_domain.clone(),
            conf.firebase.project_id.clone(),
            conf.firebase.storage_bucket.clone(),
            conf.firebase.messaging_sender_id.clone(),
            conf.firebase.app_id.clone(),
            conf.firebase.measurement_id.clone(),
        );
        let _ = firebase.try_setup_from_storage();
        tracing::debug!("get_firebase_wallet: firebase={:?}", firebase);
        firebase
    };
    firebase
}

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub principal: String,
    pub email: Option<String>,
    pub nickname: Option<String>,
    pub profile_url: Option<String>,
}

impl UserInfo {
    pub fn new(principal: String, email: String, nickname: String, profile_url: String) -> Self {
        Self {
            principal,
            email: Some(email),
            nickname: Some(nickname),
            profile_url: Some(profile_url),
        }
    }
}

impl Default for UserInfo {
    fn default() -> Self {
        Self {
            principal: "".to_string(),
            email: None,
            nickname: None,
            profile_url: None,
        }
    }
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct UserService {
    pub signer: Signal<Option<WalletSigner>>,
    pub user_info: Signal<Option<UserInfo>>,
}

impl UserService {
    pub fn init() -> Self {
        // let firebase = get_firebase_wallet();

        UserService {
            signer: use_signal(|| None),
            user_info: use_signal(|| None),
        }
    }

    pub async fn firebase_login(&mut self) -> Result<Status> {
        let mut firebase = get_firebase_wallet();
        let (evt, user_info, principal) = match firebase.request_wallet_with_google().await {
            Ok(evt) => {
                let principal = firebase.get_principal();
                let user_info = firebase.get_user_info();
                if user_info.is_none() || principal.is_empty() {
                    return Err(ServiceError::Unauthorized);
                }
                (evt, user_info.unwrap(), principal)
            }
            Err(e) => {
                tracing::error!("Error during login: {:?}", e);
                return Err(ServiceError::Unauthorized);
            }
        };
        let next_status = match evt {
            WalletEvent::Login => {
                tracing::debug!("UserService::firebase_login: login");
                self.user_info.set(Some(UserInfo {
                    principal,
                    email: Some(user_info.0),
                    nickname: Some(user_info.1),
                    profile_url: Some(user_info.2),
                }));
                Status::Login
            }
            WalletEvent::Signup => {
                tracing::debug!("UserService::firebase_login: signup");
                let client = User::get_client(config::get().api_url);
                let res = client.get_user_by_address(principal.clone()).await;
                match res {
                    Ok(user) => {
                        self.user_info.set(Some(UserInfo {
                            principal: user.address,
                            email: Some(user.email),
                            nickname: Some(user.name),
                            profile_url: user.profile_url,
                        }));
                        Status::Login
                    }
                    Err(e) => {
                        tracing::error!("Error during signup: {:?}", e);
                        Status::Signup {
                            principal,
                            email: Some(user_info.0),
                            nickname: Some(user_info.1),
                            profile_url: Some(user_info.2),
                        }
                    }
                }
            }
            WalletEvent::Logout => {
                tracing::debug!("UserService::firebase_login: logout");
                Status::Logout
            }
        };

        if next_status == Status::Login {
            self.signer.set(Some(WalletSigner::Firebase(firebase)));
            rest_api::set_signer(Box::new(*self));
        } else {
            self.signer.set(None);
        }
        Ok(next_status)
    }
}

impl rest_api::Signer for UserService {
    fn signer(&self) -> String {
        match (self.signer)() {
            Some(WalletSigner::Firebase(firebase)) => firebase.get_principal(),
            None => "".to_string(),
        }
    }

    fn sign(
        &self,
        msg: &str,
    ) -> std::result::Result<rest_api::Signature, Box<dyn std::error::Error>> {
        tracing::debug!("UserService::sign: msg={}", msg);
        match (self.signer)() {
            Some(WalletSigner::Firebase(firebase)) => {
                if !firebase.get_login() {
                    tracing::debug!("UserService::sign: not login {firebase:?}");
                    return Err(ServiceError::Unauthorized.into());
                }

                let sig = firebase.sign(msg);
                if sig.is_none() {
                    return Err(ServiceError::Unauthorized.into());
                }
                let sig = rest_api::Signature {
                    signature: sig.unwrap().as_ref().to_vec(),
                    public_key: firebase.public_key().unwrap_or_default(),
                    algorithm: rest_api::signature::SignatureAlgorithm::EdDSA,
                };

                return Ok(sig);
            }
            None => {
                tracing::debug!("UserService::sign: no signer");
                return Err(ServiceError::Unauthorized.into());
            }
        }
    }
}
