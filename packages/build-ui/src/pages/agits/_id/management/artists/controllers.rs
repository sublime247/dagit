#![allow(unused)]
use super::models::*;

use common::tables::{artists::Artist as ArtistModel, prelude::{ArtistCreateRequest, ArtistQuery}};

use bdk::prelude::*;

use crate::{config::Config, pages::agits::_id::management::Assets, routes::Route};


#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller{
    lang:Language,
    agit_id:ReadOnlySignal<i64>,
    artist: Signal<Vec<Artist>>,
    artist_asset: Signal<Vec<Assets>>,

}
impl Controller{
    pub fn new(lang:Language, agit_id:ReadOnlySignal<i64>)->Result<Self, RenderError>{
        let res = use_server_future(move || {
            async move {
                let endpoint = crate::config::get().api_url;
                let client = ArtistModel::get_client(endpoint);
                client.query(ArtistQuery::new(100).with_page(0)).await.unwrap_or_default()
            }
        })?;
        let artist = use_signal(||{
           (1..10).map(|id|Artist {
            id: id.to_string(),
            name: "Artist Name".to_string(),
            mail: "email@email.com".to_string(),
            revenue: 2.370,
            revenue_usd: 8147.63,
            attributes: vec!["Pixel".to_string(), "Animation".to_string(), "Sci-fi".to_string(), "3D".to_string(), "Digital".to_string()],
            status: "true".to_string(),
            social_media: "@social_media".to_string(),
            featured_work: "Artwork_title".to_string(),
            artwork: "Num".to_string(),
        }).collect::<Vec<_>>()
        });

        let artist_asset = use_signal(||{
          (0..8).map(|id| Assets{
              id : id.to_string(),
              title: "Asset Title".to_string(),
              artist_name: "Artist Name".to_string(),
              attributes: vec!["Pixel".to_string(), "Animation".to_string()],
              way_to_sell: "Offer".to_string(),
              owner: "247".to_string(),
              current_price: 2.370,
              current_price_usd: 8147.63,
              average_price: 2.370,
              average_price_usd: 8147.63,
              price_change_24h: 12.0,
              price_change_7d: -8.0,
              volume: 2.370,
              volume_usd: 8147.63,
              royalty: 2.370,
              royalty_usd: 8147.63,
              status: "Active".to_string(),
              verified: true,
              art_image: "https://res.cloudinary.com/dgesrup3u/image/upload/v1744880242/Screenshot_2025-04-17_at_9.56.47_AM_ll2cwy.png".to_string(),
              medium: "Digital".to_string(),
              rarity: "Rare".to_string(),
          }).collect::<Vec<_>>()
       });
      let ctrl = Self{
        lang,
        agit_id,
        artist,
        artist_asset
      };
      use_context_provider(||ctrl);
        Ok(ctrl)
    }

    pub async fn create_artist(&self){
        let endpoint = crate::config::get().api_url;
        let client = ArtistModel::get_client(endpoint);
        // act is without id. Create
        // act_by_id is with id, update or delete.
        let res = client.act(common::tables::prelude::ArtistAction::Create(ArtistCreateRequest {
            title: "New Artist".to_string(),
        })).await;
        match res {
            Ok(_) => {
                // Handle success
                btracing::info!("Artist created successfully");
                
            }
            Err(e) => {
                btracing::error!("Error creating artist: {:?}", e);
            }
        };
    }
    pub fn open_new_artist_form(&self){
        let navigate = use_navigator();
        navigate.push(Route::NewArtistPage{
            lang: self.lang,
            agit_id: self.agit_id.with(|id| *id),
        });
    }
}