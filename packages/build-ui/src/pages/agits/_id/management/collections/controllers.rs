#![allow(unused)]
use crate::pages::agits::_id::management::collections::components::{
    CollectionNameModal, NewCollectionModal, SuccessModal, TransferConfirmationModal,
};

use super::models::*;
use bdk::prelude::{dioxus_popup::PopupService, *};

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    agit_id: ReadOnlySignal<i64>,
    collections: Signal<Vec<Collection>>,
    artworks: Signal<Vec<Artwork>>,

    selected_artworks: Signal<Vec<usize>>,

    popup: PopupService,
}
impl Controller {
    pub fn new(lang: Language, agit_id: ReadOnlySignal<i64>) -> Result<Self, RenderError> {
        let mut popup: PopupService = use_context();
        let collections = use_signal(|| {
            (1..15)
                .map(|id| Collection {
                    id,
                    name: "(Collection Name)".to_string(),
                    verified: true,
                    floor_price_eth: 2.370,
                    floor_price_usd: 8147.63,
                    floor_change_eth: 2.370,
                    floor_change_usd: 8147.63,
                    volume_change_24h: 12.0,
                    volume_change_7d: -8.0,
                    volume_eth: 2.370,
                    volume_usd: 8147.63,
                    owners: "Num".to_string(),
                    stock: "Num".to_string(),
                    status: "Active".to_string(),
                })
                .collect::<Vec<_>>()
        });

        let artworks = use_signal(|| {
            (0..4)
                .map(|id| Artwork {
                    id,
                    title: "(Art Title)".to_string(),
                    artist_name: "Artist Name".to_string(),
                    verified: true,
                    collection: Some("Happy".to_string()),
                    attributes: vec!["Paid".to_string(), "Verified".to_string()],
                    ways_to_sell: "Bid".to_string(),
                    volume_eth: 2.370,
                    volume_usd: 8147.63,
                    status: "Active".to_string(),
                })
                .collect::<Vec<_>>()
        });
        let mut selected_artworks = use_signal(|| Vec::<usize>::new());

        let ctrl = Self {
            lang,
            agit_id,
            popup,
            artworks,
            collections,
            selected_artworks,
        };

        use_context_provider(|| ctrl);
        Ok(ctrl)
    }
}

impl Controller {
    pub fn create_collection(&mut self, collection_name: String) {
        let mut popup = self.popup.clone();
        popup
            .open(rsx! {
                SuccessModal {
                    show: true,
                    collection_name,
                    on_confirm: move |_| { popup.close() },
                }
            })
            .with_id("success-modal");
    }

    pub fn open_new_collection_popup(&mut self) {
        let mut popup = self.popup.clone();
        let mut selected_artworks = self.selected_artworks.clone();
        let artworks_data = self.artworks.read().clone();
        popup
            .open(rsx! {
                NewCollectionModal {
                    show: true,
                    on_close: move |_| {
                        popup.close();
                    },
                    artworks: artworks_data,
                    on_select_artworks: move |selected:Vec<usize>| {
                        let selected_count = selected.len();
                        selected_artworks.set(selected.clone());
                        let mut inner_popup = popup.clone();




                    },
                }
            })
            .with_id("new-collection-modal");
    }
}
