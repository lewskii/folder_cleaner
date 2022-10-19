use std::path::Path;


#[non_exhaustive]
pub enum FilePattern {
    Any,
    Extension(String)
}

impl FilePattern {
    pub fn matches(&self, path: &Path) -> bool {
        match self {
            Self::Any => true,
            Self::Extension(ext) => has_extension(path, ext)
        }
    }
}

fn has_extension(path: &Path, ext: &String) -> bool {
    path.extension().unwrap_or_default() == ext.as_str()
}
