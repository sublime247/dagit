#![allow(unused)]
use bdk::prelude::{dioxus_popup::PopupService, *};
use common::tables::{
    artworks::Artwork as ArtworkModel,
    prelude::{ArtworkByIdAction, ArtworkCreateRequest, ArtworkDeleteRequest, ArtworkQuery},
};

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    agit_id: ReadOnlySignal<i64>,
    artworks: Signal<Vec<ArtworkModel>>,
    popup: PopupService,
}

impl Controller {
    pub fn new(lang: Language, agit_id: ReadOnlySignal<i64>) -> Result<Self, RenderError> {
        let mut popup: PopupService = use_context();
        let res = use_server_future(move || async move {
            let endpoint = crate::config::get().api_url;
            let client = ArtworkModel::get_client(endpoint);
            client
                .query(ArtworkQuery::new(100).with_page(0))
                .await
                .unwrap_or_default()
        })?;

        tracing::debug!("res: {:?}", res);

        let artworks = use_signal(|| {
            (0..4)
                .map(|id| ArtworkModel {
                    id,
                    title: "(Art Title)".to_string(),
                    name: "Artist Name".to_string(),
                    verified: true,
                    collection_type: Some("Happy".to_string()),
                    attributes_type: vec!["Paid".to_string(), "Verified".to_string()],
                    ways_to_sell: "Bid".to_string(),
                    volume_eth: 2.370,
                    volume_usd: 8147.63,
                    current_price: 2.370,
                    average_price: 2.370,
                    royalty: 2.370,
                    price_change: 12.0,
                    owners: 145,
                    status: "Active".to_string(),
                    created_at: chrono::Utc::now().timestamp(),
                    updated_at: chrono::Utc::now().timestamp(),
                    external_link: None,
                    description: "Description".to_string(),
                    agit_id: 1,
                    collection_id: Some((1)),
                    artist_id:1,
                    owner_id:1,
                    likes: 0,
                    liked: false,
                    art_image: "https://res.cloudinary.com/dgesrup3u/image/upload/v1744880242/Screenshot_2025-04-17_at_9.56.47_AM_ll2cwy.png".to_string(),
                    last_price:100,
                    medium: "Digital".to_string(),
                    rarity: "Rare".to_string(),
                    activity_id: "1".to_string(),
                    activity_from: "20114FWO".to_string(),
                    activity_to: "20114FWO".to_string(),
                    activity_time: "30 mins ago".to_string(),
                    activity_title: "Art Title".to_string(),
                })
                .collect::<Vec<_>>()
        });
        let ctrl = Self{
            lang,
            agit_id,
            popup,
            artworks
        };
        use_context_provider(||ctrl);
        Ok(ctrl)
    }
}
