mod artists;
mod artworks;
mod collections;
mod collectors;
mod model;
mod components;

pub use artists::{ArtistDetailPage, ArtistPage, EditArtistPage, NewArtistPage};
pub use artworks::ArtworkPage;
pub use collections::{CollectionDetailPage, CollectionPage};
pub use collectors::{CollectorDetailPage, CollectorsPage};
pub use model::{Activity, Assets};

