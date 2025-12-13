pub mod anonymizer;
pub mod detector;
pub mod entity_linker;
pub mod presidio;
pub mod types;

pub use anonymizer::Anonymizer;
#[allow(unused_imports)]
pub use detector::PIIDetector;
#[allow(unused_imports)]
pub use entity_linker::EntityLinker;
#[allow(unused_imports)]
pub use presidio::{PresidioManager, PresidioStatus};
pub use types::{AnonymizationResult, AnonymizationSettings, Entity, EntityType};
