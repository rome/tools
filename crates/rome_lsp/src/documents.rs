use std::{collections::HashMap, sync::Arc};

use rome_analyze::{FileId, TextAction};
use tracing::error;

#[derive(Default)]
pub(crate) struct DocumentStore {
	documents: HashMap<FileId, Arc<Document>>,
}

impl DocumentStore {
	pub fn get(&self, id: &FileId) -> Option<Arc<Document>> {
		self.documents.get(id).cloned()
	}

	pub fn set(&mut self, id: FileId, text: impl Into<Arc<String>>, version: i32) {
		let doc = Arc::new(Document::new(text.into(), version));
		self.documents.insert(id, doc);
	}

	pub fn update_text(&mut self, id: FileId, text: impl Into<Arc<String>>, version: i32) {
		self.documents.entry(id).and_modify(|d| {
			if d.version > version {
				error!(
					"File Id {:?} has version {:?} but attempted to update with version {:?}",
					id, d.version, version
				);
			} else {
				*d = Arc::new(Document::new(text.into(), version));
			}
		});
	}

	pub fn update_actions(
		&mut self,
		id: FileId,
		actions: impl Into<Arc<Vec<TextAction>>>,
		version: i32,
	) {
		self.documents.entry(id).and_modify(|d| {
			if d.version > version {
				error!(
					"File Id {:?} has version {:?} but attempted to update with version {:?}",
					id, d.version, version
				);
			} else {
				*d = Arc::new(Document::with_actions(
					d.text.clone(),
					version,
					actions.into(),
				));
			}
		});
	}
}

#[derive(Default)]
pub(crate) struct Document {
	pub(crate) text: Arc<String>,
	pub(crate) version: i32,
	pub(crate) code_actions: Arc<Vec<TextAction>>,
}

impl Document {
	fn new(text: impl Into<Arc<String>>, version: i32) -> Self {
		let text = text.into();
		Self {
			text,
			version,
			code_actions: Arc::new(Vec::new()),
		}
	}

	fn with_actions(
		text: impl Into<Arc<String>>,
		version: i32,
		actions: impl Into<Arc<Vec<TextAction>>>,
	) -> Self {
		let text = text.into();
		Self {
			text,
			version,
			code_actions: actions.into(),
		}
	}
}
