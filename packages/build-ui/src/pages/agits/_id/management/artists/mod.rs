mod _id;
mod controllers;
mod i18n;
mod models;
mod new;
mod page;

pub use _id::{
    ArtistDetailPage, ConfirmRemoveArtistModal, EditArtistPage, RemoveArtistModal, SuccessModal,
};
pub use new::NewArtistPage;
pub use page::ArtistPage;
