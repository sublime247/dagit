#![allow(unused)]
use super::models::*;
use common::tables::{
    artists::Artist as ArtistModel,
    prelude::{ArtistByIdAction, ArtistCreateRequest, ArtistDeleteRequest, ArtistQuery},
};
use wasm_bindgen_futures::spawn_local;

use bdk::prelude::{dioxus_popup::PopupService, *};

use crate::{config::Config, pages::agits::_id::management::{artists::{ConfirmRemoveArtistModal, RemoveArtistModal, SuccessModal}, Assets}, routes::Route};
#[derive(Debug, Clone, PartialEq)]
enum ModalState{
    None,
    ConfirmRemoval,
    ConfirmNameRemoval,
    Success,

}
#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    agit_id: ReadOnlySignal<i64>,
    artist: Signal<Vec<Artist>>,
    artist_input_field: Signal<ArtistInputField>,
    artist_asset: Signal<Vec<Assets>>,
    modal_state:Signal<ModalState>,
    popup:PopupService

}
impl Controller {
    pub fn new(lang: Language, agit_id: ReadOnlySignal<i64>) -> Result<Self, RenderError> {
        let mut popup:PopupService = use_context();
        let res = use_server_future(move || async move {
            let endpoint = crate::config::get().api_url;
            let client = ArtistModel::get_client(endpoint);
            client
                .query(ArtistQuery::new(100).with_page(0))
                .await
                .unwrap_or_default()
        })?;
        tracing::debug!("res: {:?}", res);
        let artist_input_field = use_signal(|| ArtistInputField {
            display_name: String::new(),
            social_media: String::new(),
            medium: String::new(),
            theme: String::new(),
            art_style: String::new(),
            introduction: String::new(),
            biography: String::new(),
        });
        let artist = use_signal(|| {
            (1..10)
                .map(|id| Artist {
                    id: id.to_string(),
                    name: "Artist Name".to_string(),
                    mail: "email@email.com".to_string(),
                    revenue: 2.370,
                    revenue_usd: 8147.63,
                    attributes: vec![
                        "Pixel".to_string(),
                        "Animation".to_string(),
                        "Sci-fi".to_string(),
                        "3D".to_string(),
                        "Digital".to_string(),
                    ],
                    status: "true".to_string(),
                    social_media: "@social_media".to_string(),
                    featured_work: "Artwork_title".to_string(),
                    artwork: "Num".to_string(),
                })
                .collect::<Vec<_>>()
        });

        let artist_asset = use_signal(|| {
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
        let modal_state = use_signal(|| ModalState::None);

        let ctrl = Self {
            lang,
            agit_id,
            artist,
            artist_input_field,
            artist_asset,
            modal_state,
            popup 
        };
        use_context_provider(|| ctrl);
        Ok(ctrl)
    }

    pub fn create_artist(&self) {
        let artist_inputs = self.artist_input_field.with(|field| field.clone());
        // act_by_id is with id, update or delete.
        // act is without id. Create
        spawn_local(async move {
            let endpoint = crate::config::get().api_url;
            let client = ArtistModel::get_client(endpoint);
            let res = client
                .act(common::tables::prelude::ArtistAction::Create(
                    ArtistCreateRequest {
                        title: artist_inputs.display_name,
                        mail: artist_inputs.social_media.clone(),
                        social_media: artist_inputs.medium.clone(),
                        intro: artist_inputs.theme,
                        biography: artist_inputs.art_style,
                    },
                ))
                .await;
            tracing::debug!("mail: {:?}", artist_inputs.social_media);
            tracing::debug!("social_media: {:?}", artist_inputs.medium);    
            match res {
                Ok(_) => {
                    btracing::info!("Artist created successfully");
                }
                Err(e) => {
                    btracing::error!("Error creating artist: {:?}", e);
                }
            };
        });
    }
    pub fn remove_artist(&self, artist_id: i64) {
        spawn_local(async move {
            let endpoint = crate::config::get().api_url;
            let client = ArtistModel::get_client(endpoint);
            let res = client
                .act_by_id(
                    artist_id,
                    common::tables::prelude::ArtistByIdAction::Delete(ArtistDeleteRequest {}),
                )
                .await;
            match res {
                Ok(_) => {
                    btracing::info!("Artist removed successfully");
                }
                Err(e) => {
                    btracing::error!("Error removing artist: {:?}", e);
                }
            };
        });
    }



pub fn update_artist_field(&mut self, field: String, value: String) {
    match field.as_str(){
        "display_name" => self.artist_input_field.set(ArtistInputField{
            display_name: value,
            ..self.artist_input_field.with(|field| field.clone())
        }),
        "social_media" => self.artist_input_field.set(ArtistInputField{
            social_media: value,
            ..self.artist_input_field.with(|field| field.clone())
        }),
        "medium" => self.artist_input_field.set(ArtistInputField{
            medium: value,
            ..self.artist_input_field.with(|field| field.clone())
        }),
        "theme" => self.artist_input_field.set(ArtistInputField{
           theme: value,
            ..self.artist_input_field.with(|field| field.clone())
        }),
        "introduction" => self.artist_input_field.set(ArtistInputField{
           introduction: value,
            ..self.artist_input_field.with(|field| field.clone())
        }),
  
        "biography" => self.artist_input_field.set(ArtistInputField{
           biography: value,
            ..self.artist_input_field.with(|field| field.clone())
        }),
        "art_style" => self.artist_input_field.set(ArtistInputField{
           art_style: value,
            ..self.artist_input_field.with(|field| field.clone())
        }), 
        _ => {}
    }
}

    pub fn open_new_artist_form(&self) {
        let navigate = use_navigator();
        navigate.push(Route::NewArtistPage {
            lang: self.lang,
            agit_id: self.agit_id.with(|id  | *id),
        });
    }


fn update_modal_state(&mut self, state:ModalState){
    self.modal_state.set(state.clone());
    self.popup.close();

    match state {
        ModalState::None=>{},
        ModalState::ConfirmRemoval=>{
            let mut this = self.clone();
            let lang = self.lang;
            self.popup.open(
                rsx!(
                    ConfirmRemoveArtistModal{show:true,
                    on_back: move |_| this.update_modal_state(ModalState::None),
                    on_remove: move |_| {
                        // self.remove_artist(self.agit_id.with(|id| *id));
                        this.update_modal_state(ModalState::ConfirmNameRemoval);
                    },
                    lang: lang
                }
                )
            ).with_id("remove-artist-modal");
        },
        ModalState::ConfirmNameRemoval=>{
            let mut this = self.clone();
            let lang = self.lang;
            self.popup.open(
                rsx!(
                    RemoveArtistModal{
                    show:true,
                    on_back: move |_| this.update_modal_state(ModalState::None),
                    on_remove: move |_| {
                        this.remove_artist(this.agit_id.with(|id| *id));
                        this.update_modal_state(ModalState::Success);
                    },
                    lang: lang
                }
                )
            ).with_id("remove-artistName-modal");

        },
        ModalState::Success=>{
            let mut this = self.clone();
            let lang = self.lang;
            self.popup.open(
                rsx!(
                    SuccessModal{
                    show:true,
                    on_back: move |_| this.update_modal_state(ModalState::None),
                    on_confirm: move |_| this.update_modal_state(ModalState::None),
                    lang: lang
                }
                )
            ).with_id("remove-artist-modal-success");
        }
    }

}




pub fn remove_artist_popup(&mut self){
    self.update_modal_state(ModalState::ConfirmRemoval);
}
}
