use crate::prelude::*;
use crate::utils::jsx::{
    is_meaningful_jsx_text, is_whitespace_jsx_expression, jsx_split_children, JsxChild,
    JsxRawSpace, JsxSpace,
};
use crate::JsFormatter;
use rome_formatter::{
    format_args, write, Comments, CstFormatContext, FormatRuleWithOptions, VecBuffer,
};
use rome_js_syntax::{JsLanguage, JsxAnyChild, JsxChildList};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxChildList {
    layout: JsxChildListLayout,
}

impl FormatRuleWithOptions<JsxChildList> for FormatJsxChildList {
    type Options = JsxChildListLayout;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.layout = options;
        self
    }
}

impl FormatRule<JsxChildList> for FormatJsxChildList {
    type Context = JsFormatContext;

    fn fmt(&self, list: &JsxChildList, f: &mut JsFormatter) -> FormatResult<()> {
        self.disarm_debug_assertions(list, f);

        let children_meta = self.children_meta(list, f.context().comments());
        let layout = self.layout(children_meta);

        let multiline_layout = if children_meta.meaningful_text {
            MultilineLayout::Fill
        } else {
            MultilineLayout::NoFill
        };

        let mut flat = FlatBuilder::new();
        let mut multiline = MultilineBuilder::new(multiline_layout);

        let mut last: Option<JsxChild> = None;
        let mut force_multiline = layout.is_multiline();

        let mut children = jsx_split_children(list, f.context().comments())?;

        // Trim trailing new lines
        if let Some(JsxChild::EmptyLine | JsxChild::Newline) = children.last() {
            children.pop();
        }

        let mut children_iter = children.into_iter().peekable();

        // Trim leading new lines
        if let Some(JsxChild::Newline | JsxChild::EmptyLine) = children_iter.peek() {
            children_iter.next();
        }

        while let Some(child) = children_iter.next() {
            let mut child_breaks = false;

            match &child {
                // A single word: Both `a` and `b` are a word in `a b` because they're separated by JSX Whitespace.
                JsxChild::Word(word) => {
                    let separator = match children_iter.peek() {
                        Some(JsxChild::Word(_)) => {
                            // Separate words by a space or line break in extended mode
                            Some(WordSeparator::BetweenWords)
                        }

                        // Last word or last word before an element without any whitespace in between
                        Some(JsxChild::NonText(child)) => Some(WordSeparator::EndOfText {
                            is_next_self_closing: matches!(
                                child,
                                JsxAnyChild::JsxSelfClosingElement(_)
                            ),
                        }),

                        _ => None,
                    };

                    child_breaks = separator.map_or(false, |separator| separator.will_break());

                    flat.write(&format_args![word, separator], f);

                    if let Some(separator) = separator {
                        multiline.write(word, &separator, f);
                    } else {
                        multiline.write_with_empty_separator(word, f);
                    }
                }

                // * Whitespace after the opening tag and before a meaningful text: `<div> a`
                // * Whitespace before the closing tag: `a </div>`
                // * Whitespace before an opening tag: `a <div>`
                JsxChild::Whitespace => {
                    flat.write(&JsxSpace, f);

                    // ```javascript
                    // <div>a
                    // {' '}</div>
                    // ```
                    let is_after_line_break =
                        last.as_ref().map_or(false, |last| last.is_any_line());

                    // `<div>aaa </div>` or `<div> </div>`
                    let is_trailing_or_only_whitespace = children_iter.peek().is_none();

                    if is_trailing_or_only_whitespace || is_after_line_break {
                        multiline.write_with_empty_separator(&JsxRawSpace, f);
                    }
                    // Leading whitespace. Only possible if used together with a expression child
                    //
                    // ```
                    // <div>
                    //
                    //   {' '}
                    //   <b />
                    // </div>
                    // ```
                    else if last.is_none() {
                        multiline.write(&JsxRawSpace, &hard_line_break(), f);
                    } else {
                        multiline.write_with_empty_separator(&JsxSpace, f);
                    }
                }

                // A new line between some JSX text and an element
                JsxChild::Newline => {
                    child_breaks = true;

                    multiline.write_with_empty_separator(&hard_line_break(), f);
                }

                // An empty line between some JSX text and an element
                JsxChild::EmptyLine => {
                    child_breaks = true;

                    multiline.write_with_empty_separator(&empty_line(), f);
                }

                // Any child that isn't text
                JsxChild::NonText(non_text) => {
                    let line_mode = match children_iter.peek() {
                        Some(JsxChild::Newline | JsxChild::Word(_) | JsxChild::Whitespace) => {
                            // Break if the current or next element is a self closing element
                            // ```javascript
                            // <pre className="h-screen overflow-y-scroll" />adefg
                            // ```
                            // Becomes
                            // ```javascript
                            // <pre className="h-screen overflow-y-scroll" />
                            // adefg
                            // ```
                            if matches!(non_text, JsxAnyChild::JsxSelfClosingElement(_)) {
                                Some(LineMode::Hard)
                            } else {
                                Some(LineMode::Soft)
                            }
                        }

                        // Add a hard line break if what comes after the element is not a text or is all whitespace
                        Some(_) => Some(LineMode::Hard),

                        // Don't insert trailing line breaks
                        None => None,
                    };

                    child_breaks = line_mode.map_or(false, |mode| mode.is_hard());

                    let format_separator = format_with(|f| match line_mode {
                        Some(mode) => f.write_element(FormatElement::Line(mode)),
                        None => Ok(()),
                    });

                    if force_multiline {
                        multiline.write(&non_text.format(), &format_separator, f);
                    } else {
                        let mut memoized = non_text.format().memoized();

                        force_multiline = memoized.inspect(f)?.will_break();

                        flat.write(&format_args![memoized, format_separator], f);
                        multiline.write(&memoized, &format_separator, f);
                    }
                }
            }

            if child_breaks {
                flat.disable();
                force_multiline = true;
            }

            last = Some(child);
        }

        let format_multiline = format_once(|f| write!(f, [block_indent(&multiline.finish())]));
        let format_flat_children = flat.finish();

        if force_multiline {
            write!(f, [format_multiline])
        } else {
            write!(f, [best_fitting![format_flat_children, format_multiline]])
        }
    }
}

impl FormatJsxChildList {
    /// Tracks the tokens of [JsxText] and [JsxExpressionChild] nodes to be formatted and
    /// asserts that the suppression comments are checked (they get ignored).
    ///
    /// This is necessary because the formatting of [JsxChildList] bypasses the node formatting for
    /// [JsxText] and [JsxExpressionChild] and instead, formats the nodes itself.
    #[cfg(debug_assertions)]
    fn disarm_debug_assertions(&self, node: &JsxChildList, f: &mut JsFormatter) {
        use rome_formatter::CstFormatContext;
        use rome_js_syntax::{JsAnyExpression, JsAnyLiteralExpression};
        use JsxAnyChild::*;

        for child in node {
            match child {
                JsxExpressionChild(expression)
                    if is_whitespace_jsx_expression(&expression, f.context().comments()) =>
                {
                    f.context()
                        .comments()
                        .mark_suppression_checked(expression.syntax());

                    match expression.expression().unwrap() {
                        JsAnyExpression::JsAnyLiteralExpression(
                            JsAnyLiteralExpression::JsStringLiteralExpression(string_literal),
                        ) => {
                            f.context()
                                .comments()
                                .mark_suppression_checked(string_literal.syntax());

                            f.state_mut()
                                .track_token(&string_literal.value_token().unwrap());

                            f.state_mut()
                                .track_token(&expression.l_curly_token().unwrap());
                            f.state_mut()
                                .track_token(&expression.r_curly_token().unwrap());
                        }
                        _ => unreachable!(),
                    }
                }
                JsxText(text) => {
                    f.state_mut().track_token(&text.value_token().unwrap());

                    // You can't suppress a text node
                    f.context()
                        .comments()
                        .mark_suppression_checked(text.syntax());
                }
                _ => {
                    continue;
                }
            }
        }
    }

    #[cfg(not(debug_assertions))]
    fn disarm_debug_assertions(&self, _: &JsxChildList, _: &mut JsFormatter) {}

    fn layout(&self, meta: ChildrenMeta) -> JsxChildListLayout {
        match self.layout {
            JsxChildListLayout::BestFitting => {
                if meta.any_tag || meta.multiple_expressions {
                    JsxChildListLayout::Multiline
                } else {
                    JsxChildListLayout::BestFitting
                }
            }
            JsxChildListLayout::Multiline => JsxChildListLayout::Multiline,
        }
    }

    /// Computes additional meta data about the children by iterating once over all children.
    fn children_meta(&self, list: &JsxChildList, comments: &Comments<JsLanguage>) -> ChildrenMeta {
        let mut has_expression = false;

        let mut meta = ChildrenMeta::default();

        for child in list {
            use JsxAnyChild::*;

            match child {
                JsxElement(_) | JsxFragment(_) | JsxSelfClosingElement(_) => meta.any_tag = true,
                JsxExpressionChild(expression)
                    if !is_whitespace_jsx_expression(&expression, comments) =>
                {
                    meta.multiple_expressions = has_expression;
                    has_expression = true;
                }
                JsxText(text) => {
                    meta.meaningful_text = meta.meaningful_text
                        || text
                            .value_token()
                            .map_or(false, |token| is_meaningful_jsx_text(token.text()));
                }
                _ => {}
            }
        }

        meta
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub enum JsxChildListLayout {
    /// Prefers to format the children on a single line if possible.
    #[default]
    BestFitting,

    /// Forces the children to be formatted over multiple lines
    Multiline,
}

impl JsxChildListLayout {
    const fn is_multiline(&self) -> bool {
        matches!(self, JsxChildListLayout::Multiline)
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct ChildrenMeta {
    /// `true` if children contains a [JsxElement] or [JsxFragment]
    any_tag: bool,

    /// `true` if children contains more than one [JsxExpressionChild]
    multiple_expressions: bool,

    /// `true` if any child contains meaningful a [JsxText] with meaningful text.
    meaningful_text: bool,
}

#[derive(Copy, Clone, Debug)]
enum WordSeparator {
    /// Separator between two words. Creates a soft line break or space.
    ///
    /// `a b`
    BetweenWords,

    /// A separator of a word at the end of a [JsxText] element. Either because it is the last
    /// child in its parent OR it is right before the start of another child (element, expression, ...).
    ///
    /// ```javascript
    /// <div>a</div>; // last element of parent
    /// <div>a<other /></div> // last element before another element
    /// <div>a{expression}</div> // last element before expression
    /// ```
    ///
    /// Creates a soft line break EXCEPT if the next element is a self closing element, which results in a hard line break:
    ///
    /// ```javascript
    /// a = <div>ab<br/></div>;
    ///
    /// // becomes
    ///
    /// a = (
    ///     <div>
    ///         ab
    ///         <br />
    ///     </div>
    /// );
    /// ```
    EndOfText {
        /// `true` if the next element is a [JsxSelfClosingElement]
        is_next_self_closing: bool,
    },
}

impl WordSeparator {
    /// Returns if formatting this separator will result in a child that expands
    fn will_break(&self) -> bool {
        matches!(
            self,
            WordSeparator::EndOfText {
                is_next_self_closing: true
            }
        )
    }
}

impl Format<JsFormatContext> for WordSeparator {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match self {
            WordSeparator::BetweenWords => soft_line_break_or_space().fmt(f),
            WordSeparator::EndOfText {
                is_next_self_closing: self_closing,
            } => {
                // ```javascript
                // <div>ab<br/></div>
                // ```
                // Becomes
                //
                // ```javascript
                // <div>
                //  ab
                //  <br />
                // </div>
                // ```
                if *self_closing {
                    hard_line_break().fmt(f)
                }
                // Try to fit everything else on a single line
                else {
                    soft_line_break().fmt(f)
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
enum MultilineLayout {
    Fill,
    #[default]
    NoFill,
}

/// Builder that helps to create the output for the multiline layout.
///
/// The multiline layout may use [FormatElement::Fill] element that requires that its children
/// are an alternating sequence of `[element, separator, element, separator, ...]`.
///
/// This requires that each element is wrapped inside of a list if it emits more than one element to uphold
/// the constraints of [FormatElement::Fill].
///
/// However, the wrapping is only necessary for [MultilineLayout::Fill] for when the [FormatElement::Fill] element is used.
///
/// This builder takes care of doing the least amount of work necessary for the chosen layout while also guaranteeing
/// that the written element is valid
#[derive(Debug, Clone)]
struct MultilineBuilder {
    layout: MultilineLayout,
    result: FormatResult<Vec<FormatElement>>,
}

impl MultilineBuilder {
    fn new(layout: MultilineLayout) -> Self {
        Self {
            layout,
            result: Ok(Vec::new()),
        }
    }

    /// Formats an element that does not require a separator
    fn write_with_empty_separator(
        &mut self,
        content: &dyn Format<JsFormatContext>,
        f: &mut JsFormatter,
    ) {
        let result = std::mem::replace(&mut self.result, Ok(Vec::new()));

        self.result = result.and_then(|mut elements| {
            let elements = match self.layout {
                MultilineLayout::Fill => {
                    // Make sure that the separator and content only ever write a single element
                    let mut buffer = VecBuffer::new(f.state_mut());
                    write!(buffer, [content])?;

                    elements.push(buffer.into_element());

                    // Fill requires a sequence of [element, separator, element, separator]
                    // Push an empty list as separator
                    elements.push(FormatElement::List(List::default()));
                    elements
                }
                MultilineLayout::NoFill => {
                    let mut buffer = VecBuffer::new_with_vec(f.state_mut(), elements);
                    write!(buffer, [content])?;

                    buffer.into_vec()
                }
            };

            Ok(elements)
        })
    }

    fn write(
        &mut self,
        content: &dyn Format<JsFormatContext>,
        separator: &dyn Format<JsFormatContext>,
        f: &mut JsFormatter,
    ) {
        let result = std::mem::replace(&mut self.result, Ok(Vec::new()));

        self.result = result.and_then(|mut elements| {
            let elements = match self.layout {
                MultilineLayout::Fill => {
                    // Make sure that the separator and content only ever write a single element
                    let mut buffer = VecBuffer::new(f.state_mut());
                    write!(buffer, [content])?;

                    elements.push(buffer.into_element());

                    let mut buffer = VecBuffer::new_with_vec(f.state_mut(), elements);
                    write!(buffer, [separator])?;
                    buffer.into_vec()
                }
                MultilineLayout::NoFill => {
                    let mut buffer = VecBuffer::new_with_vec(f.state_mut(), elements);
                    write!(buffer, [content, separator])?;

                    buffer.into_vec()
                }
            };
            Ok(elements)
        })
    }

    fn finish(self) -> impl Format<JsFormatContext> {
        format_once(move |f| {
            let elements = self.result?;

            match self.layout {
                MultilineLayout::Fill => {
                    f.write_element(FormatElement::Fill(elements.into_boxed_slice()))
                }
                MultilineLayout::NoFill => f.write_elements(elements),
            }
        })
    }
}

#[derive(Debug)]
struct FlatBuilder {
    result: FormatResult<Vec<FormatElement>>,
    disabled: bool,
}

impl FlatBuilder {
    fn new() -> Self {
        Self {
            result: Ok(Vec::new()),
            disabled: false,
        }
    }

    fn write(&mut self, content: &dyn Format<JsFormatContext>, f: &mut JsFormatter) {
        if self.disabled {
            return;
        }

        let result = std::mem::replace(&mut self.result, Ok(Vec::new()));

        self.result = result.and_then(|elements| {
            let mut buffer = VecBuffer::new_with_vec(f.state_mut(), elements);

            write!(buffer, [content])?;

            Ok(buffer.into_vec())
        })
    }

    fn disable(&mut self) {
        self.disabled = true;
    }

    fn finish(self) -> impl Format<JsFormatContext> {
        format_once(move |f| {
            assert!(!self.disabled, "The flat builder has been disabled and thus, does no longer store any elements. Make sure you don't call disable if you later intend to format the flat content.");

            let elements = self.result?;
            f.write_elements(elements)
        })
    }
}
