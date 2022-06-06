use crate::prelude::*;
use std::cell::RefCell;
use std::marker::PhantomData;

use crate::{write, Buffer, VecBuffer};

/// Utility trait used to simplify the formatting of optional objects that are formattable.
///
/// In order to take advantage of all the functions, you only need to implement the [FormatOptionalTokenAndNode::with_or]
/// function.
pub trait FormatOptional<Context> {
    type Target: Format<Context>;

    /// This function tries to format an optional object. If the object is [None]
    /// an [empty token](crate::FormatElement::Empty) is created. If exists, the utility
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
    ///         write!(f, [token("MyToken")])
    ///     }
    /// }
    ///
    /// let none_token: Option<MyFormat> = None;
    /// // Returns `empty_element()` for a `None` value
    /// let none_formatted = format!(SimpleFormatContext::default(), [
    ///     none_token.with_or_empty(|token, f| write!(f, [token]))
    /// ]).unwrap();
    ///
    /// assert!(none_formatted.into_format_element().is_empty());
    ///
    /// let some_token = Some(MyFormat);
    /// assert_eq!(
    ///     format![SimpleFormatContext::default(), [space_token(), token("MyToken")]],
    ///     format!(
    ///         SimpleFormatContext::default(), [
    ///             some_token.with_or_empty(|token, f| {
    ///                 write!(f, [space_token(), token])
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
    ///         write!(f, [dynamic_token(&std::format!("Formatted {value} times."), TextSize::from(0))])
    ///     }
    /// }
    ///
    /// let normal = MyFormat::new();
    ///
    /// // Calls `format` for everytime the object gets formatted
    /// assert_eq!(
    ///     format!(SimpleFormatContext::default(), [token("Formatted 1 times."), token("Formatted 2 times.")]),
    ///     format!(SimpleFormatContext::default(), [normal, normal])
    /// );
    ///
    /// // Memoized memoizes the result and calls `format` only once.
    /// let memoized = normal.memoized();
    /// assert_eq!(
    ///     format!(SimpleFormatContext::default(), [token("Formatted 3 times."), token("Formatted 3 times.")]),
    ///     format![SimpleFormatContext::default(), [memoized, memoized]]
    /// );
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
    memory: RefCell<Option<FormatResult<Vec<FormatElement>>>>,
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
}

impl<F, Context> Format<Context> for Memoized<F, Context>
where
    F: Format<Context>,
{
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        // Cached
        if let Some(memory) = self.memory.borrow().as_ref() {
            return match memory {
                Ok(elements) => {
                    for element in elements {
                        f.write_element(element.clone())?;
                    }

                    Ok(())
                }
                Err(err) => Err(*err),
            };
        }
        let mut buffer = VecBuffer::new(f.state_mut());

        let result = write!(buffer, [self.inner]);

        match result {
            Ok(_) => {
                let elements = buffer.into_vec();
                for element in &elements {
                    f.write_element(element.clone())?;
                }

                *self.memory.borrow_mut() = Some(Ok(elements));

                Ok(())
            }
            Err(err) => {
                *self.memory.borrow_mut() = Some(Err(err));
                Err(err)
            }
        }
    }
}
