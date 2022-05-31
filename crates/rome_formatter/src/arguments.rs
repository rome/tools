use super::{Buffer, Format, Formatter};
use std::ffi::c_void;
use std::marker::PhantomData;

/// Stack allocated element that is pending for formatting
///
/// This struct is similar to dynamic dispatch (using `dyn Format`) by it stores a pointer to the value.
/// However, it doesn't store the pointer to `dyn Format`'s vtable, instead it statically resolves the function
/// pointer of `Format::format` and stores it in `formatter.
pub struct Argument<'fmt, O> {
    /// The value to format.
    value: *const c_void,
    /// The function pointer to `value`'s `Format::format` method
    formatter: fn(*const c_void, &mut Formatter<'_, O>) -> super::FormatResult<()>,
    lifetime: PhantomData<&'fmt ()>,
}

impl<'fmt, O> Argument<'fmt, O> {
    #[doc(hidden)]
    #[inline]
    pub fn new<F: Format<O>>(value: &'fmt F) -> Self {
        let formatter: fn(&F, &mut Formatter<'_, O>) -> super::FormatResult<()> = F::format;

        unsafe {
            Self {
                // SAFETY: `mem::transmute` is safe because
                // 1. `lifetime` keeps the lifetime it originated with `'fmt`
                // 2. `&'fmt F` and `*const c_void` have the same memory layout
                value: std::mem::transmute(value),
                // SAFETY: `mem::transmute` is safe because `fn(&F, &mut Formatter<'_>) -> Result`
                // and `fn(*const c_void, &mut Formatter<'_> -> Result` have the same ABI
                formatter: std::mem::transmute(formatter),

                lifetime: PhantomData,
            }
        }
    }

    /// Formats the value stored by this argument using the given formatter.
    pub(crate) fn format(&self, formatter: &mut Formatter<O>) -> super::FormatResult<()> {
        (self.formatter)(self.value, formatter)
    }
}

/// Stack allocated collection of items that should be formatted.
#[derive(Copy, Clone)]
pub struct Arguments<'fmt, O>(pub &'fmt [Argument<'fmt, O>]);

impl<'fmt, O> Arguments<'fmt, O> {
    #[doc(hidden)]
    #[inline]
    pub fn new(arguments: &'fmt [Argument<'fmt, O>]) -> Self {
        Self(arguments)
    }

    /// Returns the arguments
    pub(crate) fn items(&self) -> &'fmt [Argument<'fmt, O>] {
        self.0
    }
}

impl<O> Format<O> for Arguments<'_, O> {
    fn format(&self, formatter: &mut Formatter<O>) -> super::FormatResult<()> {
        formatter.write_fmt(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::{format_args, write, FormatState, VecBuffer};

    #[test]
    fn test_nesting() {
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
