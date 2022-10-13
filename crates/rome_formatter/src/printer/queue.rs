use crate::format_element::tag::TagKind;
use crate::prelude::Tag;
use crate::printer::stack::{Stack, StackedStack};
use crate::printer::{invalid_end_tag, invalid_start_tag};
use crate::{FormatElement, PrintResult};
use std::fmt::Debug;
use std::iter::FusedIterator;
use std::marker::PhantomData;

/// Queue of [FormatElement]s.
pub(super) trait Queue<'a> {
    type Stack: Stack<&'a [FormatElement]>;

    fn stack(&self) -> &Self::Stack;

    fn stack_mut(&mut self) -> &mut Self::Stack;

    /// Pops the element at the end of the queue.
    fn pop(&mut self) -> Option<&'a FormatElement> {
        let stack = self.stack_mut();
        match stack.pop() {
            Some([element]) => Some(element),
            Some([element, rest @ ..]) => {
                stack.push(rest);
                Some(element)
            }
            Some([]) => unreachable!("Guaranteed by extend_back"),
            None => None,
        }
    }

    /// Returns the next element, not traversing into [FormatElement::Interned].
    fn top_with_interned(&self) -> Option<&'a FormatElement> {
        self.stack().top().and_then(|top_slice| top_slice.first())
    }

    /// Returns the next element, recursively resolving the first element of [FormatElement::Interned].
    fn top(&self) -> Option<&'a FormatElement> {
        let mut top = self.top_with_interned();

        while let Some(FormatElement::Interned(interned)) = top {
            top = interned.first()
        }

        top
    }

    /// Queues a single element to process before the other elements in this queue.
    fn push(&mut self, element: &'a FormatElement) {
        self.extend_back(std::slice::from_ref(element))
    }

    /// Queues a slice of elements to process before the other elements in this queue.
    fn extend_back(&mut self, elements: &'a [FormatElement]) {
        match elements {
            [] => {
                // Don't push empty slices
            }
            slice => {
                self.stack_mut().push(slice);
            }
        }
    }

    /// Removes top slice.
    fn pop_slice(&mut self) -> Option<&'a [FormatElement]> {
        self.stack_mut().pop()
    }

    /// Skips all content until it finds the corresponding end tag with the given kind.
    fn skip_content(&mut self, kind: TagKind)
    where
        Self: Sized,
    {
        let iter = self.iter_content(kind);

        for _ in iter {
            // consume whole iterator until end
        }
    }

    /// Iterates over all elements until it finds the matching end tag of the specified kind.
    fn iter_content<'q>(&'q mut self, kind: TagKind) -> QueueContentIterator<'a, 'q, Self>
    where
        Self: Sized,
    {
        QueueContentIterator::new(self, kind)
    }
}

/// Queue with the elements to print.
#[derive(Debug, Default, Clone)]
pub(super) struct PrintQueue<'a> {
    slices: Vec<&'a [FormatElement]>,
}

impl<'a> PrintQueue<'a> {
    pub(super) fn new(slice: &'a [FormatElement]) -> Self {
        let slices = match slice {
            [] => Vec::default(),
            slice => vec![slice],
        };

        Self { slices }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.slices.is_empty()
    }
}

impl<'a> Queue<'a> for PrintQueue<'a> {
    type Stack = Vec<&'a [FormatElement]>;

    fn stack(&self) -> &Self::Stack {
        &self.slices
    }

    fn stack_mut(&mut self) -> &mut Self::Stack {
        &mut self.slices
    }
}

/// Queue for measuring if an element fits on the line.
///
/// The queue is a view on top of the [PrintQueue] because no elements should be removed
/// from the [PrintQueue] while measuring.
#[must_use]
#[derive(Debug)]
pub(super) struct FitsQueue<'a, 'print> {
    stack: StackedStack<'print, &'a [FormatElement]>,
}

impl<'a, 'print> FitsQueue<'a, 'print> {
    pub(super) fn new(
        print_queue: &'print PrintQueue<'a>,
        saved: Vec<&'a [FormatElement]>,
    ) -> Self {
        let stack = StackedStack::with_vec(&print_queue.slices, saved);

        Self { stack }
    }

    pub(super) fn finish(self) -> Vec<&'a [FormatElement]> {
        self.stack.into_vec()
    }
}

impl<'a, 'print> Queue<'a> for FitsQueue<'a, 'print> {
    type Stack = StackedStack<'print, &'a [FormatElement]>;

    fn stack(&self) -> &Self::Stack {
        &self.stack
    }

    fn stack_mut(&mut self) -> &mut Self::Stack {
        &mut self.stack
    }
}

/// Iterator that calls [Queue::pop] until it reaches the end of the document.
///
/// The iterator traverses into the content of any [FormatElement::Interned].
pub(super) struct QueueIterator<'a, 'q, Q: Queue<'a>> {
    queue: &'q mut Q,
    lifetime: PhantomData<&'a ()>,
}

impl<'a, Q> Iterator for QueueIterator<'a, '_, Q>
where
    Q: Queue<'a>,
{
    type Item = &'a FormatElement;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop()
    }
}

impl<'a, Q> FusedIterator for QueueIterator<'a, '_, Q> where Q: Queue<'a> {}

pub(super) struct QueueContentIterator<'a, 'q, Q: Queue<'a>> {
    queue: &'q mut Q,
    kind: TagKind,
    depth: usize,
    lifetime: PhantomData<&'a ()>,
}

impl<'a, 'q, Q> QueueContentIterator<'a, 'q, Q>
where
    Q: Queue<'a>,
{
    fn new(queue: &'q mut Q, kind: TagKind) -> Self {
        Self {
            queue,
            kind,
            depth: 1,
            lifetime: PhantomData,
        }
    }
}

impl<'a, Q> Iterator for QueueContentIterator<'a, '_, Q>
where
    Q: Queue<'a>,
{
    type Item = &'a FormatElement;

    fn next(&mut self) -> Option<Self::Item> {
        match self.depth {
            0 => None,
            _ => {
                let mut top = self.queue.pop();

                while let Some(FormatElement::Interned(interned)) = top {
                    self.queue.extend_back(interned);
                    top = self.queue.pop();
                }

                match top.expect("Missing end signal.") {
                    element @ FormatElement::Tag(tag) if tag.kind() == self.kind => {
                        if tag.is_start() {
                            self.depth += 1;
                        } else {
                            self.depth -= 1;

                            if self.depth == 0 {
                                return None;
                            }
                        }

                        Some(element)
                    }
                    element => Some(element),
                }
            }
        }
    }
}

impl<'a, Q> FusedIterator for QueueContentIterator<'a, '_, Q> where Q: Queue<'a> {}

/// A predicate determining when to end measuring if some content fits on the line.
///
/// Called for every [`element`](FormatElement) in the [FitsQueue] when measuring if a content
/// fits on the line. The measuring of the content ends after the first element [`element`](FormatElement) for which this
/// predicate returns `true` (similar to a take while iterator except that it takes while the predicate returns `false`).
pub(super) trait FitsEndPredicate {
    fn is_end(&mut self, element: &FormatElement) -> PrintResult<bool>;
}

/// Filter that includes all elements until it reaches the end of the document.
pub(super) struct AllPredicate;

impl FitsEndPredicate for AllPredicate {
    fn is_end(&mut self, _element: &FormatElement) -> PrintResult<bool> {
        Ok(false)
    }
}

/// Filter that takes all elements between two matching [Tag::StartEntry] and [Tag::EndEntry] tags.
#[derive(Debug)]
pub(super) enum SingleEntryPredicate {
    Entry { depth: usize },
    Done,
}

impl SingleEntryPredicate {
    pub(super) const fn is_done(&self) -> bool {
        matches!(self, SingleEntryPredicate::Done)
    }
}

impl Default for SingleEntryPredicate {
    fn default() -> Self {
        SingleEntryPredicate::Entry { depth: 0 }
    }
}

impl FitsEndPredicate for SingleEntryPredicate {
    fn is_end(&mut self, element: &FormatElement) -> PrintResult<bool> {
        let result = match self {
            SingleEntryPredicate::Done => true,
            SingleEntryPredicate::Entry { depth } => match element {
                FormatElement::Tag(Tag::StartEntry) => {
                    *depth += 1;

                    false
                }
                FormatElement::Tag(Tag::EndEntry) => {
                    if *depth == 0 {
                        return invalid_end_tag(TagKind::Entry, None);
                    }

                    *depth -= 1;

                    let is_end = *depth == 0;

                    if is_end {
                        *self = SingleEntryPredicate::Done;
                    }

                    is_end
                }
                FormatElement::Interned(_) => false,
                element if *depth == 0 => {
                    return invalid_start_tag(TagKind::Entry, Some(element));
                }
                _ => false,
            },
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::format_element::LineMode;
    use crate::prelude::Tag;
    use crate::printer::queue::{PrintQueue, Queue};
    use crate::FormatElement;

    #[test]
    fn extend_back_pop_last() {
        let mut queue =
            PrintQueue::new(&[FormatElement::Tag(Tag::StartEntry), FormatElement::Space]);

        assert_eq!(queue.pop(), Some(&FormatElement::Tag(Tag::StartEntry)));

        queue.extend_back(&[FormatElement::Line(LineMode::SoftOrSpace)]);

        assert_eq!(
            queue.pop(),
            Some(&FormatElement::Line(LineMode::SoftOrSpace))
        );
        assert_eq!(queue.pop(), Some(&FormatElement::Space));

        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn extend_back_empty_queue() {
        let mut queue =
            PrintQueue::new(&[FormatElement::Tag(Tag::StartEntry), FormatElement::Space]);

        assert_eq!(queue.pop(), Some(&FormatElement::Tag(Tag::StartEntry)));
        assert_eq!(queue.pop(), Some(&FormatElement::Space));

        queue.extend_back(&[FormatElement::Line(LineMode::SoftOrSpace)]);

        assert_eq!(
            queue.pop(),
            Some(&FormatElement::Line(LineMode::SoftOrSpace))
        );

        assert_eq!(queue.pop(), None);
    }
}
