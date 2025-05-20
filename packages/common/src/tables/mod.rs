pub mod agit_admins;
pub mod agits;
pub mod artists;
pub mod artworks;
pub mod categories;
pub mod collections;
pub mod collectors;
pub mod topics;
pub mod users;

pub mod user_terms;

pub mod prelude {
    pub use super::agit_admins::*;
    pub use super::agits::*;
    pub use super::artists::*;
    pub use super::artworks::*;
    pub use super::categories::*;
    pub use super::collections::*;
    pub use super::collectors::*;
    pub use super::topics::*;
    pub use super::users::*;
}
