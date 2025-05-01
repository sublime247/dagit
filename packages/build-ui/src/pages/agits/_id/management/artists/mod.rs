mod controllers;
mod i18n;
mod page;
mod _id;
mod new;
mod models;

pub use page::ArtistPage;
pub use _id::{ArtistDetailPage, EditArtistPage, ConfirmRemoveArtistModal, RemoveArtistModal, SuccessModal};
pub use new::NewArtistPage;