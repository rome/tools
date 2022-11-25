use crate::context::trailing_comma::FormatTrailingComma;
use crate::js::declarations::function_declaration::FormatFunctionOptions;
use crate::js::expressions::arrow_function_expression::{
    is_multiline_template_starting_on_same_line, FormatJsArrowFunctionExpressionOptions,
};
use crate::js::lists::array_element_list::can_concisely_print_array_list;
use crate::prelude::*;
use crate::utils::function_body::FunctionBodyCacheMode;
use crate::utils::test_call::is_test_call_expression;
use crate::utils::{is_long_curried_call, write_arguments_multi_line};
use rome_formatter::{format_args, format_element, write, VecBuffer};
use rome_js_syntax::{
    JsAnyCallArgument, JsAnyExpression, JsAnyFunctionBody, JsAnyLiteralExpression, JsAnyStatement,
    JsCallArgumentList, JsCallArguments, JsCallArgumentsFields, JsCallExpression,
    JsExpressionStatement, JsFunctionExpression, JsImportCallExpression, JsLanguage,
    TsAnyReturnType, TsType,
};
use rome_rowan::{AstSeparatedElement, AstSeparatedList, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsCallArguments;

impl FormatNodeRule<JsCallArguments> for FormatJsCallArguments {
    fn fmt_fields(&self, node: &JsCallArguments, f: &mut JsFormatter) -> FormatResult<()> {
        let JsCallArgumentsFields {
            l_paren_token,
            args,
            r_paren_token,
        } = node.as_fields();

        if args.is_empty() {
            return write!(
                f,
                [
                    l_paren_token.format(),
                    format_dangling_comments(node.syntax()).with_soft_block_indent(),
                    r_paren_token.format()
                ]
            );
        }

        let call_expression = node.parent::<JsCallExpression>();

        let (is_commonjs_or_amd_call, is_test_call) =
            call_expression
                .as_ref()
                .map_or((Ok(false), Ok(false)), |call| {
                    (
                        is_commonjs_or_amd_call(node, call),
                        is_test_call_expression(call),
                    )
                });

        if is_commonjs_or_amd_call?
            || is_multiline_template_only_args(node)
            || is_react_hook_with_deps_array(node, f.comments())
            || is_test_call?
        {
            return write!(
                f,
                [
                    l_paren_token.format(),
                    format_with(|f| {
                        f.join_with(space())
                            .entries(
                                args.format_separated(",")
                                    .with_trailing_separator(TrailingSeparator::Omit),
                            )
                            .finish()
                    }),
                    r_paren_token.format()
                ]
            );
        }

        let last_index = args.len().saturating_sub(1);
        let mut has_empty_line = false;

        let arguments: Vec<_> = args
            .elements()
            .enumerate()
            .map(|(index, element)| {
                let leading_lines = element
                    .node()
                    .map_or(0, |node| get_lines_before(node.syntax()));
                has_empty_line = has_empty_line || leading_lines > 1;

                FormatCallArgument::Default {
                    element,
                    is_last: index == last_index,
                    leading_lines,
                }
            })
            .collect();

        if has_empty_line || is_function_composition_args(node) {
            return write!(
                f,
                [FormatAllArgsBrokenOut {
                    l_paren: &l_paren_token.format(),
                    args: &arguments,
                    r_paren: &r_paren_token.format(),
                    node,
                    expand: true,
                }]
            );
        }

        if let Some(group_layout) = arguments_grouped_layout(&args, f.comments()) {
            write_grouped_arguments(node, arguments, group_layout, f)
        } else if is_long_curried_call(call_expression.as_ref()) {
            write!(
                f,
                [
                    l_paren_token.format(),
                    soft_block_indent(&format_once(|f| {
                        write_arguments_multi_line(arguments.iter(), f)
                    })),
                    r_paren_token.format(),
                ]
            )
        } else {
            write!(
                f,
                [FormatAllArgsBrokenOut {
                    l_paren: &l_paren_token.format(),
                    args: &arguments,
                    r_paren: &r_paren_token.format(),
                    node,
                    expand: false,
                }]
            )
        }
    }

    fn fmt_dangling_comments(&self, _: &JsCallArguments, _: &mut JsFormatter) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}

/// Helper for formatting a call argument
enum FormatCallArgument {
    /// Argument that has not been inspected if its formatted content breaks.
    Default {
        element: AstSeparatedElement<JsLanguage, JsAnyCallArgument>,

        /// Whether this is the last element.
        is_last: bool,

        /// The number of lines before this node
        leading_lines: usize,
    },

    /// The argument has been formatted because a caller inspected if it [Self::will_break].
    ///
    /// Allows to re-use the formatted output rather than having to call into the formatting again.
    Inspected {
        /// The formatted element
        content: FormatResult<Option<FormatElement>>,

        /// The separated element
        element: AstSeparatedElement<JsLanguage, JsAnyCallArgument>,

        /// The lines before this element
        leading_lines: usize,
    },
}

impl FormatCallArgument {
    /// Returns `true` if this argument contains any content that forces a group to [`break`](FormatElements::will_break).
    fn will_break(&mut self, f: &mut JsFormatter) -> bool {
        match &self {
            FormatCallArgument::Default {
                element,
                leading_lines,
                ..
            } => {
                let interned = f.intern(&self);

                let breaks = match &interned {
                    Ok(Some(element)) => element.will_break(),
                    _ => false,
                };

                *self = FormatCallArgument::Inspected {
                    content: interned,
                    element: element.clone(),
                    leading_lines: *leading_lines,
                };
                breaks
            }
            FormatCallArgument::Inspected {
                content: Ok(Some(result)),
                ..
            } => result.will_break(),
            FormatCallArgument::Inspected { .. } => false,
        }
    }

    /// Formats the node of this argument and caches the function body.
    ///
    /// See [JsFormatContext::cached_function_body]
    ///
    /// # Panics
    ///
    /// If [`cache_function_body`](Self::cache_function_body) or [`will_break`](Self::will_break) has been called on this argument before.
    fn cache_function_body(&mut self, f: &mut JsFormatter) {
        match &self {
            FormatCallArgument::Default {
                element,
                leading_lines,
                ..
            } => {
                let interned = f.intern(&format_once(|f| {
                    self.fmt_with_cache_mode(FunctionBodyCacheMode::Cache, f)?;
                    Ok(())
                }));

                *self = FormatCallArgument::Inspected {
                    content: interned,
                    element: element.clone(),
                    leading_lines: *leading_lines,
                };
            }
            FormatCallArgument::Inspected { .. } => {
                panic!("`cache` must be called before inspecting or formatting the element.");
            }
        }
    }

    fn fmt_with_cache_mode(
        &self,
        cache_mode: FunctionBodyCacheMode,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        match self {
            // Re-use the cached formatted output if there is any.
            FormatCallArgument::Inspected { content, .. } => match content.clone()? {
                Some(element) => {
                    f.write_element(element)?;
                    Ok(())
                }
                None => Ok(()),
            },
            FormatCallArgument::Default {
                element, is_last, ..
            } => {
                match element.node()? {
                    JsAnyCallArgument::JsAnyExpression(JsAnyExpression::JsFunctionExpression(
                        function,
                    )) => {
                        write!(
                            f,
                            [function.format().with_options(FormatFunctionOptions {
                                body_cache_mode: cache_mode,
                                ..FormatFunctionOptions::default()
                            })]
                        )?;
                    }
                    JsAnyCallArgument::JsAnyExpression(
                        JsAnyExpression::JsArrowFunctionExpression(arrow),
                    ) => {
                        write!(
                            f,
                            [arrow
                                .format()
                                .with_options(FormatJsArrowFunctionExpressionOptions {
                                    body_cache_mode: cache_mode,
                                    ..FormatJsArrowFunctionExpressionOptions::default()
                                })]
                        )?;
                    }
                    node => write!(f, [node.format()])?,
                }

                if let Some(separator) = element.trailing_separator()? {
                    if *is_last {
                        write!(f, [format_removed(separator)])
                    } else {
                        write!(f, [separator.format()])
                    }
                } else if !is_last {
                    Err(FormatError::SyntaxError)
                } else {
                    Ok(())
                }
            }
        }
    }

    /// Returns the number of leading lines before the argument's node
    fn leading_lines(&self) -> usize {
        match self {
            FormatCallArgument::Default { leading_lines, .. } => *leading_lines,
            FormatCallArgument::Inspected { leading_lines, .. } => *leading_lines,
        }
    }

    /// Returns the [`separated element`](AstSeparatedElement) of this argument.
    fn element(&self) -> &AstSeparatedElement<JsLanguage, JsAnyCallArgument> {
        match self {
            FormatCallArgument::Default { element, .. } => element,
            FormatCallArgument::Inspected { element, .. } => element,
        }
    }
}

impl Format<JsFormatContext> for FormatCallArgument {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        self.fmt_with_cache_mode(FunctionBodyCacheMode::default(), f)?;
        Ok(())
    }
}

/// Writes the function arguments, and groups the first or last argument depending on `group_layout`.
fn write_grouped_arguments(
    call_arguments: &JsCallArguments,
    mut arguments: Vec<FormatCallArgument>,
    group_layout: GroupedCallArgumentLayout,
    f: &mut JsFormatter,
) -> FormatResult<()> {
    let l_paren_token = call_arguments.l_paren_token();
    let r_paren_token = call_arguments.r_paren_token();

    let grouped_breaks = {
        let (grouped_arg, other_args) = match group_layout {
            GroupedCallArgumentLayout::GroupedFirstArgument => {
                let (first, tail) = arguments.split_at_mut(1);
                (&mut first[0], tail)
            }
            GroupedCallArgumentLayout::GroupedLastArgument => {
                let end_index = arguments.len().saturating_sub(1);
                let (head, last) = arguments.split_at_mut(end_index);
                (&mut last[0], head)
            }
        };

        let non_grouped_breaks = other_args.iter_mut().any(|arg| arg.will_break(f));

        // if any of the not grouped elements break, then fall back to the variant where
        // all arguments are printed in expanded mode.
        if non_grouped_breaks {
            return write!(
                f,
                [FormatAllArgsBrokenOut {
                    l_paren: &l_paren_token.format(),
                    args: &arguments,
                    r_paren: &r_paren_token.format(),
                    node: call_arguments,
                    expand: true,
                }]
            );
        }

        match grouped_arg.element().node()? {
            JsAnyCallArgument::JsAnyExpression(JsAnyExpression::JsArrowFunctionExpression(_)) => {
                grouped_arg.cache_function_body(f);
            }
            JsAnyCallArgument::JsAnyExpression(JsAnyExpression::JsFunctionExpression(function))
                if !other_args.is_empty() && !has_no_parameters(function) =>
            {
                grouped_arg.cache_function_body(f);
            }
            _ => {
                // Node doesn't have a function body or its a function that doesn't get re-formatted.
            }
        }

        grouped_arg.will_break(f)
    };

    // We now cache them the delimiters tokens. This is needed because `[rome_formatter::best_fitting]` will try to
    // print each version first
    // tokens on the left
    let l_paren = l_paren_token.format().memoized();

    // tokens on the right
    let r_paren = r_paren_token.format().memoized();

    // First write the most expanded variant because it needs `arguments`.
    let most_expanded = {
        let mut buffer = VecBuffer::new(f.state_mut());
        buffer.write_element(FormatElement::Tag(Tag::StartEntry))?;

        write!(
            buffer,
            [FormatAllArgsBrokenOut {
                l_paren: &l_paren,
                args: &arguments,
                r_paren: &r_paren,
                node: call_arguments,
                expand: true,
            }]
        )?;
        buffer.write_element(FormatElement::Tag(Tag::EndEntry))?;

        buffer.into_vec()
    };

    // Now reformat the first or last argument if they happen to be a function or arrow function expression.
    // Function and arrow function expression apply a custom formatting that removes soft line breaks from the parameters,
    // type parameters, and return type annotation.
    //
    // This implementation caches the function body of the "normal" formatted function or arrow function expression
    // to avoid quadratic complexity if the functions' body contains another call expression with an arrow or function expression
    // as first or last argument.
    let last_index = arguments.len() - 1;
    let grouped = arguments
        .into_iter()
        .enumerate()
        .map(|(index, argument)| {
            let layout = match group_layout {
                GroupedCallArgumentLayout::GroupedFirstArgument if index == 0 => {
                    Some(GroupedCallArgumentLayout::GroupedFirstArgument)
                }
                GroupedCallArgumentLayout::GroupedLastArgument if index == last_index => {
                    Some(GroupedCallArgumentLayout::GroupedLastArgument)
                }
                _ => None,
            };

            FormatGroupedArgument {
                argument,
                single_argument_list: last_index == 0,
                layout,
            }
            .memoized()
        })
        .collect::<Vec<_>>();

    // Write the most flat variant with the first or last argument grouped.
    let most_flat = {
        let snapshot = f.state_snapshot();
        let mut buffer = VecBuffer::new(f.state_mut());
        buffer.write_element(FormatElement::Tag(Tag::StartEntry))?;

        let result = write!(
            buffer,
            [
                l_paren,
                format_with(|f| {
                    f.join_with(soft_line_break_or_space())
                        .entries(grouped.iter())
                        .finish()
                }),
                r_paren
            ]
        );

        // Turns out, using the grouped layout isn't a good fit because some parameters of the
        // grouped function or arrow expression break. In that case, fall back to the all args expanded
        // formatting.
        // This back tracking is required because testing if the grouped argument breaks would also return `true`
        // if any content of the function body breaks. But, as far as this is concerned, it's only interested if
        // any content in the signature breaks.
        if matches!(result, Err(FormatError::PoorLayout)) {
            drop(buffer);
            f.restore_state_snapshot(snapshot);

            let mut most_expanded_iter = most_expanded.into_iter();
            // Skip over the Start/EndEntry items.
            most_expanded_iter.next();
            most_expanded_iter.next_back();

            return f.write_elements(most_expanded_iter);
        }

        buffer.write_element(FormatElement::Tag(Tag::EndEntry))?;

        buffer.into_vec().into_boxed_slice()
    };

    // Write the second variant that forces the group of the first/last argument to expand.
    let middle_variant = {
        let mut buffer = VecBuffer::new(f.state_mut());

        buffer.write_element(FormatElement::Tag(Tag::StartEntry))?;

        write!(
            buffer,
            [
                l_paren,
                format_with(|f| {
                    let mut joiner = f.join_with(soft_line_break_or_space());

                    match group_layout {
                        GroupedCallArgumentLayout::GroupedFirstArgument => {
                            joiner.entry(&group(&grouped[0]).should_expand(true));
                            joiner.entries(&grouped[1..]).finish()
                        }
                        GroupedCallArgumentLayout::GroupedLastArgument => {
                            let last_index = grouped.len() - 1;
                            joiner.entries(&grouped[..last_index]);
                            joiner
                                .entry(&group(&grouped[last_index]).should_expand(true))
                                .finish()
                        }
                    }
                }),
                r_paren
            ]
        )?;

        buffer.write_element(FormatElement::Tag(Tag::EndEntry))?;

        buffer.into_vec().into_boxed_slice()
    };

    if grouped_breaks {
        write!(f, [expand_parent()])?;
    }

    // SAFETY: Safe because variants is guaranteed to contain exactly 3 entries:
    // * most flat
    // * middle
    // * most expanded
    // ... and best fitting only requires the most flat/and expanded.
    unsafe {
        f.write_element(FormatElement::BestFitting(
            format_element::BestFitting::from_vec_unchecked(vec![
                most_flat,
                middle_variant,
                most_expanded.into_boxed_slice(),
            ]),
        ))
    }
}

/// Helper for formatting the first grouped argument (see [should_group_first_argument]).
struct FormatGroupedFirstArgument<'a> {
    argument: &'a FormatCallArgument,

    /// Whether this is the only argument in the argument list.
    is_only: bool,
}

impl Format<JsFormatContext> for FormatGroupedFirstArgument<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        use JsAnyExpression::*;

        let element = self.argument.element();

        match element.node()? {
            // Call the arrow function formatting but explicitly passes the call argument layout down
            // so that the arrow function formatting removes any soft line breaks between parameters and the return type.
            JsAnyCallArgument::JsAnyExpression(JsArrowFunctionExpression(arrow)) => {
                with_token_tracking_disabled(f, |f| {
                    write!(
                        f,
                        [arrow
                            .format()
                            .with_options(FormatJsArrowFunctionExpressionOptions {
                                body_cache_mode: FunctionBodyCacheMode::Cached,
                                call_arg_layout: Some(
                                    GroupedCallArgumentLayout::GroupedFirstArgument
                                ),
                                ..FormatJsArrowFunctionExpressionOptions::default()
                            })]
                    )?;

                    match element.trailing_separator()? {
                        None => {
                            if !self.is_only {
                                return Err(FormatError::SyntaxError);
                            }
                        }
                        // The separator is added inside of the arrow function formatting
                        Some(separator) => {
                            if self.is_only {
                                write!(f, [format_removed(separator)])?;
                            } else {
                                write!(f, [separator.format()])?;
                            }
                        }
                    }

                    Ok(())
                })
            }

            // For all other nodes, use the normal formatting (which already has been cached)
            _ => self.argument.fmt(f),
        }
    }
}

/// Helper for formatting the last grouped argument (see [should_group_last_argument]).
struct FormatGroupedLastArgument<'a> {
    argument: &'a FormatCallArgument,
    /// Is this the only argument in the arguments list
    is_only: bool,
}

impl Format<JsFormatContext> for FormatGroupedLastArgument<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        use JsAnyExpression::*;
        let element = self.argument.element();

        // For function and arrow expressions, re-format the node and pass the argument that it is the
        // last grouped argument. This changes the formatting of parameters, type parameters, and return types
        // to remove any soft line breaks.
        match element.node()? {
            JsAnyCallArgument::JsAnyExpression(JsFunctionExpression(function))
                if !self.is_only && !has_no_parameters(function) =>
            {
                with_token_tracking_disabled(f, |f| {
                    write!(
                        f,
                        [function.format().with_options(FormatFunctionOptions {
                            body_cache_mode: FunctionBodyCacheMode::Cached,
                            call_argument_layout: Some(
                                GroupedCallArgumentLayout::GroupedLastArgument
                            ),
                        })]
                    )?;

                    if let Some(separator) = element.trailing_separator()? {
                        write!(f, [format_removed(separator)])?;
                    }

                    Ok(())
                })
            }

            JsAnyCallArgument::JsAnyExpression(JsArrowFunctionExpression(arrow)) => {
                with_token_tracking_disabled(f, |f| {
                    write!(
                        f,
                        [arrow
                            .format()
                            .with_options(FormatJsArrowFunctionExpressionOptions {
                                body_cache_mode: FunctionBodyCacheMode::Cached,
                                call_arg_layout: Some(
                                    GroupedCallArgumentLayout::GroupedLastArgument
                                ),
                                ..FormatJsArrowFunctionExpressionOptions::default()
                            })]
                    )?;

                    if let Some(separator) = element.trailing_separator()? {
                        write!(f, [format_removed(separator)])?;
                    }

                    Ok(())
                })
            }
            _ => self.argument.fmt(f),
        }
    }
}

/// Disable the token tracking because it is necessary to format function/arrow expressions slightly different.
fn with_token_tracking_disabled<F: FnOnce(&mut JsFormatter) -> R, R>(
    f: &mut JsFormatter,
    callback: F,
) -> R {
    let was_disabled = f.state().is_token_tracking_disabled();
    f.state_mut().set_token_tracking_disabled(true);

    let result = callback(f);

    f.state_mut().set_token_tracking_disabled(was_disabled);

    result
}

/// Tests if `expression` has an empty parameters list.
fn has_no_parameters(expression: &JsFunctionExpression) -> bool {
    match expression.parameters() {
        // Use default formatting for expressions without parameters, will return `Err` anyway
        Err(_) => true,
        Ok(parameters) => parameters.items().is_empty(),
    }
}

/// Helper for formatting a grouped call argument (see [should_group_first_argument] and [should_group_last_argument]).
struct FormatGroupedArgument {
    argument: FormatCallArgument,

    /// Whether this argument is the only argument in the argument list.
    single_argument_list: bool,

    /// The layout to use for this argument.
    layout: Option<GroupedCallArgumentLayout>,
}

impl Format<JsFormatContext> for FormatGroupedArgument {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match self.layout {
            Some(GroupedCallArgumentLayout::GroupedFirstArgument) => FormatGroupedFirstArgument {
                argument: &self.argument,
                is_only: self.single_argument_list,
            }
            .fmt(f),
            Some(GroupedCallArgumentLayout::GroupedLastArgument) => FormatGroupedLastArgument {
                argument: &self.argument,
                is_only: self.single_argument_list,
            }
            .fmt(f),
            None => self.argument.fmt(f),
        }
    }
}

struct FormatAllArgsBrokenOut<'a> {
    l_paren: &'a dyn Format<JsFormatContext>,
    args: &'a [FormatCallArgument],
    r_paren: &'a dyn Format<JsFormatContext>,
    expand: bool,
    node: &'a JsCallArguments,
}

impl<'a> Format<JsFormatContext> for FormatAllArgsBrokenOut<'a> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let is_inside_import = self.node.parent::<JsImportCallExpression>().is_some();

        write!(
            f,
            [group(&format_args![
                self.l_paren,
                soft_block_indent(&format_with(|f| {
                    for (index, entry) in self.args.iter().enumerate() {
                        if index > 0 {
                            match entry.leading_lines() {
                                0 | 1 => write!(f, [soft_line_break_or_space()])?,
                                _ => write!(f, [empty_line()])?,
                            }
                        }

                        write!(f, [entry])?;
                    }

                    if !is_inside_import {
                        write!(f, [FormatTrailingComma::All])?;
                    }
                    Ok(())
                })),
                self.r_paren,
            ])
            .should_expand(self.expand)]
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub enum GroupedCallArgumentLayout {
    /// Group the first call argument.
    GroupedFirstArgument,

    /// Group the last call argument.
    GroupedLastArgument,
}

fn arguments_grouped_layout(
    args: &JsCallArgumentList,
    comments: &JsComments,
) -> Option<GroupedCallArgumentLayout> {
    if should_group_first_argument(args, comments).unwrap_or(false) {
        Some(GroupedCallArgumentLayout::GroupedFirstArgument)
    } else if should_group_last_argument(args, comments).unwrap_or(false) {
        Some(GroupedCallArgumentLayout::GroupedLastArgument)
    } else {
        None
    }
}

/// Checks if the the first argument requires grouping
fn should_group_first_argument(
    list: &JsCallArgumentList,
    comments: &JsComments,
) -> SyntaxResult<bool> {
    use JsAnyExpression::*;

    let mut iter = list.iter();
    match (iter.next(), iter.next()) {
        (
            Some(Ok(JsAnyCallArgument::JsAnyExpression(first))),
            Some(Ok(JsAnyCallArgument::JsAnyExpression(second))),
        ) if iter.next().is_none() => {
            match &first {
                JsFunctionExpression(_) => {}
                JsArrowFunctionExpression(arrow) => {
                    if !matches!(arrow.body(), Ok(JsAnyFunctionBody::JsFunctionBody(_))) {
                        return Ok(false);
                    }
                }
                _ => return Ok(false),
            };

            if matches!(
                second,
                JsArrowFunctionExpression(_) | JsFunctionExpression(_) | JsConditionalExpression(_)
            ) {
                return Ok(false);
            }

            Ok(!comments.has_comments(first.syntax())
                && !can_group_expression_argument(&second, false, comments)?)
        }
        _ => Ok(false),
    }
}

/// Checks if the last argument should be grouped.
fn should_group_last_argument(
    list: &JsCallArgumentList,
    comments: &JsComments,
) -> SyntaxResult<bool> {
    use JsAnyExpression::*;

    let mut iter = list.iter();
    let last = iter.next_back();

    match last {
        Some(Ok(JsAnyCallArgument::JsAnyExpression(last))) => {
            if comments.has_leading_comments(last.syntax())
                || comments.has_trailing_comments(last.syntax())
            {
                return Ok(false);
            }

            if !can_group_expression_argument(&last, false, comments)? {
                return Ok(false);
            }

            let penultimate = iter.next_back();

            if let Some(Ok(penultimate)) = &penultimate {
                if penultimate.syntax().kind() == last.syntax().kind() {
                    return Ok(false);
                }
            }

            match last {
                JsArrayExpression(array) if list.len() > 1 => {
                    // Not for `useEffect`
                    if list.len() == 2
                        && matches!(
                            penultimate,
                            Some(Ok(JsAnyCallArgument::JsAnyExpression(
                                JsArrowFunctionExpression(_)
                            )))
                        )
                    {
                        return Ok(false);
                    }

                    if can_concisely_print_array_list(&array.elements(), comments) {
                        return Ok(false);
                    }

                    Ok(true)
                }
                _ => Ok(true),
            }
        }
        _ => Ok(false),
    }
}

/// Checks if `argument` benefits from grouping in call arguments.
fn can_group_expression_argument(
    argument: &JsAnyExpression,
    is_arrow_recursion: bool,
    comments: &JsComments,
) -> SyntaxResult<bool> {
    use JsAnyExpression::*;

    let result = match argument {
        JsObjectExpression(object_expression) => {
            !object_expression.members().is_empty()
                || comments.has_comments(object_expression.syntax())
        }

        JsArrayExpression(array_expression) => {
            !array_expression.elements().is_empty()
                || comments.has_comments(array_expression.syntax())
        }

        TsTypeAssertionExpression(assertion_expression) => {
            can_group_expression_argument(&assertion_expression.expression()?, false, comments)?
        }

        TsAsExpression(as_expression) => {
            can_group_expression_argument(&as_expression.expression()?, false, comments)?
        }

        JsArrowFunctionExpression(arrow_function) => {
            let body = arrow_function.body()?;
            let return_type_annotation = arrow_function.return_type_annotation();

            // Handles cases like:
            //
            // app.get("/", (req, res): void => {
            //     res.send("Hello World!");
            // });
            //
            // export class Thing implements OtherThing {
            //   do: (type: Type) => Provider<Prop> = memoize(
            //     (type: ObjectType): Provider<Opts> => {}
            //   );
            // }
            let can_group_type =
                return_type_annotation
                    .and_then(|rty| rty.ty().ok())
                    .map_or(true, |any_type| match any_type {
                        TsAnyReturnType::TsType(TsType::TsReferenceType(_)) => match &body {
                            JsAnyFunctionBody::JsFunctionBody(body) => {
                                body.statements().iter().any(|statement| {
                                    !matches!(statement, JsAnyStatement::JsEmptyStatement(_))
                                }) || comments.has_dangling_comments(body.syntax())
                            }
                            _ => false,
                        },
                        _ => true,
                    });

            let can_group_body = match &body {
                JsAnyFunctionBody::JsFunctionBody(_)
                | JsAnyFunctionBody::JsAnyExpression(
                    JsObjectExpression(_) | JsArrayExpression(_) | JsxTagExpression(_),
                ) => true,
                JsAnyFunctionBody::JsAnyExpression(arrow @ JsArrowFunctionExpression(_)) => {
                    can_group_expression_argument(arrow, true, comments)?
                }
                JsAnyFunctionBody::JsAnyExpression(
                    JsCallExpression(_) | JsConditionalExpression(_),
                ) if !is_arrow_recursion => true,
                _ => false,
            };

            can_group_body && can_group_type
        }

        JsFunctionExpression(_) => true,
        _ => false,
    };

    Ok(result)
}

/// Tests if this is a call to commonjs [`require`](https://nodejs.org/api/modules.html#requireid)
/// or amd's [`define`](https://github.com/amdjs/amdjs-api/wiki/AMD#define-function-) function.
fn is_commonjs_or_amd_call(
    arguments: &JsCallArguments,
    call: &JsCallExpression,
) -> SyntaxResult<bool> {
    let callee = call.callee()?;

    Ok(match callee {
        JsAnyExpression::JsIdentifierExpression(identifier) => {
            let reference = identifier.name()?;

            if reference.has_name("require") {
                true
            } else if reference.has_name("define") {
                let in_statement = call.parent::<JsExpressionStatement>().is_some();

                if in_statement {
                    let args = arguments.args();
                    match args.len() {
                        1 => true,
                        2 => matches!(
                            args.first(),
                            Some(Ok(JsAnyCallArgument::JsAnyExpression(
                                JsAnyExpression::JsArrayExpression(_)
                            )))
                        ),
                        3 => {
                            let mut iter = args.iter();
                            let first = iter.next();
                            let second = iter.next();
                            matches!(
                                (first, second),
                                (
                                    Some(Ok(JsAnyCallArgument::JsAnyExpression(
                                        JsAnyExpression::JsAnyLiteralExpression(
                                            JsAnyLiteralExpression::JsStringLiteralExpression(_)
                                        )
                                    ))),
                                    Some(Ok(JsAnyCallArgument::JsAnyExpression(
                                        JsAnyExpression::JsArrayExpression(_)
                                    )))
                                )
                            )
                        }
                        _ => false,
                    }
                } else {
                    false
                }
            } else {
                false
            }
        }
        _ => false,
    })
}

/// Returns `true` if `arguments` contains a single [multiline template literal argument that starts on its own ](is_multiline_template_starting_on_same_line).
fn is_multiline_template_only_args(arguments: &JsCallArguments) -> bool {
    let args = arguments.args();

    match args.first() {
        Some(Ok(JsAnyCallArgument::JsAnyExpression(JsAnyExpression::JsTemplate(template))))
            if args.len() == 1 =>
        {
            is_multiline_template_starting_on_same_line(&template)
        }
        _ => false,
    }
}

/// This function is used to check if the code is a hook-like code:
///
/// ```js
/// useMemo(() => {}, [])
/// ```
fn is_react_hook_with_deps_array(arguments: &JsCallArguments, comments: &JsComments) -> bool {
    use JsAnyExpression::*;
    let mut args = arguments.args().iter();

    match (args.next(), args.next()) {
        (
            Some(Ok(JsAnyCallArgument::JsAnyExpression(JsArrowFunctionExpression(callback)))),
            Some(Ok(JsAnyCallArgument::JsAnyExpression(JsArrayExpression(deps)))),
        ) if arguments.args().len() == 2 => {
            if comments.has_comments(callback.syntax()) || comments.has_comments(deps.syntax()) {
                return false;
            }

            if !callback
                .parameters()
                .map_or(false, |parameters| parameters.is_empty())
            {
                return false;
            }

            matches!(callback.body(), Ok(JsAnyFunctionBody::JsFunctionBody(_)))
        }
        _ => false,
    }
}

/// Tests if a call has multiple anonymous function like (arrow or function expression) arguments.
///
/// ## Examples
///
/// ```javascript
/// compose(sortBy(x => x), flatten, map(x => [x, x*2]));
/// ```
fn is_function_composition_args(arguments: &JsCallArguments) -> bool {
    let args = arguments.args();

    if args.len() <= 1 {
        return false;
    }

    let mut has_seen_function_like = false;

    for arg in args.iter().flatten() {
        use JsAnyExpression::*;
        match arg {
            JsAnyCallArgument::JsAnyExpression(
                JsFunctionExpression(_) | JsArrowFunctionExpression(_),
            ) => {
                if has_seen_function_like {
                    return true;
                }
                has_seen_function_like = true;
            }
            JsAnyCallArgument::JsAnyExpression(JsCallExpression(call)) => {
                if call.arguments().map_or(false, |call_arguments| {
                    call_arguments.args().iter().flatten().any(|arg| {
                        matches!(
                            arg,
                            JsAnyCallArgument::JsAnyExpression(
                                JsFunctionExpression(_) | JsArrowFunctionExpression(_)
                            )
                        )
                    })
                }) {
                    return true;
                }
            }
            _ => {
                continue;
            }
        }
    }

    false
}
