#![allow(unused)]
use crate::pages::agits::_id::management::collections::components::{
    CollectionNameModal, NewCollectionModal, SuccessModal, TransferConfirmationModal,
};

use super::models::*;
use bdk::prelude::{dioxus_popup::PopupService, *};

// Define modal states to track which modal is currently shown
#[derive(Debug, Clone, PartialEq)]
enum ModalState {
    None,
    NewCollection,
    TransferConfirmation,
    CollectionName,
    Success,
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    agit_id: ReadOnlySignal<i64>,
    collections: Signal<Vec<Collection>>,
    artworks: Signal<Vec<Artwork>>,

    selected_artworks: Signal<Vec<usize>>,
    modal_state: Signal<ModalState>,
    collection_name: Signal<String>,

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
        
        let selected_artworks = use_signal(|| Vec::<usize>::new());
        let modal_state = use_signal(|| ModalState::None);
        let collection_name = use_signal(|| String::new());

        let ctrl = Self {
            lang,
            agit_id,
            popup,
            artworks,
            collections,
            selected_artworks,
            modal_state,
            collection_name,
        };

        use_context_provider(|| ctrl);
        Ok(ctrl)
    }

    
    // Method to update the modal state and open the appropriate modal using popup_service

    fn update_modal_state(&mut self, state: ModalState) {
        self.modal_state.set(state.clone());
        self.popup.close();
        
        
        // Open the appropriate modal based on state
        match state {
            ModalState::None => {},
            ModalState::NewCollection => {
                let artworks_data = self.artworks.read().clone();
                let mut selected_artworks = self.selected_artworks.clone();
                let mut this = self.clone();
                
                self.popup.open(rsx!(
                    NewCollectionModal {
                        show: true,
                        on_close: move |_| this.update_modal_state(ModalState::None),
                        artworks: artworks_data,
                        on_select_artworks: move |selected: Vec<usize>| {
                            selected_artworks.set(selected.clone());
                            this.update_modal_state(ModalState::TransferConfirmation);
                        },
                    }
                )).with_id("new-collection-modal");
            },

            ModalState::TransferConfirmation => {
                let selected_count = self.selected_artworks.read().len();
                let mut this = self.clone();
                
                self.popup.open(rsx!(
                    TransferConfirmationModal {
                        show: true,
                        selected_count,
                        on_back: move |_| this.update_modal_state(ModalState::NewCollection),
                        on_continue: move |_| this.update_modal_state(ModalState::CollectionName),
                    }
                )).with_id("transfer-confirmation-modal");
            },


            ModalState::CollectionName => {
                let mut this = self.clone();
                let mut collection_name = self.collection_name.clone();
                
                self.popup.open(rsx!(
                    CollectionNameModal {
                        show: true,
                        on_back: move |_| this.update_modal_state(ModalState::TransferConfirmation),
                        on_add: move |name: String| {
                            collection_name.set(name.clone());
                            tracing::debug!("Collection Name: {}", name);
                        //    todo:: function to simulate the api to addcollection will be called here before the success modal
                            this.update_modal_state(ModalState::Success);
                        },
                    }
                )).with_id("collection-name-modal");
            },


            ModalState::Success => {
                let collection_name = self.collection_name.read().clone();
                let mut this = self.clone();
                
                self.popup.open(rsx!(
                    SuccessModal {
                        show: true,
                        collection_name,
                        on_confirm: move |_| {
                            // Reset state and close modal
                           
                            this.update_modal_state(ModalState::None);
                        },
                    }
                )).with_id("success-modal");
            },
        }
    }


    // Public method to start the modal flow
    pub fn open_new_collection_popup(&mut self) {
            self.update_modal_state(ModalState::NewCollection);
        }
}