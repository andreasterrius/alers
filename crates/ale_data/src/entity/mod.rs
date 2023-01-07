pub mod entry;

pub use traitcast_core::Registry;
pub use traitcast_core::TraitcastFrom as Component;
use std::any::Any;

pub type Entity = Box<dyn Any>;
