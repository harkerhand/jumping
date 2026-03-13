use std::path::PathBuf;

/// An entry in the catalog
#[derive(Clone)]
pub struct Entry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub need_preview: bool,
}
