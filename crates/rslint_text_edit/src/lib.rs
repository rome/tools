//! Representation of a text edits.
//!
//! This is taken from [rust-analyzer's text_edit crate](https://rust-analyzer.github.io/rust-analyzer/text_edit/index.html)
pub use rslint_rowan::{TextRange, TextSize};

/// `InsertDelete` -- a single "atomic" change to text
///
/// Must not overlap with other `InDel`s
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Indel {
	pub insert: String,
	/// Refers to offsets in the original text
	pub delete: TextRange,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TextEdit {
	indels: Vec<Indel>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct TextEditBuilder {
	indels: Vec<Indel>,
}

impl Indel {
	pub fn insert(offset: TextSize, text: String) -> Indel {
		Indel::replace(TextRange::empty(offset), text)
	}
	pub fn delete(range: TextRange) -> Indel {
		Indel::replace(range, String::new())
	}
	pub fn replace(range: TextRange, replace_with: String) -> Indel {
		Indel {
			delete: range,
			insert: replace_with,
		}
	}

	pub fn apply(&self, text: &mut String) {
		let start: usize = self.delete.start().into();
		let end: usize = self.delete.end().into();
		text.replace_range(start..end, &self.insert);
	}
}

pub fn apply_indels(indels: &[Indel], text: &mut String) {
	match indels.len() {
		0 => return,
		1 => {
			indels[0].apply(text);
			return;
		}
		_ => (),
	}

	let mut total_len = TextSize::of(&*text);
	for indel in indels.iter() {
		total_len += TextSize::of(&indel.insert);
		total_len -= indel.delete.end() - indel.delete.start();
	}
	let mut buf = String::with_capacity(total_len.into());
	let mut prev = 0;
	for indel in indels.iter() {
		let start: usize = indel.delete.start().into();
		let end: usize = indel.delete.end().into();
		if start > prev {
			buf.push_str(&text[prev..start]);
		}
		buf.push_str(&indel.insert);
		prev = end;
	}
	buf.push_str(&text[prev..text.len()]);
	assert_eq!(TextSize::of(&buf), total_len);

	// FIXME: figure out a way to mutate the text in-place or reuse the
	// memory in some other way
	*text = buf
}

impl TextEdit {
	pub fn builder() -> TextEditBuilder {
		TextEditBuilder::default()
	}

	pub fn insert(offset: TextSize, text: String) -> TextEdit {
		let mut builder = TextEdit::builder();
		builder.insert(offset, text);
		builder.finish()
	}

	pub fn delete(range: TextRange) -> TextEdit {
		let mut builder = TextEdit::builder();
		builder.delete(range);
		builder.finish()
	}

	pub fn replace(range: TextRange, replace_with: String) -> TextEdit {
		let mut builder = TextEdit::builder();
		builder.replace(range, replace_with);
		builder.finish()
	}

	pub fn len(&self) -> usize {
		self.indels.len()
	}

	pub fn is_empty(&self) -> bool {
		self.indels.is_empty()
	}

	pub fn iter(&self) -> std::slice::Iter<'_, Indel> {
		self.into_iter()
	}

	pub fn apply(&self, text: &mut String) {
		apply_indels(&self.indels, text)
	}

	pub fn union(&mut self, other: TextEdit) -> Result<(), TextEdit> {
		// FIXME: can be done without allocating intermediate vector
		let mut all = self.iter().chain(other.iter()).collect::<Vec<_>>();
		if !check_disjoint(&mut all) {
			return Err(other);
		}
		self.indels.extend(other.indels);
		assert_disjoint(&mut self.indels);
		Ok(())
	}

	pub fn apply_to_offset(&self, offset: TextSize) -> Option<TextSize> {
		let mut res = offset;
		for indel in self.indels.iter() {
			if indel.delete.start() >= offset {
				break;
			}
			if offset < indel.delete.end() {
				return None;
			}
			res += TextSize::of(&indel.insert);
			res -= indel.delete.len();
		}
		Some(res)
	}
}

impl IntoIterator for TextEdit {
	type Item = Indel;
	type IntoIter = std::vec::IntoIter<Indel>;

	fn into_iter(self) -> Self::IntoIter {
		self.indels.into_iter()
	}
}

impl<'a> IntoIterator for &'a TextEdit {
	type Item = &'a Indel;
	type IntoIter = std::slice::Iter<'a, Indel>;

	fn into_iter(self) -> Self::IntoIter {
		self.indels.iter()
	}
}

impl TextEditBuilder {
	pub fn replace(&mut self, range: TextRange, replace_with: String) {
		self.indel(Indel::replace(range, replace_with))
	}
	pub fn delete(&mut self, range: TextRange) {
		self.indel(Indel::delete(range))
	}
	pub fn insert(&mut self, offset: TextSize, text: String) {
		self.indel(Indel::insert(offset, text))
	}
	pub fn finish(self) -> TextEdit {
		let mut indels = self.indels;
		assert_disjoint(&mut indels);
		TextEdit { indels }
	}
	pub fn invalidates_offset(&self, offset: TextSize) -> bool {
		self.indels
			.iter()
			.any(|indel| indel.delete.contains_inclusive(offset))
	}
	fn indel(&mut self, indel: Indel) {
		self.indels.push(indel);
		if self.indels.len() <= 16 {
			assert_disjoint(&mut self.indels);
		}
	}
}

fn assert_disjoint(indels: &mut [impl std::borrow::Borrow<Indel>]) {
	assert!(check_disjoint(indels));
}
fn check_disjoint(indels: &mut [impl std::borrow::Borrow<Indel>]) -> bool {
	indels.sort_by_key(|indel| (indel.borrow().delete.start(), indel.borrow().delete.end()));
	indels
		.iter()
		.zip(indels.iter().skip(1))
		.all(|(l, r)| l.borrow().delete.end() <= r.borrow().delete.start())
}
