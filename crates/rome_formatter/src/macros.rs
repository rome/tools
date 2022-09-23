/// Constructs the parameters for other formatting macros.
///
/// This macro functions by taking a list of objects implementing [crate::Format]. It canonicalize the
/// arguments into a single type.
///
/// This macro produces a value of type [crate::Arguments]. This value can be passed to
/// the macros within [crate]. All other formatting macros ([`format!`](crate::format!),
/// [`write!`](crate::write!)) are proxied through this one. This macro avoids heap allocations.
///
/// You can use the [`Arguments`] value that `format_args!` returns in  `Format` contexts
/// as seen below.
///
/// ```rust
/// use rome_formatter::{SimpleFormatContext, format, format_args};
/// use rome_formatter::prelude::*;
///
/// # fn main() -> FormatResult<()> {
/// let formatted = format!(SimpleFormatContext::default(), [
///     format_args!(text("Hello World"))
/// ])?;
///
/// assert_eq!("Hello World", formatted.print()?.as_code());
/// # Ok(())
/// # }
/// ```
///
/// [`Format`]: crate::Format
/// [`Arguments`]: crate::Arguments
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
/// generally this comes from an implementation of the [crate::Buffer] trait.
///
/// # Examples
///
/// ```rust
/// use rome_formatter::prelude::*;
/// use rome_formatter::{Buffer, FormatState, SimpleFormatContext, VecBuffer, write};
///
/// # fn main() -> FormatResult<()> {
/// let mut state = FormatState::new(SimpleFormatContext::default());
/// let mut buffer = VecBuffer::new(&mut state);
/// write!(&mut buffer, [text("Hello"), space()])?;
/// write!(&mut buffer, [text("World")])?;
///
/// assert_eq!(
///     buffer.into_vec(),
///     vec![
///         FormatElement::Text(Text::Static { text: "Hello" }),
///         FormatElement::Space,
///         FormatElement::Text(Text::Static { text: "World" }),
///     ]
///  );
/// #  Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! write {
    ($dst:expr, [$($arg:expr),+ $(,)?]) => {{
        let result = $dst.write_fmt($crate::format_args!($($arg),+));
        result
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
/// # fn main() -> FormatResult<()> {
/// let mut state = FormatState::new(SimpleFormatContext::default());
/// let mut buffer = VecBuffer::new(&mut state);
///
/// dbg_write!(buffer, [text("Hello")])?;
/// // ^-- prints: [src/main.rs:7][0] = StaticToken("Hello")
///
/// assert_eq!(buffer.into_vec(), vec![FormatElement::Text(Text::Static { text: "Hello" })]);
/// # Ok(())
/// # }
/// ```
///
/// Note that the macro is intended as debugging tool and therefore you should avoid having
/// uses of it in version control for long periods (other than in tests and similar). Format output
/// from production code is better done with `[write!]`
#[macro_export]
macro_rules! dbg_write {
    ($dst:expr, [$($arg:expr),+ $(,)?]) => {{
        use $crate::BufferExtensions;
        let mut count = 0;
        let mut inspect = $dst.inspect(|element: &FormatElement| {
            std::eprintln!(
                "[{}:{}][{}] = {element:#?}",
                std::file!(), std::line!(), count
            );
            count += 1;
        });
        let result = inspect.write_fmt($crate::format_args!($($arg),+));
        result
    }}
}

/// Creates the Format IR for a value.
///
/// The first argument `format!` receives is the [crate::FormatContext] that specify how elements must be formatted.
/// Additional parameters passed get formatted by using their [crate::Format] implementation.
///
///
/// ## Examples
///
/// ```
/// use rome_formatter::prelude::*;
/// use rome_formatter::format;
///
/// let formatted = format!(SimpleFormatContext::default(), [text("("), text("a"), text(")")]).unwrap();
///
/// assert_eq!(
///     formatted.into_document(),
///     Document::from(vec![
///         FormatElement::Text(Text::Static { text: "(" }),
///         FormatElement::Text(Text::Static { text: "a" }),
///         FormatElement::Text(Text::Static { text: ")" }),
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
/// use rome_formatter::{Formatted, LineWidth, format, format_args, SimpleFormatOptions};
/// use rome_formatter::prelude::*;
///
/// # fn main() -> FormatResult<()> {
/// let formatted = format!(
///     SimpleFormatContext::default(),
///     [
///         text("aVeryLongIdentifier"),
///         best_fitting!(
///             // Everything fits on a single line
///             format_args!(
///                 text("("),
///                 group(&format_args![
///                     text("["),
///                         soft_block_indent(&format_args![
///                         text("1,"),
///                         soft_line_break_or_space(),
///                         text("2,"),
///                         soft_line_break_or_space(),
///                         text("3"),
///                     ]),
///                     text("]")
///                 ]),
///                 text(")")
///             ),
///
///             // Breaks after `[`, but prints all elements on a single line
///             format_args!(
///                 text("("),
///                 text("["),
///                 block_indent(&text("1, 2, 3")),
///                 text("]"),
///                 text(")"),
///             ),
///
///             // Breaks after `[` and prints each element on a single line
///             format_args!(
///                 text("("),
///                 block_indent(&format_args![
///                     text("["),
///                     block_indent(&format_args![
///                         text("1,"),
///                         hard_line_break(),
///                         text("2,"),
///                         hard_line_break(),
///                         text("3"),
///                     ]),
///                     text("]"),
///                 ]),
///                 text(")")
///             )
///         )
///     ]
/// )?;
///
/// let document = formatted.into_document();
///
/// // Takes the first variant if everything fits on a single line
/// assert_eq!(
///     "aVeryLongIdentifier([1, 2, 3])",
///     Formatted::new(document.clone(), SimpleFormatContext::default())
///         .print()?
///         .as_code()
/// );
///
/// // It takes the second if the first variant doesn't fit on a single line. The second variant
/// // has some additional line breaks to make sure inner groups don't break
/// assert_eq!(
///     "aVeryLongIdentifier([\n\t1, 2, 3\n])",
///     Formatted::new(document.clone(), SimpleFormatContext::new(SimpleFormatOptions { line_width: 21.try_into().unwrap(), ..SimpleFormatOptions::default() }))
///         .print()?
///         .as_code()
/// );
///
/// // Prints the last option as last resort
/// assert_eq!(
///     "aVeryLongIdentifier(\n\t[\n\t\t1,\n\t\t2,\n\t\t3\n\t]\n)",
///     Formatted::new(document.clone(), SimpleFormatContext::new(SimpleFormatOptions { line_width: 20.try_into().unwrap(), ..SimpleFormatOptions::default() }))
///         .print()?
///         .as_code()
/// );
/// # Ok(())
/// # }
/// ```
///
/// ## Complexity
/// Be mindful of using this IR element as it has a considerable performance penalty:
/// * There are multiple representation for the same content. This results in increased memory usage
///   and traversal time in the printer.
/// * The worst case complexity is that the printer tires each variant. This can result in quadratic
///   complexity if used in nested structures.
///
/// ## Behavior
/// This IR is similar to Prettier's `conditionalGroup`. It provides similar functionality but
/// differs in that Prettier automatically wraps each variant in a `Group`. Rome doesn't do so.
/// You can wrap the variant content in a group if you want to use soft line breaks.
/// Unlike in Prettier, the printer will try to fit **only the first variant** in [`Flat`] mode,
/// then it will try to fit the rest of the variants in [`Expanded`] mode.
///
/// A variant that is measured in [`Expanded`] mode will be considered to fit if it can be printed without
/// overflowing the current line with all of its inner groups expanded. Those inner groups could still end
/// up being printed in flat mode if they fit on the line while printing. But there is currently no way
/// to enforce that a specific group inside a variant must be flat when measuring if that variant fits.
///
/// [`Flat`]: crate::format_element::PrintMode::Flat
/// [`Expanded`]: crate::format_element::PrintMode::Expanded
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
    use crate::{write, FormatState, SimpleFormatOptions, VecBuffer};

    struct TestFormat;

    impl Format<()> for TestFormat {
        fn fmt(&self, f: &mut Formatter<()>) -> FormatResult<()> {
            write!(f, [text("test")])
        }
    }

    #[test]
    fn test_single_element() {
        let mut state = FormatState::new(());
        let mut buffer = VecBuffer::new(&mut state);

        write![&mut buffer, [TestFormat]].unwrap();

        assert_eq!(
            buffer.into_vec(),
            vec![FormatElement::Text(Text::Static { text: "test" })]
        );
    }

    #[test]
    fn test_multiple_elements() {
        let mut state = FormatState::new(());
        let mut buffer = VecBuffer::new(&mut state);

        write![
            &mut buffer,
            [text("a"), space(), text("simple"), space(), TestFormat]
        ]
        .unwrap();

        assert_eq!(
            buffer.into_vec(),
            vec![
                FormatElement::Text(Text::Static { text: "a" }),
                FormatElement::Space,
                FormatElement::Text(Text::Static { text: "simple" }),
                FormatElement::Space,
                FormatElement::Text(Text::Static { text: "test" })
            ]
        );
    }

    #[test]
    fn best_fitting_variants_print_as_lists() {
        use crate::prelude::*;
        use crate::{format, format_args, Formatted};

        // The second variant below should be selected when printing at a width of 30
        let formatted_best_fitting = format!(
            SimpleFormatContext::default(),
            [
                text("aVeryLongIdentifier"),
                soft_line_break_or_space(),
                best_fitting!(
                    format_args!(text(
                        "Something that will not fit on a 30 character print width."
                    ),),
                    format_args![
                        text("Start"),
                        soft_line_break(),
                        &group(&soft_block_indent(&format_args![
                            text("1,"),
                            soft_line_break_or_space(),
                            text("2,"),
                            soft_line_break_or_space(),
                            text("3"),
                        ])),
                        soft_line_break_or_space(),
                        &soft_block_indent(&format_args![
                            text("1,"),
                            soft_line_break_or_space(),
                            text("2,"),
                            soft_line_break_or_space(),
                            group(&format_args!(
                                text("A,"),
                                soft_line_break_or_space(),
                                text("B")
                            )),
                            soft_line_break_or_space(),
                            text("3")
                        ]),
                        soft_line_break_or_space(),
                        text("End")
                    ],
                    format_args!(text("Most"), hard_line_break(), text("Expanded"))
                )
            ]
        )
        .unwrap();

        // This matches the IR above except that the `best_fitting` was replaced with
        // the contents of its second variant.
        let formatted_normal_list = format!(
            SimpleFormatContext::default(),
            [
                text("aVeryLongIdentifier"),
                soft_line_break_or_space(),
                format_args![
                    text("Start"),
                    soft_line_break(),
                    &group(&soft_block_indent(&format_args![
                        text("1,"),
                        soft_line_break_or_space(),
                        text("2,"),
                        soft_line_break_or_space(),
                        text("3"),
                    ])),
                    soft_line_break_or_space(),
                    &soft_block_indent(&format_args![
                        text("1,"),
                        soft_line_break_or_space(),
                        text("2,"),
                        soft_line_break_or_space(),
                        group(&format_args!(
                            text("A,"),
                            soft_line_break_or_space(),
                            text("B")
                        )),
                        soft_line_break_or_space(),
                        text("3")
                    ]),
                    soft_line_break_or_space(),
                    text("End")
                ],
            ]
        )
        .unwrap();

        let best_fitting_code = Formatted::new(
            formatted_best_fitting.into_document(),
            SimpleFormatContext::new(SimpleFormatOptions {
                line_width: 30.try_into().unwrap(),
                ..SimpleFormatOptions::default()
            }),
        )
        .print()
        .expect("Document to be valid")
        .as_code()
        .to_string();

        let normal_list_code = Formatted::new(
            formatted_normal_list.into_document(),
            SimpleFormatContext::new(SimpleFormatOptions {
                line_width: 30.try_into().unwrap(),
                ..SimpleFormatOptions::default()
            }),
        )
        .print()
        .expect("Document to be valid")
        .as_code()
        .to_string();

        // The variant that "fits" will print its contents as if it were a normal list
        // outside of a BestFitting element.
        assert_eq!(best_fitting_code, normal_list_code);
    }
}
