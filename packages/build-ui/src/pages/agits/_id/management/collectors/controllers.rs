#![allow(unused)]
use std::io::Read;

use super::models::*;
use crate::pages::agits::_id::management::Activity;
use bdk::prelude::{dioxus_popup::PopupService, *};
use common::tables::{
    collectors::Collector as CollectorModel,
    prelude::{
        CollectorByIdAction, CollectorCreateRequest, CollectorDeleteRequest, CollectorQuery,
    },
};
use wasm_bindgen_futures::spawn_local;

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    collector: Signal<Vec<Collector>>,
    // asset: Signal<Vec<Assets>>,
    activity: Signal<Vec<Activity>>,
    popup: PopupService,
}
impl Controller {
    pub fn new(lang: Language, agit_id: ReadOnlySignal<i64>) -> Result<Self, RenderError> {
        let mut popup: PopupService = use_context();

        let res = use_server_future(move || async move {
            let endpoint = crate::config::get().api_url;
            let client = CollectorModel::get_client(endpoint);
            client
                .query(CollectorQuery::new(100).with_page(0))
                .await
                .unwrap_or_default()
        })?;
        tracing::debug!("res: {:?}", res);

        let collector = use_signal(|| {
            (1..15)
                .map(|id| Collector {
                    id: id.to_string(),
                    total_volume: 2.370,
                    total_volume_usd: 8147.63,
                    collector_id: "10FEO!20".to_string(),
                    owned: 2,
                    token_ids: vec!["1234567890".to_string(), "1234567890".to_string()],
                    wallet_address: "0x1234567890abcdef".to_string(),
                    last_activity: "2023-10-01".to_string(),
                    verified: true,
                })
                .collect::<Vec<_>>()
        });

    //     let asset = use_signal(|| {
    //         (0..8).map(|id| Assets{
    //     id : id.to_string(),
    //     title: "Asset Title".to_string(),
    //     artist_name: "Artist Name".to_string(),
    //     attributes: vec!["Pixel".to_string(), "Animation".to_string()],
    //     way_to_sell: "Offer".to_string(),
    //     owner: "247".to_string(),
    //     current_price: 2.370,
    //     current_price_usd: 8147.63,
    //     average_price: 2.370,
    //     average_price_usd: 8147.63,
    //     price_change_24h: 12.0,
    //     price_change_7d: -8.0,
    //     volume: 2.370,
    //     volume_usd: 8147.63,
    //     royalty: 2.370,
    //     royalty_usd: 8147.63,
    //     status: "Active".to_string(),
    //     verified: true,
    //     art_image: "https://res.cloudinary.com/dgesrup3u/image/upload/v1744880242/Screenshot_2025-04-17_at_9.56.47_AM_ll2cwy.png".to_string(),
    //     medium: "Digital".to_string(),
    //     rarity: "Rare".to_string(),

    // }).collect::<Vec<_>>()
    //     });

        let activity = use_signal(|| {
            (0..6)
                .map(|id| Activity {
                    id: id.to_string(),
                    from: "20114FWO".to_string(),
                    to: "20114FWO".to_string(),
                    time: "30 mins ago".to_string(),
                    title: "Art Title".to_string(),
                })
                .collect::<Vec<_>>()
        });

        let ctrl = Self {
            lang,
            collector,
            popup,
            // asset,
            activity,
        };
        use_context_provider(|| ctrl);
        Ok(ctrl)
    }
    pub fn create_collector(&self) {
        spawn_local(async move {
            let endpoint = crate::config::get().api_url;
            let client = CollectorModel::get_client(endpoint);
            let res = client
                .act(common::tables::prelude::CollectorAction::Create(
                    CollectorCreateRequest {
                        title: "".to_string(),
                        description: "".to_string(),
                        external_link: None,
                        banner_url: "".to_string(),
                        logo_url: "".to_string(),
                    },
                ))
                .await;
            match res {
                Ok(_) => {
                    tracing::debug!("Collector created successfully");
                }
                Err(err) => {
                    tracing::error!("Error creating collector: {:?}", err);
                }
            }
        });
    }

    pub fn remove_collector(&self, collector_id: i64) {
        spawn_local(async move {
            let endpoint = crate::config::get().api_url;
            let client = CollectorModel::get_client(endpoint);
            let res = client
                .act_by_id(
                    collector_id,
                    CollectorByIdAction::Delete(CollectorDeleteRequest {}),
                )
                .await;
            match res {
                Ok(_) => {
                    tracing::debug!("Collector removed successfully");
                }
                Err(err) => {
                    tracing::error!("Error removing collector: {:?}", err);
                }
            }
        });
    }
}
