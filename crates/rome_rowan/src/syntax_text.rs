use std::fmt;

use crate::{
	cursor::{SyntaxNode, SyntaxToken},
	TextRange, TextSize,
};

#[derive(Clone)]
pub struct SyntaxText {
	node: SyntaxNode,
	range: TextRange,
}

impl SyntaxText {
	pub(crate) fn new(node: SyntaxNode) -> SyntaxText {
		let range = node.text_range();
		SyntaxText { node, range }
	}

	pub(crate) fn with_range(node: SyntaxNode, range: TextRange) -> SyntaxText {
		SyntaxText { node, range }
	}

	pub fn len(&self) -> TextSize {
		self.range.len()
	}

	pub fn is_empty(&self) -> bool {
		self.range.is_empty()
	}

	pub fn contains_char(&self, c: char) -> bool {
		self.try_for_each_chunk(|chunk| if chunk.contains(c) { Err(()) } else { Ok(()) })
			.is_err()
	}

	pub fn find_char(&self, c: char) -> Option<TextSize> {
		let mut acc: TextSize = 0.into();
		let res = self.try_for_each_chunk(|chunk| {
			if let Some(pos) = chunk.find(c) {
				let pos: TextSize = (pos as u32).into();
				return Err(acc + pos);
			}
			acc += TextSize::of(chunk);
			Ok(())
		});
		found(res)
	}

	pub fn char_at(&self, offset: TextSize) -> Option<char> {
		let mut start: TextSize = 0.into();
		let res = self.try_for_each_chunk(|chunk| {
			let end = start + TextSize::of(chunk);
			if start <= offset && offset < end {
				let off: usize = u32::from(offset - start) as usize;
				return Err(chunk[off..].chars().next().unwrap());
			}
			start = end;
			Ok(())
		});
		found(res)
	}

	pub fn slice<R: private::SyntaxTextRange>(&self, range: R) -> SyntaxText {
		let start = range.start().unwrap_or_default();
		let end = range.end().unwrap_or_else(|| self.len());
		assert!(start <= end);
		let len = end - start;
		let start = self.range.start() + start;
		let end = start + len;
		assert!(
			start <= end,
			"invalid slice, range: {:?}, slice: {:?}",
			self.range,
			(range.start(), range.end()),
		);
		let range = TextRange::new(start, end);
		assert!(
			self.range.contains_range(range),
			"invalid slice, range: {:?}, slice: {:?}",
			self.range,
			range,
		);
		SyntaxText {
			node: self.node.clone(),
			range,
		}
	}

	pub fn try_fold_chunks<T, F, E>(&self, init: T, mut f: F) -> Result<T, E>
	where
		F: FnMut(T, &str) -> Result<T, E>,
	{
		self.tokens_with_ranges()
			.try_fold(init, move |acc, (token, range)| {
				f(acc, &token.text()[range])
			})
	}

	pub fn try_for_each_chunk<F: FnMut(&str) -> Result<(), E>, E>(
		&self,
		mut f: F,
	) -> Result<(), E> {
		self.try_fold_chunks((), move |(), chunk| f(chunk))
	}

	pub fn for_each_chunk<F: FnMut(&str)>(&self, mut f: F) {
		enum Void {}
		match self.try_for_each_chunk(|chunk| {
			f(chunk);
			Ok::<(), Void>(())
		}) {
			Ok(()) => (),
			Err(void) => match void {},
		}
	}

	fn tokens_with_ranges(&self) -> impl Iterator<Item = (SyntaxToken, TextRange)> {
		let text_range = self.range;
		self.node
			.descendants_with_tokens()
			.filter_map(|element| element.into_token())
			.filter_map(move |token| {
				let token_range = token.text_range();
				let range = text_range.intersect(token_range)?;
				Some((token, range - token_range.start()))
			})
	}
}

fn found<T>(res: Result<(), T>) -> Option<T> {
	match res {
		Ok(()) => None,
		Err(it) => Some(it),
	}
}

impl fmt::Debug for SyntaxText {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(&self.to_string(), f)
	}
}

impl fmt::Display for SyntaxText {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.try_for_each_chunk(|chunk| fmt::Display::fmt(chunk, f))
	}
}

impl From<SyntaxText> for String {
	fn from(text: SyntaxText) -> String {
		text.to_string()
	}
}

impl PartialEq<str> for SyntaxText {
	fn eq(&self, mut rhs: &str) -> bool {
		self.try_for_each_chunk(|chunk| {
			if !rhs.starts_with(chunk) {
				return Err(());
			}
			rhs = &rhs[chunk.len()..];
			Ok(())
		})
		.is_ok() && rhs.is_empty()
	}
}

impl PartialEq<SyntaxText> for str {
	fn eq(&self, rhs: &SyntaxText) -> bool {
		rhs == self
	}
}

impl PartialEq<&'_ str> for SyntaxText {
	fn eq(&self, rhs: &&str) -> bool {
		self == *rhs
	}
}

impl PartialEq<SyntaxText> for &'_ str {
	fn eq(&self, rhs: &SyntaxText) -> bool {
		rhs == self
	}
}

impl PartialEq for SyntaxText {
	fn eq(&self, other: &SyntaxText) -> bool {
		if self.range.len() != other.range.len() {
			return false;
		}
		let mut lhs = self.tokens_with_ranges();
		let mut rhs = other.tokens_with_ranges();
		zip_texts(&mut lhs, &mut rhs).is_none()
			&& lhs.all(|it| it.1.is_empty())
			&& rhs.all(|it| it.1.is_empty())
	}
}

fn zip_texts<I: Iterator<Item = (SyntaxToken, TextRange)>>(xs: &mut I, ys: &mut I) -> Option<()> {
	let mut x = xs.next()?;
	let mut y = ys.next()?;
	loop {
		while x.1.is_empty() {
			x = xs.next()?;
		}
		while y.1.is_empty() {
			y = ys.next()?;
		}
		let x_text = &x.0.text()[x.1];
		let y_text = &y.0.text()[y.1];
		if !(x_text.starts_with(y_text) || y_text.starts_with(x_text)) {
			return Some(());
		}
		let advance = std::cmp::min(x.1.len(), y.1.len());
		x.1 = TextRange::new(x.1.start() + advance, x.1.end());
		y.1 = TextRange::new(y.1.start() + advance, y.1.end());
	}
}

impl Eq for SyntaxText {}

mod private {
	use std::ops;

	use crate::{TextRange, TextSize};

	pub trait SyntaxTextRange {
		fn start(&self) -> Option<TextSize>;
		fn end(&self) -> Option<TextSize>;
	}

	impl SyntaxTextRange for TextRange {
		fn start(&self) -> Option<TextSize> {
			Some(TextRange::start(*self))
		}
		fn end(&self) -> Option<TextSize> {
			Some(TextRange::end(*self))
		}
	}

	impl SyntaxTextRange for ops::Range<TextSize> {
		fn start(&self) -> Option<TextSize> {
			Some(self.start)
		}
		fn end(&self) -> Option<TextSize> {
			Some(self.end)
		}
	}

	impl SyntaxTextRange for ops::RangeFrom<TextSize> {
		fn start(&self) -> Option<TextSize> {
			Some(self.start)
		}
		fn end(&self) -> Option<TextSize> {
			None
		}
	}

	impl SyntaxTextRange for ops::RangeTo<TextSize> {
		fn start(&self) -> Option<TextSize> {
			None
		}
		fn end(&self) -> Option<TextSize> {
			Some(self.end)
		}
	}

	impl SyntaxTextRange for ops::RangeFull {
		fn start(&self) -> Option<TextSize> {
			None
		}
		fn end(&self) -> Option<TextSize> {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::api::{RawLanguage, RawLanguageKind};
	use crate::{SyntaxNode, TreeBuilder};

	fn build_tree(chunks: &[&str]) -> SyntaxNode<RawLanguage> {
		let mut builder = TreeBuilder::<'_, RawLanguage>::new();
		builder.start_node(RawLanguageKind(1));
		for &chunk in chunks.iter() {
			builder.token(RawLanguageKind(2), chunk)
		}
		builder.finish_node();
		builder.finish()
	}

	#[test]
	fn test_text_equality() {
		fn do_check(t1: &[&str], t2: &[&str]) {
			let t1 = build_tree(t1).text();
			let t2 = build_tree(t2).text();
			let expected = t1.to_string() == t2.to_string();
			let actual = t1 == t2;
			assert_eq!(
				expected, actual,
				"`{}` (SyntaxText) `{}` (SyntaxText)",
				t1, t2
			);
			let actual = t1 == *t2.to_string();
			assert_eq!(expected, actual, "`{}` (SyntaxText) `{}` (&str)", t1, t2);
		}
		fn check(t1: &[&str], t2: &[&str]) {
			do_check(t1, t2);
			do_check(t2, t1)
		}

		check(&[""], &[""]);
		check(&["a"], &[""]);
		check(&["a"], &["a"]);
		check(&["abc"], &["def"]);
		check(&["hello", "world"], &["hello", "world"]);
		check(&["hellowo", "rld"], &["hell", "oworld"]);
		check(&["hel", "lowo", "rld"], &["helloworld"]);
		check(&["{", "abc", "}"], &["{", "123", "}"]);
		check(&["{", "abc", "}", "{"], &["{", "123", "}"]);
		check(&["{", "abc", "}"], &["{", "123", "}", "{"]);
		check(&["{", "abc", "}ab"], &["{", "abc", "}", "ab"]);
	}
}
