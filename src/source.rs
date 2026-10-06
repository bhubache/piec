#[derive(Debug)]
pub struct SourceFile {
    pub file_path: String,
    pub source: String,
}

impl SourceFile {
    pub fn new(file_path: String, source: String) -> Self {
        Self { file_path, source }
    }
}
