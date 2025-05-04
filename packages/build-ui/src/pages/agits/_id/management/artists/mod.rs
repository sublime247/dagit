mod _id;
mod controllers;
mod i18n;
mod models;
mod new;
mod page;
mod components;

pub use _id::{
    ArtistDetailPage, EditArtistPage,
};
pub use components::{
    ArtistTable, InputField
};
pub use new::NewArtistPage;
pub use page::ArtistPage;
