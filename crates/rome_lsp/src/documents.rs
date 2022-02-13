use std::sync::Arc;

use rome_analyze::FileId;

#[derive(Clone)]
pub struct Document {
    pub text: Arc<str>,
    pub version: i32,
    pub file_id: FileId,
}

impl Document {
    pub fn new(text: impl Into<Arc<str>>, version: i32, file_id: FileId) -> Self {
        Self {
            text: text.into(),
            version,
            file_id,
        }
    }
}
