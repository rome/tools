use super::{Buffer, Format, Formatter};
use crate::FormatResult;
use std::ffi::c_void;
use std::marker::PhantomData;

/// Mono-morphed type to format an object. Used by the [rome_formatter::format], [rome_formatter::format_args], and
/// [rome_formatter::write] macros.
///
/// This struct is similar to a dynamic dispatch (using `dyn Format`) because it stores a pointer to the value.
/// However, it doesn't store the pointer to `dyn Format`'s vtable, instead it statically resolves the function
/// pointer of `Format::format` and stores it in `formatter`.
#[derive(Copy, Clone)]
pub struct Argument<'fmt, O> {
    /// The value to format stored as a raw pointer where `lifetime` stores the value's lifetime.
    value: *const c_void,

    /// Stores the lifetime of the value. To get the most out of our dear borrow checker.
    lifetime: PhantomData<&'fmt ()>,

    /// The function pointer to `value`'s `Format::format` method
    formatter: fn(*const c_void, &mut Formatter<'_, O>) -> FormatResult<()>,
}

impl<'fmt, O> Argument<'fmt, O> {
    /// Called by the [rome_formatter::format_args] macro. Creates a mono-morphed value for formatting
    /// an object.
    #[doc(hidden)]
    #[inline]
    pub fn new<F: Format<O>>(value: &'fmt F) -> Self {
        fn formatter<F: Format<O>, O>(
            ptr: *const c_void,
            fmt: &mut Formatter<O>,
        ) -> FormatResult<()> {
            // SAFETY: Safe because the 'fmt lifetime is captured by the 'lifetime' field.
            F::format(unsafe { &*(ptr as *const F) }, fmt)
        }

        Self {
            value: value as *const F as *const c_void,
            lifetime: PhantomData,
            formatter: formatter::<F, O>,
        }
    }

    /// Formats the value stored by this argument using the given formatter.
    #[inline]
    pub(crate) fn format(&self, formatter: &mut Formatter<O>) -> super::FormatResult<()> {
        (self.formatter)(self.value, formatter)
    }
}

/// Sequence of objects that should be formatted in the specified order.
///
/// The [`format_args!`] macro will safely create an instance of this structure.
///
/// You can use the `Arguments<a>` that [`format_args!]` return in `Format` context as seen below.
/// It will call the `format` function for every of it's objects.
///
/// ```rust
/// use rome_formatter::prelude::*;
/// use rome_formatter::{format, format_args};
///
/// let formatted = format!(SimpleFormatContext::default(), [
///     format_args!(token("a"), space_token(), token("b"))
/// ]).unwrap();
///
/// assert_eq!("a b", formatted.print().as_code());
/// ```
pub struct Arguments<'fmt, O>(pub &'fmt [Argument<'fmt, O>]);

impl<'fmt, O> Arguments<'fmt, O> {
    #[doc(hidden)]
    #[inline]
    pub fn new(arguments: &'fmt [Argument<'fmt, O>]) -> Self {
        Self(arguments)
    }

    /// Returns the arguments
    #[inline]
    pub(super) fn items(&self) -> &'fmt [Argument<'fmt, O>] {
        self.0
    }
}

impl<O> Copy for Arguments<'_, O> {}

impl<O> Clone for Arguments<'_, O> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<O> Format<O> for Arguments<'_, O> {
    #[inline]
    fn format(&self, formatter: &mut Formatter<O>) -> FormatResult<()> {
        formatter.write_fmt(*self)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::{format_args, write, FormatState, VecBuffer};

    #[test]
    fn test_nesting() {
        std::format_args!("test {}", "a");
        // Format_arguments not very useful, but I guess the same as normal format_args

        let mut state = FormatState::new(());
        let mut buffer = VecBuffer::new(&mut state);

        write!(
            &mut buffer,
            [
                token("function"),
                space_token(),
                token("a"),
                space_token(),
                group_elements(&format_args!(token("("), token(")")))
            ]
        )
        .unwrap();

        assert_eq!(
            buffer.into_element(),
            FormatElement::List(List::new(vec![
                FormatElement::Token(Token::Static { text: "function" }),
                FormatElement::Space,
                FormatElement::Token(Token::Static { text: "a" }),
                FormatElement::Space,
                FormatElement::Group(Group::new(FormatElement::List(List::new(vec![
                    FormatElement::Token(Token::Static { text: "(" }),
                    FormatElement::Token(Token::Static { text: ")" }),
                ]))))
            ]))
        );
    }
}
