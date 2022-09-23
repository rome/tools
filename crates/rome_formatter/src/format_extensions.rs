use crate::prelude::*;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::Deref;

use crate::Buffer;

/// Utility trait used to simplify the formatting of optional objects that are formattable.
pub trait FormatOptional<Context> {
    type Target: Format<Context>;

    /// This function tries to format an optional object. If the object is [None]
    /// nothing is written in the buffer. If exists, the utility
    /// formats the object and passes it to the closure.
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_formatter::{write, format};
    /// use rome_formatter::prelude::*;
    /// use rome_rowan::TextSize;
    ///
    /// struct MyFormat;
    ///
    /// impl Format<SimpleFormatContext> for MyFormat {
    ///     fn fmt(&self, f: &mut Formatter<SimpleFormatContext>) -> FormatResult<()> {
    ///         write!(f, [text("MyToken")])
    ///     }
    /// }
    ///
    /// let none_token: Option<MyFormat> = None;
    /// // Returns `empty_element()` for a `None` value
    /// let none_formatted = format!(SimpleFormatContext::default(), [
    ///     none_token.with_or_empty(|token, f| write!(f, [token]))
    /// ]).unwrap();
    ///
    /// assert!(none_formatted.into_document().is_empty());
    ///
    /// let some_token = Some(MyFormat);
    /// assert_eq!(
    ///     format![SimpleFormatContext::default(), [space(), text("MyToken")]],
    ///     format!(
    ///         SimpleFormatContext::default(), [
    ///             some_token.with_or_empty(|token, f| {
    ///                 write!(f, [space(), token])
    ///             })
    ///         ]
    ///     )
    /// );
    fn with_or_empty<With>(self, with: With) -> Option<FormatItemWith<With, Self::Target>>
    where
        With: Fn(&Self::Target, &mut Formatter<Context>) -> FormatResult<()>;
}

#[derive(Copy, Clone, Debug)]
pub struct FormatItemWith<With, Format> {
    with: With,
    inner: Format,
}

impl<With, F, Context> Format<Context> for FormatItemWith<With, F>
where
    F: Format<Context>,
    With: Fn(&F, &mut Formatter<Context>) -> FormatResult<()>,
{
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        (self.with)(&self.inner, f)
    }
}

impl<F: Format<Context>, Context> FormatOptional<Context> for Option<F> {
    type Target = F;

    #[inline]
    fn with_or_empty<With>(self, with: With) -> Option<FormatItemWith<With, F>>
    where
        With: Fn(&F, &mut Formatter<Context>) -> FormatResult<()>,
    {
        self.map(|value| FormatItemWith { inner: value, with })
    }
}

/// Utility trait that allows memorizing the output of a [Format].
/// Useful to avoid re-formatting the same object twice.
pub trait MemoizeFormat<Context> {
    /// Returns a formattable object that memoizes the result of `Format` by cloning.
    /// Mainly useful if the same sub-tree can appear twice in the formatted output because it's
    /// used inside of `if_group_breaks` or `if_group_fits_single_line`.
    ///
    /// ```
    /// use std::cell::Cell;
    /// use rome_formatter::{format, write};
    /// use rome_formatter::prelude::*;
    /// use rome_rowan::TextSize;
    ///
    /// struct MyFormat {
    ///   value: Cell<u64>
    /// }
    ///
    /// impl MyFormat {
    ///     pub fn new() -> Self {
    ///         Self { value: Cell::new(1) }
    ///     }
    /// }
    ///
    /// impl Format<SimpleFormatContext> for MyFormat {
    ///     fn fmt(&self, f: &mut Formatter<SimpleFormatContext>) -> FormatResult<()> {
    ///         let value = self.value.get();
    ///         self.value.set(value + 1);
    ///
    ///         write!(f, [dynamic_text(&std::format!("Formatted {value} times."), TextSize::from(0))])
    ///     }
    /// }
    ///
    /// # fn main() -> FormatResult<()> {
    /// let normal = MyFormat::new();
    ///
    /// // Calls `format` for everytime the object gets formatted
    /// assert_eq!(
    ///     "Formatted 1 times. Formatted 2 times.",
    ///     format!(SimpleFormatContext::default(), [normal, space(), normal])?.print()?.as_code()
    /// );
    ///
    /// // Memoized memoizes the result and calls `format` only once.
    /// let memoized = normal.memoized();
    /// assert_eq!(
    ///     "Formatted 3 times. Formatted 3 times.",
    ///     format![SimpleFormatContext::default(), [memoized, space(), memoized]]?.print()?.as_code()
    /// );
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn memoized(self) -> Memoized<Self, Context>
    where
        Self: Sized + Format<Context>,
    {
        Memoized::new(self)
    }
}

impl<T, Context> MemoizeFormat<Context> for T where T: Format<Context> {}

/// Memoizes the output of its inner [Format] to avoid re-formatting a potential expensive object.
#[derive(Debug)]
pub struct Memoized<F, Context> {
    inner: F,
    memory: RefCell<Option<FormatResult<Option<FormatElement>>>>,
    options: PhantomData<Context>,
}

impl<F, Context> Memoized<F, Context>
where
    F: Format<Context>,
{
    fn new(inner: F) -> Self {
        Self {
            inner,
            memory: RefCell::new(None),
            options: PhantomData,
        }
    }

    /// Gives access to the memoized content.
    ///
    /// Performs the formatting if the content hasn't been formatted at this point.
    ///
    /// # Example
    ///
    /// Inspect if some memoized content breaks.
    ///
    /// ```rust
    /// use std::cell::Cell;
    /// use rome_formatter::{format, write};
    /// use rome_formatter::prelude::*;
    /// use rome_rowan::TextSize;
    ///
    /// #[derive(Default)]
    /// struct Counter {
    ///   value: Cell<u64>
    /// }
    ///
    /// impl Format<SimpleFormatContext> for Counter {
    ///     fn fmt(&self, f: &mut Formatter<SimpleFormatContext>) -> FormatResult<()> {
    ///         let current = self.value.get();
    ///
    ///         write!(f, [
    ///             text("Count:"),
    ///             space(),
    ///             dynamic_text(&std::format!("{current}"), TextSize::default()),
    ///             hard_line_break()
    ///         ])?;
    ///
    ///         self.value.set(current + 1);
    ///         Ok(())
    ///     }
    /// }
    ///
    /// # fn main() -> FormatResult<()> {
    /// let content = format_with(|f| {
    ///     let mut counter = Counter::default().memoized();
    ///     let counter_content = counter.inspect(f)?;
    ///
    ///     if counter_content.will_break() {
    ///         write!(f, [text("Counter:"), block_indent(&counter)])
    ///     } else {
    ///         write!(f, [text("Counter:"), counter])
    ///     }?;
    ///
    ///     write!(f, [counter])
    /// });
    ///
    ///
    /// let formatted = format!(SimpleFormatContext::default(), [content])?;
    /// assert_eq!("Counter:\n\tCount: 0\nCount: 0\n", formatted.print()?.as_code());
    /// # Ok(())
    /// # }
    ///
    /// ```
    pub fn inspect(&mut self, f: &mut Formatter<Context>) -> FormatResult<&[FormatElement]> {
        let result = self
            .memory
            .get_mut()
            .get_or_insert_with(|| f.intern(&self.inner));

        match result.as_ref() {
            Ok(Some(FormatElement::Interned(interned))) => Ok(interned.deref()),
            Ok(Some(other)) => Ok(std::slice::from_ref(other)),
            Ok(None) => Ok(&[]),
            Err(error) => Err(*error),
        }
    }
}

impl<F, Context> Format<Context> for Memoized<F, Context>
where
    F: Format<Context>,
{
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        let mut memory = self.memory.borrow_mut();
        let result = memory.get_or_insert_with(|| f.intern(&self.inner));

        match result {
            Ok(Some(elements)) => {
                f.write_element(elements.clone())?;

                Ok(())
            }
            Ok(None) => Ok(()),
            Err(err) => Err(*err),
        }
    }
}
