mod page;

mod analytics;
mod design;
mod extension_tool;
mod hub;
mod management;
mod orders;

pub use analytics::{ReportPage, TrafficPage};
pub use page::HomePage;

pub use design::DesignPage;
pub use extension_tool::ExtensionToolPage;
pub use hub::{DaoPage, FaqPage, OraclePage};
pub use management::{ArtistPage, ArtworkPage, CollectionPage, CollectorsPage, CollectorDetailPage, CollectionDetailPage};
pub use orders::{SalesRequestPage, ShippingLabelPage};
