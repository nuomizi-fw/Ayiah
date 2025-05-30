pub mod provider;

use provider::MetadataProvider;

pub enum OrganizeMethod {
    SoftLink,
    HardLink,
    Copy,
    Cut,
}

pub struct AyiahScraper {
    pub organize_method: OrganizeMethod,
    pub source_path: String,
    pub target_path: String,
    pub metadata_provider: Option<Box<dyn MetadataProvider>>,
}

impl AyiahScraper {
    pub fn new(organize_method: OrganizeMethod, source_path: String, target_path: String) -> Self {
        Self {
            organize_method,
            source_path,
            target_path,
            metadata_provider: None,
        }
    }
}
