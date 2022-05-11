use crate::{Formatter, IntoFormatElement};
use rome_formatter::{ConcatBuilder, FormatElement, FormatError, FormatResult};

/// The macro `format` is a convenience macro to chain a list of [FormatElement] or objects
/// that implement [IntoFormatElement] (which is implemented by all object implementing [Format]).
///
/// # Examples
///
/// Let's suppose you need to create tokens for the string `"foo": "bar"`,
/// you would write:
///
/// ```rust
/// use rome_formatter::{concat_elements, format_elements, FormatElement, FormatOptions, FormatResult, space_token, token};
/// use rome_js_formatter::{Format, formatted, Formatter};
///
/// struct TestFormat;
///
/// impl Format for TestFormat {
///     fn format(&self, _: &Formatter) -> FormatResult<FormatElement> {
///         Ok(token("test"))
///     }
/// }
///
/// let formatter = Formatter::new(FormatOptions::default());
///
/// let formatted = formatted![
///         &formatter,
///         token("a"),
///         space_token(),
///         token("simple"),
///         space_token(),
///         TestFormat
///     ]
///     .unwrap();
///
///     assert_eq!(
///         formatted,
///         concat_elements([
///             token("a"),
///             space_token(),
///             token("simple"),
///             space_token(),
///             token("test")
///         ])
///  );
/// ```
///
/// Or you can also create single element:
/// ```
/// use rome_formatter::{FormatOptions, token};
/// use rome_js_formatter::{formatted, Formatter};
///
/// let formatter = Formatter::new(FormatOptions::default());
///
/// let formatted = formatted![&formatter, token("test")].unwrap();
///
/// assert_eq!(formatted, token("test"));
/// ```
#[macro_export]
macro_rules! formatted {

    // called for things like formatted![formatter, token("test")]
    ($formatter:expr, $element:expr) => {
        {
            $crate::IntoFormatElement::into_format_element($element, $formatter)
        }
    };

    ($formatter:expr, $($element:expr),+ $(,)?) => {{
        use $crate::macros::FormatBuilder;

        const SIZE: usize = $crate::__count_elements!($($element),*);

        let mut builder = FormatBuilder::new(SIZE);

        $(
                     builder.entry($element, $formatter);
        )+

        builder.finish()
    }};
}

// Helper macro that counts the count of elements passed
#[doc(hidden)]
#[macro_export]
macro_rules! __count_elements {
    () => {0usize};
    ($ex:expr) => {1usize};
    ($_head:expr, $($tail:expr),* $(,)?) => {1usize + $crate::__count_elements!($($tail),*)};
}

#[doc(hidden)]
pub struct FormatBuilder {
    builder: ConcatBuilder,
    result: Result<(), FormatError>,
}

impl FormatBuilder {
    #[inline]
    pub fn new(size: usize) -> Self {
        let mut builder = ConcatBuilder::new();
        builder.size_hint((size, Some(size)));

        Self {
            builder,
            result: Ok(()),
        }
    }

    #[inline]
    pub fn entry<T>(&mut self, element: T, formatter: &Formatter)
    where
        T: IntoFormatElement,
    {
        self.result = self.result.and_then(|_| {
            self.builder.entry(element.into_format_element(formatter)?);
            Ok(())
        });
    }

    #[inline]
    pub fn finish(self) -> FormatResult<FormatElement> {
        self.result.map(|_| self.builder.finish())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Format, Formatter};
    use rome_formatter::{
        concat_elements, space_token, token, FormatElement, FormatOptions, FormatResult,
    };

    struct TestFormat;

    impl Format for TestFormat {
        fn format(&self, _: &Formatter) -> FormatResult<FormatElement> {
            Ok(token("test"))
        }
    }

    #[test]
    fn test_single_element() {
        let formatter = Formatter::new(FormatOptions::default());

        let formatted = formatted![&formatter, TestFormat].unwrap();

        assert_eq!(formatted, token("test"));
    }

    #[test]
    fn test_multiple_elements() {
        let formatter = Formatter::new(FormatOptions::default());

        let formatted = formatted![
            &formatter,
            token("a"),
            space_token(),
            token("simple"),
            space_token(),
            TestFormat
        ]
        .unwrap();

        assert_eq!(
            formatted,
            concat_elements([
                token("a"),
                space_token(),
                token("simple"),
                space_token(),
                token("test")
            ])
        );
    }
}
