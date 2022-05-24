use crate::prelude::*;
use crate::{ConcatBuilder, IntoFormatElement};
use std::marker::PhantomData;

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
/// assert_eq!(r#"foo: { bar: lorem }"#, Formatted::new(element, PrinterOptions::default()).print().as_code());
/// ```
/// Or you can also create single element:
/// ```
/// use rome_formatter::{Formatted, FormatOptions};
/// use rome_formatter::prelude::*;
///
/// use rome_formatter::prelude::*;
/// let element = format_elements![token("single")];
/// assert_eq!(r#"single"#, Formatted::new(element, PrinterOptions::default()).print().as_code());
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
///     type Options = ();
///     fn format(&self, _: &Formatter<Self::Options>) -> FormatResult<FormatElement> {
///         Ok(token("test"))
///     }
/// }
///
/// let formatter = Formatter::default();
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
/// let formatter = Formatter::<()>::default();
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

/// Provides multiple different alternatives and the printer picks the first one that fits.
/// Use this as last resort because it requires that the printer must try all variants in the worst case.
/// The passed variants must be in the following order:
/// * First: The variant that takes up most space horizontally
/// * Last: The variant that takes up the least space horizontally by splitting the content over multiple lines.
///
/// ## Examples
///
/// ```
/// use rome_formatter::{Formatted, LineWidth};
/// use rome_formatter::prelude::*;
///
/// let elements = format_elements![
///   token("aVeryLongIdentifier"),
///   best_fitting!(
///     // Everything fits on a single line
///     format_elements![
///         token("("),
///         group_elements(format_elements![
///             token("["),
///                 soft_block_indent(format_elements![
///                 token("1,"),
///                 soft_line_break_or_space(),
///                 token("2,"),
///                 soft_line_break_or_space(),
///                 token("3"),
///             ]),
///             token("]")
///         ]),
///         token(")")
///     ],
///
///     // Breaks after `[`, but prints all elements on a single line
///     format_elements![
///         token("("),
///         token("["),
///         block_indent(token("1, 2, 3")),
///         token("]"),
///         token(")"),
///     ],
///
///     // Breaks after `[` and prints each element on a single line
///     format_elements![
///         token("("),
///         block_indent(format_elements![
///             token("["),
///             block_indent(format_elements![
///                 token("1,"),
///                 hard_line_break(),
///                 token("2,"),
///                 hard_line_break(),
///                 token("3"),
///             ]),
///             token("]"),
///         ]),
///         token(")")
///     ]
///   )
/// ];
///
/// // Takes the first variant if everything fits on a single line
/// assert_eq!(
///     "aVeryLongIdentifier([1, 2, 3])",
///     Formatted::new(elements.clone(), PrinterOptions::default())
///         .print()
///         .as_code()
/// );
///
/// // It takes the second if the first variant doesn't fit on a single line. The second variant
/// // has some additional line breaks to make sure inner groups don't break
/// assert_eq!(
///     "aVeryLongIdentifier([\n\t1, 2, 3\n])",
///     Formatted::new(elements.clone(), PrinterOptions::default().with_print_width(21.try_into().unwrap()))
///         .print()
///         .as_code()
/// );
///
/// // Prints the last option as last resort
/// assert_eq!(
///     "aVeryLongIdentifier(\n\t[\n\t\t1,\n\t\t2,\n\t\t3\n\t]\n)",
///     Formatted::new(elements.clone(), PrinterOptions::default().with_print_width(20.try_into().unwrap()))
///         .print()
///         .as_code()
/// );
/// ```
///
/// ## Complexity
/// Be mindful of using this IR element as it has a considerable performance penalty:
/// * There are multiple representation for the same content. This results in increased memory usage
///   and traversal time in the printer.
/// * The worst case complexity is that the printer tires each variant. This can result in quadratic
///   complexity if used in nested structures.
///
/// ## Prettier
/// This IR is similar to Prettier's `ConditionalGroupContent` IR. It provides the same functionality but
/// differs in that Prettier automatically wraps each variant in a `Group`. Rome doesn't do so.
/// You can wrap the variant content in a group if you want to use soft line breaks.
#[macro_export]
macro_rules! best_fitting {
    ($least_expanded:expr, $($tail:expr),+ $(,)?) => {{
        let inner = unsafe {
            $crate::format_element::BestFitting::from_slice_unchecked(&[$least_expanded, $($tail),+])
        };
        FormatElement::BestFitting(inner)
    }}
}

#[doc(hidden)]
pub struct FormatBuilder<O> {
    builder: ConcatBuilder,
    result: Result<(), FormatError>,
    options: PhantomData<O>,
}

impl<O> FormatBuilder<O> {
    #[inline]
    pub fn new(size: usize) -> Self {
        let mut builder = ConcatBuilder::new();
        builder.size_hint((size, Some(size)));

        Self {
            builder,
            result: Ok(()),
            options: PhantomData,
        }
    }

    #[inline]
    pub fn entry<T>(&mut self, element: T, formatter: &Formatter<O>)
    where
        T: IntoFormatElement<O>,
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

    struct TestFormat;

    impl Format for TestFormat {
        type Options = ();
        fn format(&self, _: &Formatter<Self::Options>) -> FormatResult<FormatElement> {
            Ok(token("test"))
        }
    }

    #[test]
    fn test_single_element() {
        let formatter = Formatter::new(());

        let formatted = formatted![&formatter, [TestFormat]].unwrap();

        assert_eq!(formatted, token("test"));
    }

    #[test]
    fn test_multiple_elements() {
        let formatter = Formatter::new(());

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
