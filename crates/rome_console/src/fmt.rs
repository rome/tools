use std::{borrow::Cow, fmt, io, time::Duration};

use termcolor::{ColorSpec, WriteColor};

use crate::{markup, Markup, MarkupElement};

/// A stack-allocated linked-list of [MarkupElement] slices
enum MarkupElements<'a> {
    Root,
    Node(&'a Self, &'a [MarkupElement]),
}

impl<'a> MarkupElements<'a> {
    /// Iterates on all the element slices depth-first
    fn for_each(&self, func: &mut impl FnMut(&'a [MarkupElement])) {
        if let Self::Node(parent, elem) = self {
            parent.for_each(func);
            func(elem);
        }
    }
}

/// The [Formatter] is the `rome_console` equivalent to [std::fmt::Formatter]:
/// it's never constructed directly by consumers, and can only be used through
/// the mutable reference passed to implementations of the [Display] trait).
/// It manages the state of the markup to print, and implementations of
/// [Display] can call into its methods to append content into the current
/// printing session
pub struct Formatter<'fmt> {
    /// Stack of markup elements currently applied to the text being printed
    state: MarkupElements<'fmt>,
    /// Inner IO writer this [Formatter] will print text into
    writer: &'fmt mut dyn WriteColor,
}

impl<'fmt> Formatter<'fmt> {
    /// Create a new instance of the [Formatter] using the provided `writer` for printing
    pub(crate) fn new(writer: &'fmt mut dyn WriteColor) -> Self {
        Self {
            state: MarkupElements::Root,
            writer,
        }
    }

    /// Return a new instance of the [Formatter] with `elements` appended to its element stack
    fn with_elements<'b>(&'b mut self, elements: &'b [MarkupElement]) -> Formatter<'b> {
        Formatter {
            state: MarkupElements::Node(&self.state, elements),
            writer: self.writer,
        }
    }

    /// Write a piece of markup into this formatter
    pub fn write_markup(&mut self, markup: Markup) -> io::Result<()> {
        for node in markup.0 {
            let mut fmt = self.with_elements(node.elements);
            node.content.fmt(&mut fmt)?;
        }

        Ok(())
    }

    /// Applies the current format in `state` to `writer`, calls `func` to
    /// print a piece of text, then reset the printing format
    fn with_format(
        &mut self,
        func: impl FnOnce(&mut dyn WriteColor) -> io::Result<()>,
    ) -> io::Result<()> {
        let mut color = ColorSpec::new();
        self.state.for_each(&mut |elements| {
            for element in elements {
                element.update_color(&mut color);
            }
        });

        if let Err(err) = self.writer.set_color(&color) {
            self.writer.reset()?;
            return Err(err);
        }

        let result = func(self.writer);
        self.writer.reset()?;
        result
    }

    /// Write a slice of text into this formatter
    pub fn write_str(&mut self, content: &str) -> io::Result<()> {
        self.with_format(|writer| writer.write_all(content.as_bytes()))
    }

    /// Write formatted text into this formatter
    pub fn write_fmt(&mut self, content: fmt::Arguments) -> io::Result<()> {
        self.with_format(|writer| writer.write_fmt(content))
    }
}

/// Formatting trait for types to be displayed as markup, the `rome_console`
/// equivalent to [std::fmt::Display]
///
/// # Example
/// Implementing `Display` on a custom struct
/// ```
/// use std::io;
/// use rome_console::{fmt::{Display, Formatter}, markup};
///
/// struct Warning(String);
///
/// impl Display for Warning {
///     fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
///         fmt.write_markup(markup! {
///             <Warn>{self.0}</Warn>
///         })
///     }
/// }
///
/// let warning = Warning(String::from("content"));
/// markup! {
///     <Emphasis>{warning}</Emphasis>
/// };
/// ```
pub trait Display {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()>;
}

// Blanket implementations of Display for reference types
impl<'a, T> Display for &'a T
where
    T: Display + ?Sized,
{
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        T::fmt(self, fmt)
    }
}

impl<'a, T> Display for Cow<'a, T>
where
    T: Display + ToOwned + ?Sized,
{
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        T::fmt(self, fmt)
    }
}

// Simple implementations of Display calling through to write_str for types
// that implement Deref<str>
impl Display for str {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        fmt.write_str(self)
    }
}

impl Display for String {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        fmt.write_str(self)
    }
}

// Implement Display for Markup and Rust format Arguments
impl<'a> Display for Markup<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        fmt.write_markup(*self)
    }
}

impl<'a> Display for std::fmt::Arguments<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        fmt.write_fmt(*self)
    }
}

/// Implement [Display] for types that implement [std::fmt::Display] by calling
/// through to [Formatter::write_fmt]
macro_rules! impl_std_display {
    ($ty:ty) => {
        impl Display for $ty {
            fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
                write!(fmt, "{self}")
            }
        }
    };
}

impl_std_display!(char);
impl_std_display!(i8);
impl_std_display!(i16);
impl_std_display!(i32);
impl_std_display!(i64);
impl_std_display!(i128);
impl_std_display!(isize);
impl_std_display!(u8);
impl_std_display!(u16);
impl_std_display!(u32);
impl_std_display!(u64);
impl_std_display!(u128);
impl_std_display!(usize);

impl Display for Duration {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        use crate as rome_console;

        let secs = self.as_secs();
        if secs > 1 {
            return fmt.write_markup(markup! {
                {secs}<Dim>"s"</Dim>
            });
        }

        let millis = self.as_millis();
        if millis > 1 {
            return fmt.write_markup(markup! {
                {millis}<Dim>"ms"</Dim>
            });
        }

        let micros = self.as_micros();
        if micros > 1 {
            return fmt.write_markup(markup! {
                {micros}<Dim>"µs"</Dim>
            });
        }

        let nanos = self.as_nanos();
        fmt.write_markup(markup! {
            {nanos}<Dim>"ns"</Dim>
        })
    }
}
