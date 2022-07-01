use super::{write, Arguments, FormatElement};
use crate::format_element::{LabelId, List};
use crate::{Format, FormatResult, FormatState};
use std::any::{Any, TypeId};
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

/// A trait for writing or formatting into [FormatElement]-accepting buffers or streams.
pub trait Buffer {
    /// The context used during formatting
    type Context;

    /// Writes a [crate::FormatElement] into this buffer, returning whether the write succeeded.
    ///
    /// # Errors
    /// This function will return an instance of [crate::FormatError] on error.
    ///
    /// # Examples
    ///
    /// ```
    /// use rome_formatter::{Buffer, FormatElement, FormatState, SimpleFormatContext, Token, VecBuffer};
    ///
    /// let mut state = FormatState::new(SimpleFormatContext::default());
    /// let mut buffer = VecBuffer::new(&mut state);
    ///
    /// buffer.write_element(FormatElement::Token( Token::Static { text: "test"})).unwrap();
    ///
    /// assert_eq!(buffer.into_element(), FormatElement::Token( Token::Static { text: "test"}));
    /// ```
    ///
    fn write_element(&mut self, element: FormatElement) -> FormatResult<()>;

    /// Glue for usage of the [`write!`] macro with implementors of this trait.
    ///
    /// This method should generally not be invoked manually, but rather through the [`write!`] macro itself.
    ///
    /// # Examples
    ///
    /// ```
    /// use rome_formatter::prelude::*;
    /// use rome_formatter::{Buffer, FormatState, SimpleFormatContext, Token, VecBuffer, format_args};
    ///
    /// let mut state = FormatState::new(SimpleFormatContext::default());
    /// let mut buffer = VecBuffer::new(&mut state);
    ///
    /// buffer.write_fmt(format_args!(token("Hello World"))).unwrap();
    ///
    /// assert_eq!(buffer.into_element(), FormatElement::Token( Token::Static { text: "Hello World"}));
    /// ```
    fn write_fmt(mut self: &mut Self, arguments: Arguments<Self::Context>) -> FormatResult<()> {
        write(&mut self, arguments)
    }

    /// Returns the formatting state relevant for this formatting session.
    fn state(&self) -> &FormatState<Self::Context>;

    /// Returns the mutable formatting state relevant for this formatting session.
    fn state_mut(&mut self) -> &mut FormatState<Self::Context>;

    /// Takes a snapshot of the Buffers state, excluding the formatter state.
    fn snapshot(&self) -> BufferSnapshot;

    /// Restores the snapshot buffer
    ///
    /// ## Panics
    /// If the passed snapshot id is a snapshot of another buffer OR
    /// if the snapshot is restored out of order
    fn restore_snapshot(&mut self, snapshot: BufferSnapshot);
}

/// Snapshot of a buffer state that can be restored at a later point.
///
/// Used in cases where the formatting of an object fails but a parent formatter knows an alternative
/// strategy on how to format the object that might succeed.
#[derive(Debug)]
pub enum BufferSnapshot {
    /// Stores an absolute position of a buffers state, for example, the offset of the last written element.
    Position(usize),

    /// Generic structure for custom buffers that need to store more complex data. Slightly more
    /// expensive because it requires allocating the buffer state on the heap.
    Any(Box<dyn Any>),
}

impl BufferSnapshot {
    /// Creates a new buffer snapshot that points to the specified position.
    pub const fn position(index: usize) -> Self {
        Self::Position(index)
    }

    /// Unwraps the position value.
    ///
    /// # Panics
    ///
    /// If self is not a [`BufferSnapshot::Position`]
    pub fn unwrap_position(&self) -> usize {
        match self {
            BufferSnapshot::Position(index) => *index,
            BufferSnapshot::Any(_) => panic!("Tried to unwrap Any snapshot as a position."),
        }
    }

    /// Unwraps the any value.
    ///
    /// # Panics
    ///
    /// If `self` is not a [`BufferSnapshot::Any`].
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

/// Implements the `[Buffer]` trait for all mutable references of objects implementing [Buffer].
impl<W: Buffer<Context = Context> + ?Sized, Context> Buffer for &mut W {
    type Context = Context;

    fn write_element(&mut self, element: FormatElement) -> FormatResult<()> {
        (**self).write_element(element)
    }

    fn write_fmt(&mut self, args: Arguments<Context>) -> FormatResult<()> {
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

/// Vector backed [`Buffer`] implementation.
///
/// The buffer writes all elements into the internal elements buffer.
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

    /// Creates a buffer with the specified capacity
    pub fn with_capacity(capacity: usize, context: &'a mut FormatState<Context>) -> Self {
        Self {
            state: context,
            elements: Vec::with_capacity(capacity),
        }
    }

    /// Consumes the buffer and returns its content as a [`FormatElement`]
    pub fn into_element(mut self) -> FormatElement {
        self.take_element()
    }

    /// Consumes the buffer and returns the written [`FormatElement]`s as a vector.
    pub fn into_vec(self) -> Vec<FormatElement> {
        self.elements
    }

    /// Takes the elements without consuming self
    pub fn take_element(&mut self) -> FormatElement {
        if self.len() == 1 {
            // Safety: Guaranteed by len check above
            self.elements.pop().unwrap()
        } else {
            FormatElement::List(List::new(std::mem::take(&mut self.elements)))
        }
    }
}

impl<Context> Deref for VecBuffer<'_, Context> {
    type Target = [FormatElement];

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
            FormatElement::List(list) => self.elements.extend(list.into_vec()),
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
        assert!(
            self.elements.len() >= position,
            r#"Outdated snapshot. This buffer contains fewer elements than at the time the snapshot was taken.
Make sure that you take and restore the snapshot in order and that this snapshot belongs to the current buffer."#
        );

        self.elements.truncate(position);
    }
}

/// This struct wraps an existing buffer and emits a preamble text when the first text is written.
///
/// This can be useful if you, for example, want to write some content if what gets written next isn't empty.
///
/// # Examples
///
/// ```
/// use rome_formatter::{FormatState, Formatted, PreambleBuffer, SimpleFormatContext, VecBuffer, write};
/// use rome_formatter::prelude::*;
///
/// let mut state = FormatState::new(SimpleFormatContext::default());
/// let mut buffer = VecBuffer::new(&mut state);
///
/// struct Preamble;
///
/// impl Format<SimpleFormatContext> for Preamble {
///     fn fmt(&self, f: &mut Formatter<SimpleFormatContext>) -> FormatResult<()> {
///         write!(f, [token("# heading"), hard_line_break()])
///     }
/// }
///
/// let mut with_preamble = PreambleBuffer::new(&mut buffer, Preamble);
///
/// write!(&mut with_preamble, [token("this text will be on a new line")]).unwrap();
///
/// drop(with_preamble);
///
/// let formatted = Formatted::new(buffer.into_element(), PrinterOptions::default());
/// assert_eq!("# heading\nthis text will be on a new line", formatted.print().as_code());
/// ```
///
/// The pre-amble does not get written if no content is written to the buffer.
///
/// ```
/// use rome_formatter::{FormatState, Formatted, PreambleBuffer, SimpleFormatContext, VecBuffer, write};
/// use rome_formatter::prelude::*;
///
/// let mut state = FormatState::new(SimpleFormatContext::default());
/// let mut buffer = VecBuffer::new(&mut state);
///
/// struct Preamble;
///
/// impl Format<SimpleFormatContext> for Preamble {
///     fn fmt(&self, f: &mut Formatter<SimpleFormatContext>) -> FormatResult<()> {
///         write!(f, [token("# heading"), hard_line_break()])
///     }
/// }
///
/// let mut with_preamble = PreambleBuffer::new(&mut buffer, Preamble);
/// drop(with_preamble);
///
/// let formatted = Formatted::new(buffer.into_element(), PrinterOptions::default());
/// assert_eq!("", formatted.print().as_code());
/// ```
pub struct PreambleBuffer<'buf, Preamble, Context> {
    /// The wrapped buffer
    inner: &'buf mut dyn Buffer<Context = Context>,

    /// The pre-amble to write once the first content gets written to this buffer.
    preamble: Preamble,

    /// Whether some content (including the pre-amble) has been written at this point.
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

    /// Returns `true` if the preamble has been written, `false` otherwise.
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
        if element.is_empty() {
            Ok(())
        } else {
            if self.empty {
                write!(self.inner, [&self.preamble])?;
                self.empty = false;
            }

            self.inner.write_element(element)
        }
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

struct PreambleBufferSnapshot {
    inner: BufferSnapshot,
    empty: bool,
}

/// Buffer that allows you inspecting elements as they get written to the formatter.
pub struct Inspect<'inner, Context, Inspector> {
    inner: &'inner mut dyn Buffer<Context = Context>,
    inspector: Inspector,
}

impl<'inner, Context, Inspector> Inspect<'inner, Context, Inspector> {
    fn new(inner: &'inner mut dyn Buffer<Context = Context>, inspector: Inspector) -> Self {
        Self { inner, inspector }
    }
}

impl<'inner, Context, Inspector> Buffer for Inspect<'inner, Context, Inspector>
where
    Inspector: FnMut(&FormatElement),
{
    type Context = Context;

    fn write_element(&mut self, element: FormatElement) -> FormatResult<()> {
        (self.inspector)(&element);
        self.inner.write_element(element)
    }

    fn state(&self) -> &FormatState<Self::Context> {
        self.inner.state()
    }

    fn state_mut(&mut self) -> &mut FormatState<Self::Context> {
        self.inner.state_mut()
    }

    fn snapshot(&self) -> BufferSnapshot {
        self.inner.snapshot()
    }

    fn restore_snapshot(&mut self, snapshot: BufferSnapshot) {
        self.inner.restore_snapshot(snapshot)
    }
}

pub trait BufferExtensions: Buffer + Sized {
    /// Returns a new buffer that calls the passed inspector for every element that gets written to the output
    #[must_use]
    fn inspect<F>(&mut self, inspector: F) -> Inspect<Self::Context, F>
    where
        F: FnMut(&FormatElement),
    {
        Inspect::new(self, inspector)
    }

    /// Writes a sequence of elements into this buffer.
    fn write_elements<I>(&mut self, elements: I) -> FormatResult<()>
    where
        I: IntoIterator<Item = FormatElement>,
    {
        for element in elements {
            self.write_element(element)?;
        }

        Ok(())
    }

    /// It emits a custom buffer called [WillBreakBuffer], which tracks
    /// it he last element written in the main buffer breaks, it does so by
    /// checking if their IR emits an [element](FormatElement) that breaks.
    ///
    /// This functionality can be used only one element and only after the element
    /// is written in the buffer.
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_formatter::{format, format_args, write, LineWidth};
    /// use rome_formatter::prelude::*;
    ///
    /// let context = SimpleFormatContext {
    ///     line_width: LineWidth::try_from(20).unwrap(),
    ///     ..SimpleFormatContext::default()
    /// };
    ///
    ///
    /// let formatted = format!(context, [format_with(|f| {
    ///     
    ///     let element = format_with(|f| {
    ///         write!(f, [
    ///             token("hello"),
    ///             hard_line_break(),
    ///             token("world!")
    ///         ])
    ///     });
    ///     let mut buffer = f.inspect_will_break();
    ///     write!(buffer, [element])?;
    ///     let does_element_break = buffer.will_break();
    ///
    ///     if does_element_break {
    ///         write!(f, [hard_line_break(), token("break")])
    ///     } else {
    ///         write!(f, [token("did not break")])
    ///     }   
    ///    
    /// })]).unwrap();
    ///
    /// assert_eq!(
    ///     "hello\nworld!\nbreak",
    ///     formatted.print().as_code()
    /// );
    /// ```
    fn inspect_will_break(&mut self) -> WillBreakBuffer<Self::Context> {
        WillBreakBuffer::new(self)
    }

    /// It emits a custom buffer called [IsLabelledBuffer], which tracks
    /// labelled elements written in the main buffer, it does so by
    /// checking if [element](FormatElement) is [label](FormatElement::Label)
    /// with expected [label_id](LabelId).
    ///
    /// This functionality can be used only on one element and only after the element
    /// is written in the buffer.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use rome_formatter::prelude::*;
    /// use rome_formatter::{format, write, LineWidth};
    ///
    /// enum SomeLabelId {}
    ///
    /// let context = SimpleFormatContext {
    ///     line_width: LineWidth::try_from(20).unwrap(),
    ///     ..SimpleFormatContext::default()
    /// };
    ///
    /// let formatted = format!(
    ///     context,
    ///     [format_with(|f| {
    ///         let element = format_with(|f| {
    ///             let label_id = f.label_of::<SomeLabelId>();
    ///
    ///             write!(f, [labelled(label_id, &token("labelled")),])
    ///         })
    ///         .memoized();
    ///
    ///         let is_labelled = {
    ///             let mut null_buffer = f.inspect_null();
    ///             let mut buffer = null_buffer.inspect_is_labelled::<SomeLabelId>();
    ///             write!(buffer, [element])?;
    ///             buffer.is_labelled()
    ///         };
    ///
    ///         if is_labelled {
    ///             write!(f, [token("This is "), &element])
    ///         } else {
    ///             write!(f, [token("This is not "), &element])
    ///         }
    ///     })]
    /// )
    /// .unwrap();
    ///
    /// assert_eq!("This is labelled", formatted.print().as_code());
    /// ```
    fn inspect_is_labelled<T: ?Sized + 'static>(&mut self) -> IsLabelledBuffer<Self::Context> {
        let label_id = self.label_of::<T>();
        IsLabelledBuffer::new(self, label_id)
    }

    /// It creates a buffer where all the elements are ignored, so the elements
    /// are not written anywhere at all.
    ///
    /// This can be useful when formatters are not yet written inside the main buffer
    /// and the consumer needs to inspect them, to decide the formatting layout in advance.
    ///
    /// ## Examples
    ///
    /// The following example shows how to use it with the `will_break` functionality
    ///
    /// ```
    /// use rome_formatter::{format, format_args, write, LineWidth};
    /// use rome_formatter::prelude::*;
    ///
    /// let context = SimpleFormatContext {
    ///     line_width: LineWidth::try_from(20).unwrap(),
    ///     ..SimpleFormatContext::default()
    /// };
    ///
    ///
    /// let formatted = format!(context, [format_with(|f| {
    ///
    ///     let element = format_with(|f| {
    ///         write!(f, [
    ///             token("hello"),
    ///             hard_line_break(),
    ///             token("world!")
    ///         ])
    ///     }).memoized();
    ///
    ///     let will_break =  {
    ///         let mut null_buffer =  f.inspect_null();
    ///         let mut buffer = null_buffer.inspect_will_break();
    ///         write!(buffer, [element])?;
    ///         buffer.will_break()
    ///     };
    ///
    ///
    ///     if will_break {
    ///         write!(f, [token("break"), hard_line_break(), &element])
    ///     } else {
    ///         write!(f, [token("did not break")])
    ///     }   
    ///    
    /// })]).unwrap();
    ///
    /// assert_eq!(
    ///     "break\nhello\nworld!",
    ///     formatted.print().as_code()
    /// );
    /// ```
    #[must_use]
    fn inspect_null(&mut self) -> NullBuffer<Self::Context> {
        NullBuffer::new(self)
    }

    #[must_use]
    fn label_of<T: ?Sized + 'static>(&self) -> LabelId {
        LabelId::of::<T>()
    }
}

impl<T> BufferExtensions for T where T: Buffer {}

#[must_use = "must eventually call `is_labelled()` to retrieve the information"]
pub struct IsLabelledBuffer<'buffer, Context> {
    inner: &'buffer mut dyn Buffer<Context = Context>,
    label_id: LabelId,
    is_labelled: bool,
}

impl<'buffer, Context> IsLabelledBuffer<'buffer, Context> {
    pub fn new(buffer: &'buffer mut dyn Buffer<Context = Context>, label_id: LabelId) -> Self {
        Self {
            inner: buffer,
            label_id,
            is_labelled: false,
        }
    }

    pub fn is_labelled(&self) -> bool {
        self.is_labelled
    }
}

impl<Context> Buffer for IsLabelledBuffer<'_, Context> {
    type Context = Context;

    fn write_element(&mut self, element: FormatElement) -> FormatResult<()> {
        if !self.is_labelled {
            let label = match &element {
                FormatElement::Label(label) => Some(label),
                FormatElement::Interned(inner) => {
                    if let FormatElement::Label(label) = inner.deref() {
                        Some(label)
                    } else {
                        None
                    }
                }
                _ => None,
            };

            self.is_labelled = label
                .map(|label| label.label_id() == self.label_id)
                .unwrap_or(false);
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
        BufferSnapshot::Any(Box::new(IsLabelledSnapshot {
            inner: self.inner.snapshot(),
            label_id: self.label_id,
            is_labelled: self.is_labelled,
        }))
    }

    fn restore_snapshot(&mut self, snapshot: BufferSnapshot) {
        let snapshot = snapshot.unwrap_any::<IsLabelledSnapshot>();
        self.inner.restore_snapshot(snapshot.inner);
        self.is_labelled = snapshot.is_labelled;
        self.label_id = snapshot.label_id;
    }
}

struct IsLabelledSnapshot {
    inner: BufferSnapshot,
    label_id: LabelId,
    is_labelled: bool,
}

#[must_use = "must eventually call `will_break()` to retrieve the information"]
pub struct WillBreakBuffer<'buffer, Context> {
    breaks: bool,
    inner: &'buffer mut dyn Buffer<Context = Context>,
}

impl<'buffer, Context> WillBreakBuffer<'buffer, Context> {
    pub fn new(buffer: &'buffer mut dyn Buffer<Context = Context>) -> Self {
        Self {
            breaks: false,
            inner: buffer,
        }
    }

    pub fn will_break(&self) -> bool {
        self.breaks
    }
}

impl<Context> Buffer for WillBreakBuffer<'_, Context> {
    type Context = Context;

    fn write_element(&mut self, element: FormatElement) -> FormatResult<()> {
        self.breaks = self.breaks || element.will_break();
        self.inner.write_element(element)
    }

    fn state(&self) -> &FormatState<Self::Context> {
        self.inner.state()
    }

    fn state_mut(&mut self) -> &mut FormatState<Self::Context> {
        self.inner.state_mut()
    }

    fn snapshot(&self) -> BufferSnapshot {
        BufferSnapshot::Any(Box::new(WillBreakSnapshot {
            inner: self.inner.snapshot(),
            breaks: self.breaks,
        }))
    }

    fn restore_snapshot(&mut self, snapshot: BufferSnapshot) {
        let snapshot = snapshot.unwrap_any::<WillBreakSnapshot>();
        self.inner.restore_snapshot(snapshot.inner);
        self.breaks = snapshot.breaks;
    }
}

struct WillBreakSnapshot {
    inner: BufferSnapshot,
    breaks: bool,
}

pub struct NullBuffer<'buffer, Context> {
    inner: &'buffer mut dyn Buffer<Context = Context>,
}

impl<'buffer, Context> NullBuffer<'buffer, Context> {
    pub fn new(buffer: &'buffer mut dyn Buffer<Context = Context>) -> Self {
        Self { inner: buffer }
    }
}

impl<Context> Buffer for NullBuffer<'_, Context> {
    type Context = Context;

    fn write_element(&mut self, element: FormatElement) -> FormatResult<()> {
        drop(element);
        Ok(())
    }

    fn state(&self) -> &FormatState<Self::Context> {
        self.inner.state()
    }

    fn state_mut(&mut self) -> &mut FormatState<Self::Context> {
        self.inner.state_mut()
    }

    fn snapshot(&self) -> BufferSnapshot {
        BufferSnapshot::Position(0)
    }

    fn restore_snapshot(&mut self, snapshot: BufferSnapshot) {
        drop(snapshot);
    }
}
