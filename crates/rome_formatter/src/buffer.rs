use super::{write, Arguments, FormatElement};
use crate::format_element::List;
use crate::formatter::FormatState;
use std::any::{Any, TypeId};

use crate::{Format, FormatResult};

use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

pub trait Buffer {
    type Context;

    fn write_element(&mut self, element: FormatElement) -> FormatResult<()>;

    fn write_fmt(mut self: &mut Self, arguments: &Arguments<Self::Context>) -> FormatResult<()> {
        write(&mut self, arguments)
    }

    fn state(&self) -> &FormatState<Self::Context>;

    fn state_mut(&mut self) -> &mut FormatState<Self::Context>;

    /// Takes a snapshot of the Buffers state, excluding the formatter state.
    fn snapshot(&self) -> BufferSnapshot;

    /// Restores the snapshot with the given id.
    ///
    /// ## Panics
    /// If the passed snapshot id is a snapshot of another buffer OR
    /// if the snapshot is restored out of order
    fn restore_snapshot(&mut self, snapshot: BufferSnapshot);
}

#[derive(Debug)]
pub enum BufferSnapshot {
    Position(usize),
    Any(Box<dyn Any>),
}

impl BufferSnapshot {
    pub const fn position(index: usize) -> Self {
        Self::Position(index)
    }

    /// Unwraps the position value.
    ///
    /// ## Panics
    /// If self is not [BufferSnapshot::Position]
    pub fn unwrap_position(&self) -> usize {
        match self {
            BufferSnapshot::Position(index) => *index,
            BufferSnapshot::Any(_) => panic!("Tried to unwrap Any snapshot as a position."),
        }
    }

    pub fn unwrap_any<T: 'static>(self) -> T {
        match self {
            BufferSnapshot::Position(_) => {
                panic!("Tried to unwrap Position snapshot as Any snapshot.")
            }
            BufferSnapshot::Any(value) => match value.downcast::<T>() {
                Ok(snapshot) => *snapshot,
                Err(err) => {
                    panic!(
                        "Tried to unwrap snapshot of type {:?} as {:?}",
                        err.type_id(),
                        TypeId::of::<T>()
                    )
                }
            },
        }
    }
}

impl<W: Buffer<Context = Context> + ?Sized, Context> Buffer for &mut W {
    type Context = Context;

    fn write_element(&mut self, element: FormatElement) -> FormatResult<()> {
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

    fn snapshot(&self) -> BufferSnapshot {
        (**self).snapshot()
    }

    fn restore_snapshot(&mut self, snapshot: BufferSnapshot) {
        (**self).restore_snapshot(snapshot)
    }
}

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

    fn write_element(&mut self, element: FormatElement) -> FormatResult<()> {
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

        Ok(())
    }

    fn state(&self) -> &FormatState<Self::Context> {
        self.state
    }

    fn state_mut(&mut self) -> &mut FormatState<Self::Context> {
        &mut self.state
    }

    fn snapshot(&self) -> BufferSnapshot {
        BufferSnapshot::position(self.elements.len())
    }

    fn restore_snapshot(&mut self, snapshot: BufferSnapshot) {
        let position = snapshot.unwrap_position();
        assert!(self.elements.len() >= position);

        self.elements.truncate(position)
    }
}

/// Buffer that writes a pre-amble before the first written content.
/// Emits nothing if the buffer is empty
pub struct PreambleBuffer<'buf, Preamble, Context> {
    inner: &'buf mut dyn Buffer<Context = Context>,
    preamble: Preamble,
    empty: bool,
}

impl<'buf, Preamble, Context> PreambleBuffer<'buf, Preamble, Context> {
    pub fn new(inner: &'buf mut dyn Buffer<Context = Context>, preamble: Preamble) -> Self {
        Self {
            inner,
            preamble,
            empty: true,
        }
    }

    /// Returns `true` if the preamble has been written, false otherwise
    pub fn did_write_preamble(&self) -> bool {
        !self.empty
    }
}

impl<Preamble, Context> Buffer for PreambleBuffer<'_, Preamble, Context>
where
    Preamble: Format<Context>,
{
    type Context = Context;

    fn write_element(&mut self, element: FormatElement) -> FormatResult<()> {
        if self.empty {
            write!(self.inner, [&self.preamble])?;
            self.empty = false;
        }

        self.inner.write_element(element)
    }

    fn state(&self) -> &FormatState<Self::Context> {
        self.inner.state()
    }

    fn state_mut(&mut self) -> &mut FormatState<Self::Context> {
        self.inner.state_mut()
    }

    fn snapshot(&self) -> BufferSnapshot {
        BufferSnapshot::Any(Box::new(PreambleBufferSnapshot {
            inner: self.inner.snapshot(),
            empty: self.empty,
        }))
    }

    fn restore_snapshot(&mut self, snapshot: BufferSnapshot) {
        let snapshot = snapshot.unwrap_any::<PreambleBufferSnapshot>();

        self.empty = snapshot.empty;
        self.inner.restore_snapshot(snapshot.inner);
    }
}

pub struct PreambleBufferSnapshot {
    inner: BufferSnapshot,
    empty: bool,
}
