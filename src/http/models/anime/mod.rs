#[allow(clippy::module_inception)]
mod anime;
mod anime_related;
mod anime_similar;
mod character;
mod creator;
mod episode;
mod recommendation;
mod resource;
mod tag;

pub use self::{
    anime::*, anime_related::*, anime_similar::*, character::*, creator::*, episode::*,
    recommendation::*, resource::*, tag::*,
};
