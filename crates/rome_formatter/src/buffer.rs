use super::{write, Arguments, FormatElement};
use crate::format_element::List;
use crate::formatter::FormatState;

use crate::FormatResult;

use std::fmt;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

pub trait Buffer {
    type Context;

    fn write_element(&mut self, element: FormatElement);

    fn write_fmt(mut self: &mut Self, arguments: &Arguments<Self::Context>) -> FormatResult<()> {
        write(&mut self, arguments)
    }

    fn state(&self) -> &FormatState<Self::Context>;

    fn state_mut(&mut self) -> &mut FormatState<Self::Context>;

    /// Takes a snapshot of the Buffers state, excluding the formatter state.
    fn snapshot(&mut self) -> BufferSnapshotId;

    /// Restores the snapshot with the given id.
    ///
    /// ## Panics
    /// If the passed snapshot id is a snapshot of another buffer OR
    /// if the snapshot is restored out of order
    fn restore_snapshot(&mut self, snapshot: BufferSnapshotId);

    /// Releases the snapshot with the given id
    ///
    /// ## Panics
    /// If the passed snapshot id is a snapshot of another buffer OR
    /// If the snapshot is restored out of order.
    fn release_snapshot(&mut self, snapshot: BufferSnapshotId);
}

/// Id of a buffer snapshot. What the meaning of the inner value is depends on the [Buffer] implementation.
/// It can either be an index into a local store with all taken snapshots that stores additional values
/// or it may be the length of the internal [FormatElement] buffer.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct BufferSnapshotId(usize);

impl BufferSnapshotId {
    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    pub const fn value(&self) -> usize {
        self.0
    }
}

impl<W: Buffer<Context = Context> + ?Sized, Context> Buffer for &mut W {
    type Context = Context;

    fn write_element(&mut self, element: FormatElement) {
        (**self).write_element(element)
    }

    fn write_fmt(&mut self, args: &Arguments<Context>) -> FormatResult<()> {
        (**self).write_fmt(args)
    }

    fn state(&self) -> &FormatState<Self::Context> {
        (**self).state()
    }

    fn state_mut(&mut self) -> &mut FormatState<Self::Context> {
        (**self).state_mut()
    }

    fn snapshot(&mut self) -> BufferSnapshotId {
        (**self).snapshot()
    }

    fn restore_snapshot(&mut self, snapshot: BufferSnapshotId) {
        (**self).restore_snapshot(snapshot)
    }

    fn release_snapshot(&mut self, snapshot: BufferSnapshotId) {
        (**self).release_snapshot(snapshot)
    }
}

// TODO use Smallvec internally
#[derive(Debug)]
pub struct VecBuffer<'a, Context> {
    state: &'a mut FormatState<Context>,
    elements: Vec<FormatElement>,
}

impl<'a, Context> VecBuffer<'a, Context> {
    pub fn new(state: &'a mut FormatState<Context>) -> Self {
        Self {
            state,
            elements: vec![],
        }
    }

    pub fn with_capacity(capacity: usize, context: &'a mut FormatState<Context>) -> Self {
        Self {
            state: context,
            elements: Vec::with_capacity(capacity),
        }
    }

    /// Writes the elements from this buffer into the passed buffer
    pub fn write_into(
        &mut self,
        buffer: &mut dyn Buffer<Context = Context>,
    ) -> super::FormatResult<()> {
        for element in self.drain(..) {
            buffer.write_element(element);
        }

        Ok(())
    }

    pub fn into_document(self) -> Document {
        Document(self.elements)
    }

    pub fn into_element(mut self) -> FormatElement {
        if self.len() == 1 {
            // Safety: Guaranteed by len check above
            self.elements.pop().unwrap()
        } else {
            FormatElement::List(List::new(self.elements))
        }
    }

    pub fn into_vec(self) -> Vec<FormatElement> {
        self.elements
    }
}

impl<Context> Deref for VecBuffer<'_, Context> {
    type Target = Vec<FormatElement>;

    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

impl<Context> DerefMut for VecBuffer<'_, Context> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements
    }
}

impl<Context> Buffer for VecBuffer<'_, Context> {
    type Context = Context;

    fn write_element(&mut self, element: FormatElement) {
        match element {
            FormatElement::List(list) => {
                if self.elements.is_empty() {
                    self.elements = list.into_vec()
                } else {
                    self.elements.extend(list.into_vec())
                }
            }
            element => self.elements.push(element),
        }
    }

    fn state(&self) -> &FormatState<Self::Context> {
        self.state
    }

    fn state_mut(&mut self) -> &mut FormatState<Self::Context> {
        &mut self.state
    }

    fn snapshot(&mut self) -> BufferSnapshotId {
        BufferSnapshotId::new(self.elements.len())
    }

    fn restore_snapshot(&mut self, snapshot: BufferSnapshotId) {
        assert!(self.elements.len() >= snapshot.value());

        self.elements.truncate(snapshot.value())
    }

    fn release_snapshot(&mut self, snapshot: BufferSnapshotId) {
        assert!(self.elements.len() >= snapshot.value());
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Document(Vec<FormatElement>);

impl Document {
    pub fn into_vec(self) -> Vec<FormatElement> {
        self.0
    }

    pub fn into_element(mut self) -> FormatElement {
        if self.0.len() == 1 {
            self.0.pop().unwrap()
        } else {
            FormatElement::List(List::new(self.0))
        }
    }
}

impl FromIterator<FormatElement> for Document {
    fn from_iter<T: IntoIterator<Item = FormatElement>>(iter: T) -> Self {
        Document(Vec::from_iter(iter))
    }
}

impl fmt::Debug for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(&self.0).finish()
    }
}

impl Deref for Document {
    type Target = [FormatElement];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
