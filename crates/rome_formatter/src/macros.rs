/// Constructs the parameters for other formatting macros.
///
/// This macro functions by taking a list of objects implementing [Format]. It canonicalize the
/// arguments into a single type.
///
/// This macro produces a value of type [`Arguments`]. This value can be passed to
/// the macros within [`rome_formatter`]. All other formatting macros ([`format!`],
/// [`write!`]) are proxied through this one. This macro avoids heap allocations.
///
/// You can use the [`Arguments`] value that `format_args!` returns in  `Format` contexts
/// as seen below.
///
/// ```rust
/// use rome_formatter::{SimpleFormatContext, format, format_args};
/// use rome_formatter::prelude::*;
///
/// let formatted = format!(SimpleFormatContext::default(), [
///     format_args!(token("Hello World"))
/// ]).unwrap();
///
/// assert_eq!("Hello World", formatted.print().as_code());
/// ```
///
/// [`Format`]: crate::Format
/// [`Arguments`]: crate::Arguments
/// [`format!`]: crate::format
/// [`write!`]: crate::write
#[macro_export]
macro_rules! format_args {
    ($($value:expr),+ $(,)?) => {
        $crate::Arguments::new(&[
            $(
                $crate::Argument::new(&$value)
            ),+
        ])
    }
}

/// Writes formatted data into a buffer.
///
/// This macro accepts a 'buffer' and a list of format arguments. Each argument will be formatted
/// and the result will be passed to the buffer. The writer may be any value with a `write_fmt` method;
/// generally this comes from an implementation of the [`Buffer`] trait.
///
/// # Examples
///
/// ```rust
/// use rome_formatter::prelude::*;
/// use rome_formatter::{Buffer, FormatState, SimpleFormatContext, VecBuffer, write};
///
/// fn main() -> FormatResult<()> {
///     let mut state = FormatState::new(SimpleFormatContext::default());
///     let mut buffer = VecBuffer::new(&mut state);
///     write!(&mut buffer, [token("Hello"), space_token()])?;
///     write!(&mut buffer, [token("World")])?;
///
///     assert_eq!(
///         buffer.into_element(),
///         FormatElement::from_iter([
///             FormatElement::Token(Token::Static { text: "Hello" }),
///             FormatElement::Space,
///             FormatElement::Token(Token::Static { text: "World" }),
///         ])
///     );
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! write {
    ($dst:expr, [$($arg:expr),+ $(,)?]) => {{
        $dst.write_fmt($crate::format_args!($($arg),+))
    }}
}

/// Writes formatted data into the given buffer and prints all written elements for a quick and dirty debugging.
///
/// An example:
///
/// ```rust
/// use rome_formatter::prelude::*;
/// use rome_formatter::{FormatState, VecBuffer};
///
/// let mut state = FormatState::new(SimpleFormatContext::default());
/// let mut buffer = VecBuffer::new(&mut state);
///
/// dbg_write!(&mut buffer, [token("Hello")]).unwrap();
/// // ^-- prints: [src/main.rs:7][0] = StaticToken("Hello")
///
/// assert_eq!(buffer.into_element(), FormatElement::Token(Token::Static { text: "Hello" }));
/// ```
///
/// Note that the macro is intended as debugging tool and therefore you should avoid having
/// uses of it in version control for long periods (other than in tests and similar). Format output
/// from production code is better done with `[write!]`
#[macro_export]
macro_rules! dbg_write {
    ($dst:expr, [$($arg:expr),+ $(,)?]) => {{
        let mut count = 0;
        let mut inspect = $crate::Inspect::new($dst, |element: &FormatElement| {
            std::eprintln!(
                "[{}:{}][{}] = {element:#?}",
                std::file!(), std::line!(), count
            );
            count += 1;
        });
        inspect.write_fmt($crate::format_args!($($arg),+))
    }}
}

/// Creates the Format IR for a value.
///
/// The first argument `format!` receives is the [FormatContext] that specify how elements must be formatted.
/// Additional parameters passed get formatted by using their [Format] implementation.
///
///
/// ## Examples
///
/// ```
/// use rome_formatter::prelude::*;
/// use rome_formatter::format;
///
/// let formatted = format!(SimpleFormatContext::default(), [token("("), token("a"), token(")")]).unwrap();
///
/// assert_eq!(
///     formatted.into_format_element(),
///     FormatElement::from_iter([
///         FormatElement::Token(Token::Static { text: "(" }),
///         FormatElement::Token(Token::Static { text: "a" }),
///         FormatElement::Token(Token::Static { text: ")" }),
///     ])
/// );
/// ```
#[macro_export]
macro_rules! format {
    ($context:expr, [$($arg:expr),+ $(,)?]) => {{
        ($crate::format($context, $crate::format_args!($($arg),+)))
    }}
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
/// use rome_formatter::{Formatted, LineWidth, format, format_args};
/// use rome_formatter::prelude::*;
///
/// let formatted = format!(
///     SimpleFormatContext::default(),
///     [
///         token("aVeryLongIdentifier"),
///         best_fitting!(
///             // Everything fits on a single line
///             format_args!(
///                 token("("),
///                 group_elements(&format_args![
///                     token("["),
///                         soft_block_indent(&format_args![
///                         token("1,"),
///                         soft_line_break_or_space(),
///                         token("2,"),
///                         soft_line_break_or_space(),
///                         token("3"),
///                     ]),
///                     token("]")
///                 ]),
///                 token(")")
///             ),
///
///             // Breaks after `[`, but prints all elements on a single line
///             format_args!(
///                 token("("),
///                 token("["),
///                 block_indent(&token("1, 2, 3")),
///                 token("]"),
///                 token(")"),
///             ),
///
///             // Breaks after `[` and prints each element on a single line
///             format_args!(
///                 token("("),
///                 block_indent(&format_args![
///                     token("["),
///                     block_indent(&format_args![
///                         token("1,"),
///                         hard_line_break(),
///                         token("2,"),
///                         hard_line_break(),
///                         token("3"),
///                     ]),
///                     token("]"),
///                 ]),
///                 token(")")
///             )
///         )
///     ]
/// ).unwrap();
///
/// let elements = formatted.into_format_element();
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
        unsafe {
            $crate::BestFitting::from_arguments_unchecked($crate::format_args!($least_expanded, $($tail),+))
        }
    }}
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::{write, FormatState, VecBuffer};

    struct TestFormat;

    impl Format<()> for TestFormat {
        fn fmt(&self, f: &mut Formatter<()>) -> FormatResult<()> {
            write!(f, [token("test")])
        }
    }

    #[test]
    fn test_single_element() {
        let mut state = FormatState::new(());
        let mut buffer = VecBuffer::new(&mut state);

        write![&mut buffer, [TestFormat]].unwrap();

        assert_eq!(
            buffer.into_element(),
            FormatElement::Token(Token::Static { text: "test" })
        );
    }

    #[test]
    fn test_multiple_elements() {
        let mut state = FormatState::new(());
        let mut buffer = VecBuffer::new(&mut state);

        write![
            &mut buffer,
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
            buffer.into_element(),
            FormatElement::List(List::new(vec![
                FormatElement::Token(Token::Static { text: "a" }),
                FormatElement::Space,
                FormatElement::Token(Token::Static { text: "simple" }),
                FormatElement::Space,
                FormatElement::Token(Token::Static { text: "test" })
            ]))
        );
    }
}
