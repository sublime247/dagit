#![allow(unused)]
use std::io::Read;

use bdk::prelude::{dioxus_popup::PopupService, *};

use super::models::{Collector, Asset};
// use super::models::*;


#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller{
    lang: Language,
    collector: Signal<Vec<Collector>>,
    asset: Signal<Vec<Asset>>,
    popup: PopupService,

 
}
impl Controller{
  pub fn new(lang: Language, agit_id: ReadOnlySignal<i64>)-> Result<Self, RenderError>{
    let mut popup :PopupService = use_context();
    let collector = use_signal(||{
        (1..15)
        .map(|id| Collector{
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
   
 let asset = use_signal(||{
    (0..8).map(|id| Asset{
        id : id.to_string(),
        title: "Asset Title".to_string(),
        artist_name: "Artist Name".to_string(),
        attributes: vec!["Pixel".to_string(), "Animation".to_string()],
        way_to_sell: "Offer".to_string(),
        owner: "Num".to_string(),
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

    }).collect::<Vec<_>>()
 });

    let ctrl = Self{
        lang,
        collector,
        popup,
        asset,
    };
    use_context_provider(||ctrl);
    Ok(ctrl)
  }
}