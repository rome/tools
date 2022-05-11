use crate::prelude::*;
use std::cell::RefCell;

use crate::IntoFormatElement;
use rome_rowan::SyntaxResult;

/// Utility trait used to simplify the formatting of optional objects that are formattable.
///
/// In order to take advantage of all the functions, you only need to implement the [FormatOptionalTokenAndNode::with_or]
/// function.
pub trait FormatOptional {
    /// This function tries to format an optional object. If the object is [None]
    /// an [empty token](crate::FormatElement::Empty) is created. If exists, the utility
    /// formats the object and passes it to the closure.
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_formatter::prelude::*;
    /// use rome_rowan::TextSize;
    ///
    /// struct MyFormat;
    ///
    /// impl Format for MyFormat {
    /// fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
    ///         Ok(token("MyToken"))
    ///     }
    /// }
    ///
    /// let formatter = Formatter::default();
    ///
    /// let none_token: Option<MyFormat> = None;
    /// // Returns `empty_element()` for a `None` value
    /// let none_result = none_token.with_or_empty(|token| token);
    /// assert_eq!(Ok(empty_element()), formatted![&formatter, [none_result]]);
    ///
    /// let some_token = Some(MyFormat);
    /// let some_result = some_token.with_or_empty(|token| {
    ///     formatted![&formatter, [space_token(), token]]
    /// });
    /// assert_eq!(formatted![&formatter, [space_token(), token("MyToken")]], formatted![&formatter, [some_result]]);
    fn with_or_empty<With, WithResult>(
        &self,
        with: With,
    ) -> FormatWithOr<With, fn() -> FormatElement, WithResult, FormatElement>
    where
        With: Fn(FormatElement) -> WithResult,
        WithResult: IntoFormatElement,
    {
        self.with_or(with, empty_element)
    }

    /// This function tries to format an optional formattable object as is. If the object is [None],
    /// it calls the passed closure, which has to return a [crate::FormatElement]
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_formatter::prelude::*;
    /// use rome_rowan::TextSize;
    ///
    /// struct MyFormat;
    ///
    /// impl Format for MyFormat {
    /// fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
    ///         Ok(token("MyToken"))
    ///     }
    /// }
    ///
    /// let formatter = Formatter::default();
    /// let none_token: Option<MyFormat> = None;
    /// let result = none_token.or_format(|| token(" other result"));
    ///
    /// assert_eq!(Ok(token(" other result")), formatted![&formatter, [result]]);
    fn or_format<Or, OrResult>(
        &self,
        op: Or,
    ) -> FormatWithOr<fn(FormatElement) -> FormatElement, Or, FormatElement, OrResult>
    where
        Or: Fn() -> OrResult,
        OrResult: IntoFormatElement,
        Self: Sized,
    {
        self.with_or(|token| token, op)
    }

    /// If the object isn't [None], it will call the first closure which will accept formatted element.
    ///
    /// If the object is [None], the second closure will be called.
    ///
    /// Both closures have to return a [crate::FormatElement]. This function will make sure to wrap them into [Ok].
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_formatter::prelude::*;
    /// use rome_rowan::TextSize;
    ///
    /// struct MyFormat;
    ///
    /// impl Format for MyFormat {
    /// fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
    ///         Ok(token("MyToken"))
    ///     }
    /// }
    ///
    /// let formatter = Formatter::default();
    /// let none_token: Option<MyFormat> = None;
    ///
    /// // It returns the `or` result if called on `None`
    /// let none_result = none_token.with_or(|token| token, || {
    ///     token("empty")
    /// });
    /// assert_eq!(Ok(token("empty")), formatted![&formatter, [none_result]]);
    ///
    /// // Returns the result of the first callback when called with `Some(value)`
    /// let some_result = Some(MyFormat).with_or(|token| {
    ///     formatted![&formatter, [space_token(), token]]
    /// }, || empty_element());
    ///
    /// assert_eq!(formatted![&formatter, [space_token(), token("MyToken")]], formatted![&formatter, [some_result]]);
    fn with_or<With, Or, WithResult, OrResult>(
        &self,
        with: With,
        op: Or,
    ) -> FormatWithOr<With, Or, WithResult, OrResult>
    where
        With: Fn(FormatElement) -> WithResult,
        WithResult: IntoFormatElement,
        Or: Fn() -> OrResult,
        OrResult: IntoFormatElement;
}

/// Utility trait for formatting a formattable object with some additional content.
pub trait FormatWith {
    /// Allows to chain a formattable object with another [elements](FormatElement)
    ///
    /// The function will decorate the result with [Ok]
    ///
    /// The formatted element is passed to the closure, which then can appended to additional elements.
    /// This method is useful in case, for example, a token has to be chained with a space.
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_formatter::prelude::*;
    /// use rome_rowan::TextSize;
    ///
    /// struct MyFormat;
    ///
    /// impl Format for MyFormat {
    /// fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
    ///         Ok(token("MyToken"))
    ///     }
    /// }
    ///
    /// let formatter = Formatter::default();
    ///
    /// let result = MyFormat.with(|string_literal| {
    ///     formatted![&formatter, [string_literal, space_token(), token("+")]]
    /// });
    ///
    /// assert_eq!(formatted![&formatter, [token("MyToken"), space_token(), token("+")]], formatted![&formatter, [result]])
    fn with<With, WithResult>(&self, with: With) -> FormatItemWith<With, WithResult>
    where
        With: Fn(FormatElement) -> WithResult,
        WithResult: IntoFormatElement;
}

pub struct FormatItemWith<'a, With, WithResult>
where
    With: Fn(FormatElement) -> WithResult,
    WithResult: IntoFormatElement,
{
    with: With,
    inner: &'a dyn Format,
}

impl<'a, With, WithResult> Format for FormatItemWith<'a, With, WithResult>
where
    With: Fn(FormatElement) -> WithResult,
    WithResult: IntoFormatElement,
{
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let element = self.inner.format(formatter)?;

        (self.with)(element).into_format_element(formatter)
    }
}

impl<F: Format> FormatWith for F {
    fn with<With, WithResult>(&self, with: With) -> FormatItemWith<With, WithResult>
    where
        With: Fn(FormatElement) -> WithResult,
        WithResult: IntoFormatElement,
    {
        FormatItemWith { with, inner: self }
    }
}

impl<F: Format> FormatOptional for SyntaxResult<Option<F>> {
    fn with_or<With, Or, WithResult, OrResult>(
        &self,
        with: With,
        op: Or,
    ) -> FormatWithOr<With, Or, WithResult, OrResult>
    where
        With: Fn(FormatElement) -> WithResult,
        WithResult: IntoFormatElement,
        Or: Fn() -> OrResult,
        OrResult: IntoFormatElement,
    {
        match self {
            Err(_) => FormatWithOr::With { inner: self, with },
            Ok(Some(value)) => FormatWithOr::With { inner: value, with },
            Ok(None) => FormatWithOr::Or(op),
        }
    }
}

impl<F: Format> FormatOptional for Option<F> {
    fn with_or<With, Or, WithResult, OrResult>(
        &self,
        with: With,
        op: Or,
    ) -> FormatWithOr<With, Or, WithResult, OrResult>
    where
        With: Fn(FormatElement) -> WithResult,
        WithResult: IntoFormatElement,
        Or: Fn() -> OrResult,
        OrResult: IntoFormatElement,
    {
        match self {
            None => FormatWithOr::Or(op),
            Some(value) => FormatWithOr::With { inner: value, with },
        }
    }
}

pub enum FormatWithOr<'a, With, Or, WithResult, OrResult>
where
    With: Fn(FormatElement) -> WithResult,
    Or: Fn() -> OrResult,
    WithResult: IntoFormatElement,
    OrResult: IntoFormatElement,
{
    With { inner: &'a dyn Format, with: With },
    Or(Or),
}

impl<'a, With, Or, WithResult, OrResult> Format for FormatWithOr<'a, With, Or, WithResult, OrResult>
where
    With: Fn(FormatElement) -> WithResult,
    Or: Fn() -> OrResult,
    WithResult: IntoFormatElement,
    OrResult: IntoFormatElement,
{
    #[inline]
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            FormatWithOr::Or(op) => op().into_format_element(formatter),
            FormatWithOr::With { inner, with } => {
                with(inner.format(formatter)?).into_format_element(formatter)
            }
        }
    }
}

/// Utility trait that allows memorizing the output of a [Format].
/// Useful to avoid re-formatting the same object twice.
pub trait MemoizeFormat {
    /// Returns a formattable object that memoizes the result of `Format` by cloning.
    /// Mainly useful if the same sub-tree can appear twice in the formatted output because it's
    /// used inside of `if_group_breaks` or `if_group_fits_single_line`.
    ///
    /// ```
    /// use std::cell::Cell;
    /// use rome_formatter::FormatOptions;
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
    /// impl Format for MyFormat {fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
    ///         let value = self.value.get();
    ///         self.value.set(value + 1);
    ///
    ///         Ok(FormatElement::from(Token::new_dynamic(format!("Formatted {value} times."), TextSize::from(0))))
    ///     }
    /// }
    ///
    /// let formatter = Formatter::new(FormatOptions::default());
    /// let normal = MyFormat::new();
    ///
    /// // Calls `format` for everytime the object gets formatted
    /// assert_eq!(
    ///     Ok(format_elements![token("Formatted 1 times."), token("Formatted 2 times.")]),
    ///     formatted![&formatter, [&normal, &normal]]
    /// );
    ///
    /// // Memoized memoizes the result and calls `format` only once.
    /// let memoized = normal.memoized();
    /// assert_eq!(
    ///     Ok(format_elements![token("Formatted 3 times."), token("Formatted 3 times.")]),
    ///     formatted![&formatter, [&memoized, &memoized]]
    /// );
    /// ```
    ///
    fn memoized(self) -> Memoized<Self>
    where
        Self: Sized + Format,
    {
        Memoized::new(self)
    }
}

impl<F> MemoizeFormat for F where F: Format {}

/// Memoizes the output of its inner [Format] to avoid re-formatting a potential expensive object.
pub struct Memoized<F> {
    inner: F,
    memory: RefCell<Option<FormatResult<FormatElement>>>,
}

impl<F: Format> Memoized<F> {
    fn new(inner: F) -> Self {
        Self {
            inner,
            memory: RefCell::new(None),
        }
    }
}

impl<F> Format for Memoized<F>
where
    F: Format,
{
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        if let Some(memory) = self.memory.borrow().as_ref() {
            return memory.clone();
        }

        let formatted = self.inner.format(formatter);
        *self.memory.borrow_mut() = Some(formatted.clone());

        formatted
    }
}
