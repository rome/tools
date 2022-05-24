//! Copied from Rust's unstable iter.intersperse().

use std::{fmt::Debug, iter::Peekable};

use crate::prelude::*;

/// An iterator adapter that places a separator between all elements.
#[derive(Debug, Clone)]
pub struct Intersperse<I: Iterator>
where
    I::Item: Clone,
{
    separator: I::Item,
    iter: Peekable<I>,
    needs_sep: bool,
}

impl<I: Iterator> Intersperse<I>
where
    I::Item: Clone,
{
    pub fn new(iter: I, separator: I::Item) -> Self {
        Self {
            iter: iter.peekable(),
            separator,
            needs_sep: false,
        }
    }
}

impl<I> Iterator for Intersperse<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        if self.needs_sep && self.iter.peek().is_some() {
            self.needs_sep = false;
            Some(self.separator.clone())
        } else {
            self.needs_sep = true;
            self.iter.next()
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lo, hi) = self.iter.size_hint();
        let next_is_elem = !self.needs_sep;
        let lo = lo.saturating_sub(next_is_elem as usize).saturating_add(lo);
        let hi = match hi {
            Some(hi) => hi.saturating_sub(next_is_elem as usize).checked_add(hi),
            None => None,
        };
        (lo, hi)
    }

    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        let mut accum = init;

        // Use `peek()` first to avoid calling `next()` on an empty iterator.
        if !self.needs_sep || self.iter.peek().is_some() {
            if let Some(x) = self.iter.next() {
                accum = f(accum, x);
            }
        }

        let element = &self.separator;

        self.iter.fold(accum, |mut accum, x| {
            accum = f(accum, element.clone());
            accum = f(accum, x);
            accum
        })
    }
}

/// An iterator adapter that places a separator between all elements.
/// The separator element is generated by a factory function called
/// by the iterator with the previous and next SyntaxNode as arguments
#[derive(Debug, Clone)]
pub struct IntersperseFn<I: Iterator, F, N>
where
    I: Iterator<Item = (N, FormatElement)>,
{
    separator_factory: F,
    iter: Peekable<I>,
    prev_item: Option<N>,
}

impl<I, F, N> IntersperseFn<I, F, N>
where
    I: Iterator<Item = (N, FormatElement)>,
    F: FnMut(&N, &N, &FormatElement) -> FormatElement,
{
    pub fn new(iter: I, separator_factory: F) -> Self {
        Self {
            iter: iter.peekable(),
            separator_factory,
            prev_item: None,
        }
    }
}

impl<I, F, N> Iterator for IntersperseFn<I, F, N>
where
    I: Iterator<Item = (N, FormatElement)>,
    F: FnMut(&N, &N, &FormatElement) -> FormatElement,
{
    type Item = FormatElement;

    #[inline]
    fn next(&mut self) -> Option<FormatElement> {
        if let Some(prev_node) = self.prev_item.take() {
            if let Some((next_node, next_elem)) = self.iter.peek() {
                return Some((self.separator_factory)(&prev_node, next_node, next_elem));
            }
        }

        match self.iter.next() {
            Some((node, elem)) => {
                self.prev_item = Some(node);
                Some(elem)
            }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lo, hi) = self.iter.size_hint();
        let next_is_elem = self.prev_item.is_none();
        let lo = lo.saturating_sub(next_is_elem as usize).saturating_add(lo);
        let hi = match hi {
            Some(hi) => hi.saturating_sub(next_is_elem as usize).checked_add(hi),
            None => None,
        };
        (lo, hi)
    }
}

impl<I, F, N> ExactSizeIterator for IntersperseFn<I, F, N>
where
    I: Iterator<Item = (N, FormatElement)>,
    F: FnMut(&N, &N, &FormatElement) -> FormatElement,
{
}
