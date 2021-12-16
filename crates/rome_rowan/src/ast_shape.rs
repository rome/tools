use crate::api::RawLanguage;
use crate::Language;

pub trait AstTreeShape: Language {
	fn validate_slot(parent: Self::Kind, index: usize, value: Option<Self::Kind>) -> bool;

	fn validate_end(parent: Self::Kind, length: usize) -> bool;
}

impl AstTreeShape for RawLanguage {
	fn validate_slot(_: Self::Kind, _: usize, _: Option<Self::Kind>) -> bool {
		true
	}

	fn validate_end(_: Self::Kind, _: usize) -> bool {
		true
	}
}
