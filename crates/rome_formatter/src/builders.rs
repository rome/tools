use crate::format_element::List;
use crate::{empty_element, FormatElement};

#[derive(Default)]
pub struct ConcatBuilder {
    elements: Vec<FormatElement>,
    size_hint: Option<usize>,
}

impl ConcatBuilder {
    #[inline]
    pub fn new() -> Self {
        Self {
            elements: vec![],
            size_hint: None,
        }
    }

    #[inline]
    pub fn entry(&mut self, element: FormatElement) {
        if element.is_empty() {
            return;
        }

        if self.elements.is_empty() && self.size_hint.is_some() {
            // SAFETY: Guaranteed by the `is_some` check above
            let size_hint = self.size_hint.unwrap();

            match element {
                FormatElement::List(list) => {
                    self.elements = list.into_vec();
                    self.elements.reserve(size_hint - 1);
                }
                item => {
                    self.elements.reserve(size_hint);
                    self.elements.push(item);
                }
            }
        } else {
            match element {
                FormatElement::List(list) => self.elements.extend(list.into_vec()),
                item => self.elements.push(item),
            }
        }
    }

    #[inline]
    pub fn size_hint(&mut self, hint: (usize, Option<usize>)) {
        let (lower_bound, upper_bound) = hint;

        if let Some(upper_bound) = upper_bound {
            debug_assert!(lower_bound <= upper_bound, "Expected lower bound {lower_bound} to be less than or equal to upper bound {upper_bound}");
            self.size_hint = Some(upper_bound);
        } else {
            self.size_hint = Some(lower_bound);
        }
    }

    #[inline]
    pub fn finish(mut self) -> FormatElement {
        if self.elements.is_empty() {
            empty_element()
        } else if self.elements.len() == 1 {
            // Safety: Guaranteed to succeed by the length check above
            self.elements.pop().unwrap()
        } else {
            FormatElement::List(List::new(self.elements))
        }
    }
}
