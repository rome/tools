use crate::prelude::*;
use crate::{ConcatBuilder, IntoFormatElement};

/// The macro `format_elements` is a convenience macro to
/// use when writing a list of tokens that should be at the same level
/// without particular rule.
///
/// # Examples
///
/// Let's suppose you need to create tokens for the string `"foo": "bar"`,
/// you would write:
///
/// ```rust
/// use rome_formatter::prelude::*;
///
/// let element = format_elements![token("foo:"), space_token(), token("bar")];
/// ```
///
/// The macro can be also nested, although the macro needs to be decorated with the token you need.
/// For example, let's try to format following string:
///
/// ```no_rust
/// foo: { bar: lorem }
/// ```
/// You would write it like the following:
///
/// ```rust
/// use rome_formatter::{FormatOptions, Formatted};
/// use rome_formatter::prelude::*;
///
/// let element = format_elements![
///   token("foo:"),
///   space_token(),
///   token("{"),
///   space_token(),
///   token("bar:"),
///   space_token(),
///   token("lorem"),
///   space_token(),
///   token("}")
/// ];
/// assert_eq!(r#"foo: { bar: lorem }"#, Formatted::new(element, FormatOptions::default()).print().as_code());
/// ```
/// Or you can also create single element:
/// ```
/// use rome_formatter::{Formatted, FormatOptions};
/// use rome_formatter::prelude::*;
///
/// use rome_formatter::prelude::*;
/// let element = format_elements![token("single")];
/// assert_eq!(r#"single"#, Formatted::new(element, FormatOptions::default()).print().as_code());
/// ```
#[macro_export]
macro_rules! format_elements {

    // called for things like format_tokens!["hey"]
    ($element:expr) => {
        {
            use $crate::FormatElement;
            FormatElement::from($element)
        }
    };

    ( $( $element:expr ),+ $(,)?) => {{
        use $crate::{FormatElement, concat_elements};
        concat_elements([
            $(
                     FormatElement::from($element)
            ),+
        ])
    }};
}

/// The macro `formatted` is a convenience macro to chain a list of [FormatElement] or objects
/// that implement [IntoFormatElement] (which is implemented by all object implementing [Format]).
///
/// # Examples
///
/// Let's suppose you need to create tokens for the string `"foo": "bar"`,
/// you would write:
///
/// ```rust
/// use rome_formatter::FormatOptions;
/// use rome_formatter::prelude::*;
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
///     &formatter,
///     [
///         token("a"),
///         space_token(),
///         token("simple"),
///         space_token(),
///         TestFormat
///     ]
///  ]
///  .unwrap();
///
///  assert_eq!(
///     formatted,
///     concat_elements([
///         token("a"),
///         space_token(),
///         token("simple"),
///         space_token(),
///         token("test")
///     ])
///  );
/// ```
///
/// Or you can also create single element:
/// ```
/// use rome_formatter::prelude::*;
/// use rome_formatter::FormatOptions;
///
/// let formatter = Formatter::new(FormatOptions::default());
///
/// let formatted = formatted![&formatter, [token("test")]].unwrap();
///
/// assert_eq!(formatted, token("test"));
/// ```
#[macro_export]
macro_rules! formatted {

    // called for things like formatted![formatter, [token("test")]]
    ($formatter:expr, [$element:expr]) => {
        {
            $crate::IntoFormatElement::into_format_element($element, $formatter)
        }
    };

    ($formatter:expr, [$($element:expr),+ $(,)?]) => {{
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
    use crate::prelude::*;
    use crate::FormatOptions;

    struct TestFormat;

    impl Format for TestFormat {
        fn format(&self, _: &Formatter) -> FormatResult<FormatElement> {
            Ok(token("test"))
        }
    }

    #[test]
    fn test_single_element() {
        let formatter = Formatter::new(FormatOptions::default());

        let formatted = formatted![&formatter, [TestFormat]].unwrap();

        assert_eq!(formatted, token("test"));
    }

    #[test]
    fn test_multiple_elements() {
        let formatter = Formatter::new(FormatOptions::default());

        let formatted = formatted![
            &formatter,
            [
                token("a"),
                space_token(),
                token("simple"),
                space_token(),
                TestFormat
            ]
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
