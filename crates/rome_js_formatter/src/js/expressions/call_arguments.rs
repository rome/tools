use crate::js::expressions::arrow_function_expression::{
    is_multiline_template_starting_on_same_line, FormatJsArrowFunctionExpressionOptions,
};
use crate::js::lists::array_element_list::can_concisely_print_array_list;
use crate::prelude::*;
use crate::utils::{is_long_curried_call, write_arguments_multi_line};
use rome_formatter::{format_args, format_element, write, CstFormatContext, VecBuffer};
use rome_js_syntax::{
    JsAnyArrowFunctionParameters, JsAnyBinding, JsAnyBindingPattern, JsAnyCallArgument,
    JsAnyExpression, JsAnyFormalParameter, JsAnyFunctionBody, JsAnyLiteralExpression, JsAnyName,
    JsAnyParameter, JsAnyStatement, JsArrowFunctionExpression, JsCallArgumentList, JsCallArguments,
    JsCallArgumentsFields, JsCallExpression, JsExpressionStatement, JsFunctionExpression,
    JsLanguage, JsParameters, TsAnyReturnType, TsType,
};
use rome_rowan::{AstSeparatedElement, AstSeparatedList, SyntaxResult, SyntaxTokenText};

#[derive(Debug, Clone, Default)]
pub struct FormatJsCallArguments;

impl FormatNodeRule<JsCallArguments> for FormatJsCallArguments {
    fn fmt_fields(&self, node: &JsCallArguments, f: &mut JsFormatter) -> FormatResult<()> {
        let JsCallArgumentsFields {
            l_paren_token,
            args,
            r_paren_token,
        } = node.as_fields();

        let l_paren_token = l_paren_token?;
        let r_paren_token = r_paren_token?;

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
                    format_once(|f| {
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

        let mut arguments: Vec<_> = args
            .elements()
            .enumerate()
            .map(|(index, element)| {
                let leading_lines = element
                    .node()
                    .map_or(0, |node| get_lines_before(node.syntax()));
                has_empty_line = has_empty_line || leading_lines > 1;

                FormatArgumentElement::Unformatted {
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
                    args: arguments.iter(),
                    r_paren: &r_paren_token.format(),
                    expand: true,
                }]
            );
        }

        let comments = f.context().comments();
        let should_group_first_argument = should_group_first_argument(&args, comments)?;
        let should_group_last_argument =
            !should_group_first_argument && should_group_last_argument(&args, comments)?;

        // if the first or last groups needs grouping, then we prepare some special formatting
        if should_group_first_argument || should_group_last_argument {
            // We finished the "simple cases", we now need to use `best_fitting`.
            // We now need to allocate a new vector with cached nodes, this is needed because
            // we can't attempt to print the same node twice without incur in "printed token twice" errors.
            // We also disallow the trailing separator, we are interested in doing it manually.
            let (grouped_arg, other_args) = if should_group_first_argument {
                let (first, tail) = arguments.split_at_mut(1);
                (&mut first[0], tail)
            } else {
                let end_index = arguments.len().saturating_sub(1);
                let (head, last) = arguments.split_at_mut(end_index);
                (&mut last[0], head)
            };

            let non_grouped_breaks = other_args.iter_mut().any(|arg| arg.will_break(f));

            // We now cache them the delimiters tokens. This is needed because `[rome_formatter::best_fitting]` will try to
            // print each version first
            // tokens on the left
            let l_paren = l_paren_token.format().memoized();

            // tokens on the right
            let r_paren = r_paren_token.format().memoized();

            if non_grouped_breaks {
                return write!(
                    f,
                    [FormatAllArgsBrokenOut {
                        l_paren: &l_paren,
                        args: arguments.iter(),
                        r_paren: &r_paren,
                        expand: true
                    }]
                );
            }

            let grouped_breaks = grouped_arg.will_break(f);

            drop(grouped_arg);
            drop(other_args);

            let last_index = arguments.len() - 1;
            let grouped = arguments
                .iter()
                .enumerate()
                .map(|(index, element)| {
                    FormatGroupedElement {
                        element,
                        single_argument_list: last_index == 0,
                        layout: if should_group_first_argument && index == 0 {
                            Some(ExpandCallArgumentLayout::ExpandFirstArg)
                        } else if should_group_last_argument && index == last_index {
                            Some(ExpandCallArgumentLayout::ExpandLastArg)
                        } else {
                            None
                        },
                    }
                    .memoized()
                })
                .collect::<Vec<_>>();

            // TODO Possible to re-use most expanded variant for AllArgsBrokenOut rathern than having to allocate a new vec
            // for groued
            // Most expanded variant
            let most_expanded = {
                let mut buffer = VecBuffer::new(f.state_mut());
                buffer.write_element(FormatElement::Tag(Tag::StartEntry))?;

                write!(
                    buffer,
                    [FormatAllArgsBrokenOut {
                        l_paren: &l_paren,
                        args: arguments.iter(),
                        r_paren: &r_paren,
                        expand: true
                    }]
                )?;
                buffer.write_element(FormatElement::Tag(Tag::EndEntry))?;

                buffer.into_vec().into_boxed_slice()
            };

            // Write the most flat variant
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

                if matches!(result, Err(FormatError::PoorLayout)) {
                    drop(buffer);
                    f.restore_state_snapshot(snapshot);

                    return write!(
                        f,
                        [FormatAllArgsBrokenOut {
                            l_paren: &l_paren,
                            args: arguments.iter(),
                            r_paren: &r_paren,
                            expand: true
                        }]
                    );
                }

                buffer.write_element(FormatElement::Tag(Tag::EndEntry))?;

                buffer.into_vec().into_boxed_slice()
            };

            // Write second variant
            let middle_variant = {
                let mut buffer = VecBuffer::new(f.state_mut());

                buffer.write_element(FormatElement::Tag(Tag::StartEntry))?;

                write!(
                    buffer,
                    [
                        l_paren,
                        format_with(|f| {
                            // `should_group_first_argument` and `should_group_last_argument` are mutually exclusive
                            // which means that if one is `false`, then the other is `true`.
                            // This means that in this branch we format the case where `should_group_first_argument`,
                            // in the else branch we format the case where `should_group_last_argument` is `true`.
                            let mut joiner = f.join_with(soft_line_break_or_space());
                            if should_group_first_argument {
                                // special formatting of the first element
                                joiner.entry(&group(&grouped[0]).should_expand(true));
                                joiner.entries(&grouped[1..]).finish()
                            } else {
                                let last_index = grouped.len() - 1;
                                joiner.entries(&grouped[..last_index]);
                                joiner
                                    .entry(&group(&grouped[last_index]).should_expand(true))
                                    .finish()
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

            // SAFETY: Safe because variants is guaranteed to contain exactly 3 entries (two are required)
            unsafe {
                f.write_element(FormatElement::BestFitting(
                    format_element::BestFitting::from_vec_unchecked(vec![
                        most_flat,
                        middle_variant,
                        most_expanded,
                    ]),
                ))
            }
        } else if call_expression.as_ref().map_or(false, is_long_curried_call) {
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
                    args: arguments.iter(),
                    r_paren: &r_paren_token.format(),
                    expand: false
                }]
            )
        }
    }

    fn fmt_dangling_comments(&self, _: &JsCallArguments, _: &mut JsFormatter) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}

enum FormatArgumentElement {
    Unformatted {
        element: AstSeparatedElement<JsLanguage, JsAnyCallArgument>,
        is_last: bool,
        leading_lines: usize,
    },
    // TODO use memoized?
    Memoized {
        content: FormatResult<Option<FormatElement>>,
        element: AstSeparatedElement<JsLanguage, JsAnyCallArgument>,
        leading_lines: usize,
    },
}

impl FormatArgumentElement {
    fn will_break(&mut self, f: &mut JsFormatter) -> bool {
        let breaks = match &self {
            FormatArgumentElement::Unformatted {
                element,
                leading_lines,
                ..
            } => {
                let interned = f.intern(&self);

                let breaks = match &interned {
                    Ok(Some(element)) => element.will_break(),
                    _ => false,
                };

                *self = FormatArgumentElement::Memoized {
                    content: interned,
                    element: element.clone(),
                    leading_lines: *leading_lines,
                };
                breaks
            }
            FormatArgumentElement::Memoized {
                content: Ok(Some(result)),
                ..
            } => result.will_break(),
            FormatArgumentElement::Memoized { .. } => false,
        };

        breaks
    }

    fn leading_lines(&self) -> usize {
        match self {
            FormatArgumentElement::Unformatted { leading_lines, .. } => *leading_lines,
            FormatArgumentElement::Memoized { leading_lines, .. } => *leading_lines,
        }
    }

    fn element(&self) -> &AstSeparatedElement<JsLanguage, JsAnyCallArgument> {
        match self {
            FormatArgumentElement::Unformatted { element, .. } => element,
            FormatArgumentElement::Memoized { element, .. } => element,
        }
    }
}

impl Format<JsFormatContext> for FormatArgumentElement {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match self {
            FormatArgumentElement::Memoized { content, .. } => match content.clone()? {
                Some(element) => f.write_element(element),
                None => Ok(()),
            },
            FormatArgumentElement::Unformatted {
                element, is_last, ..
            } => {
                let node = element.node()?;

                write!(f, [node.format()])?;

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
}

struct FormatFirstGroupedElement<'a> {
    element: &'a FormatArgumentElement,
    is_only: bool,
}

impl Format<JsFormatContext> for FormatFirstGroupedElement<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        use JsAnyExpression::*;

        let element = self.element.element();

        match element.node()? {
            JsAnyCallArgument::JsAnyExpression(JsArrowFunctionExpression(arrow))
                if !is_simple_arrow_function_expression(arrow) =>
            {
                let was_enabled = f.state().is_token_tracking_enabled();

                f.state_mut().set_token_tracking_enabled(false);

                write!(
                    f,
                    [arrow
                        .format()
                        .with_options(FormatJsArrowFunctionExpressionOptions {
                            assignment_layout: None,
                            call_arg_layout: Some(ExpandCallArgumentLayout::ExpandFirstArg)
                        })]
                )?;

                match element.trailing_separator()? {
                    None => {
                        if !self.is_only {
                            return Err(FormatError::SyntaxError);
                        }
                    }
                    Some(separator) => {
                        if self.is_only {
                            write!(f, [format_removed(separator)])?;
                        } else {
                            write!(f, [separator.format()])?;
                        }
                    }
                }

                f.state_mut().set_token_tracking_enabled(was_enabled);

                Ok(())
            }
            _ => self.element.fmt(f),
        }
    }
}

struct FormatLastGroupElement<'a> {
    element: &'a FormatArgumentElement,
    /// Is this the only argument in the arguments list
    is_only: bool,
}

impl Format<JsFormatContext> for FormatLastGroupElement<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        use JsAnyExpression::*;
        let element = self.element.element();

        match element.node()? {
            JsAnyCallArgument::JsAnyExpression(JsFunctionExpression(function))
                if !self.is_only && !is_simple_function_expression(function) =>
            {
                let was_enabled = f.state().is_token_tracking_enabled();

                f.state_mut().set_token_tracking_enabled(false);

                write!(
                    f,
                    [function
                        .format()
                        .with_options(Some(ExpandCallArgumentLayout::ExpandLastArg))]
                )?;

                f.state_mut().set_token_tracking_enabled(was_enabled);

                Ok(())
            }
            JsAnyCallArgument::JsAnyExpression(JsArrowFunctionExpression(arrow))
                if !is_simple_arrow_function_expression(arrow) =>
            {
                let was_enabled = f.state().is_token_tracking_enabled();

                f.state_mut().set_token_tracking_enabled(false);

                write!(
                    f,
                    [arrow
                        .format()
                        .with_options(FormatJsArrowFunctionExpressionOptions {
                            assignment_layout: None,
                            call_arg_layout: Some(ExpandCallArgumentLayout::ExpandLastArg)
                        })]
                )?;

                if let Some(separator) = element.trailing_separator()? {
                    write!(f, [format_removed(&separator)])?;
                }

                f.state_mut().set_token_tracking_enabled(was_enabled);

                Ok(())
            }
            _ => self.element.fmt(f),
        }
    }
}

fn is_simple_function_expression(expression: &JsFunctionExpression) -> bool {
    match expression.parameters() {
        // Use default formatting for expressions without parameters, will return an Err anyway
        Err(_) => true,
        Ok(parameters) => parameters.items().is_empty(),
    }
}

fn is_simple_arrow_function_expression(expression: &JsArrowFunctionExpression) -> bool {
    expression.parameters().map_or(false, |parameters| parameters.is_empty()) && expression.return_type_annotation().is_none() &&
        // expand last args disables arrow chain formatting so it's necessary to call the formatting again.
        !matches!(expression.body(), Ok(JsAnyFunctionBody::JsAnyExpression(JsArrowFunctionExpression_)))
}

fn is_simple_parameters(parameters: &JsParameters) -> bool {
    let items = parameters.items();

    match items.first() {
        None => true,
        Some(Ok(JsAnyParameter::JsAnyFormalParameter(
            JsAnyFormalParameter::JsFormalParameter(formal),
        ))) if items.len() == 1 => {
            matches!(
                formal.binding(),
                Ok(JsAnyBindingPattern::JsAnyBinding(
                    JsAnyBinding::JsIdentifierBinding(_)
                ))
            ) && formal.type_annotation().is_none()
                && formal.initializer().is_none()
        }
        _ => false,
    }
}

struct FormatGroupedElement<'a> {
    element: &'a FormatArgumentElement,
    single_argument_list: bool,
    layout: Option<ExpandCallArgumentLayout>,
}

impl Format<JsFormatContext> for FormatGroupedElement<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match self.layout {
            Some(ExpandCallArgumentLayout::ExpandFirstArg) => FormatFirstGroupedElement {
                element: &self.element,
                is_only: self.single_argument_list,
            }
            .fmt(f),
            Some(ExpandCallArgumentLayout::ExpandLastArg) => FormatLastGroupElement {
                element: &self.element,
                is_only: self.single_argument_list,
            }
            .fmt(f),
            None => self.element.fmt(f),
        }
    }
}

trait FormatArgumentEntry: Format<JsFormatContext> {
    fn leading_lines(&self) -> usize;
}

struct FormatAllArgsBrokenOut<'a, I> {
    l_paren: &'a dyn Format<JsFormatContext>,
    args: I,
    r_paren: &'a dyn Format<JsFormatContext>,
    expand: bool,
}

impl<'a, I> Format<JsFormatContext> for FormatAllArgsBrokenOut<'a, I>
where
    I: Iterator<Item = &'a FormatArgumentElement> + Clone,
{
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        write!(
            f,
            [group(&format_args![
                self.l_paren,
                soft_block_indent(&format_with(|f| {
                    for (index, entry) in self.args.clone().enumerate() {
                        if index > 0 {
                            match entry.leading_lines() {
                                0 | 1 => write!(f, [soft_line_break_or_space()])?,
                                _ => write!(f, [empty_line()])?,
                            }
                        }

                        write!(f, [entry])?;
                    }

                    write!(f, [if_group_breaks(&text(","))])
                })),
                self.r_paren,
            ])
            .should_expand(self.expand)]
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ExpandCallArgumentLayout {
    ExpandLastArg,
    ExpandFirstArg,
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

/// Checks if the last group requires grouping
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

/// Checks if the current argument could be grouped
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

/// This is a specialised function that checks if the current [call expression]
/// resembles a call expression usually used by a testing frameworks.
///
/// If the [call expression] matches the criteria, a different formatting is applied.
///
/// To evaluable the eligibility of a  [call expression] to be a test framework like,
/// we need to check its [callee] and its [arguments].
///
/// 1. The [callee] must contain a name or a chain of names that belongs to the
/// test frameworks, for example: `test()`, `test.only()`, etc.
/// 2. The [arguments] should be at the least 2
/// 3. The first argument has to be a string literal
/// 4. The third argument, if present, has to be a number literal
/// 5. The second argument has to be an [arrow function expression] or [function expression]
/// 6. Both function must have zero or one parameters
///
/// [call expression]: crate::rome_js_syntax::JsCallExpression
/// [callee]: crate::rome_js_syntax::JsAnyExpression
/// [arguments]: crate::rome_js_syntax::JsCallArgumentList
/// [arrow function expression]: crate::rome_js_syntax::JsArrowFunctionExpression
/// [function expression]: crate::rome_js_syntax::JsCallArgumentList
pub(crate) fn is_test_call_expression(call_expression: &JsCallExpression) -> SyntaxResult<bool> {
    use JsAnyExpression::*;

    let callee = call_expression.callee()?;
    let arguments = call_expression.arguments()?;

    let mut args = arguments.args().iter();

    match (args.next(), args.next(), args.next()) {
        (Some(Ok(argument)), None, None) if arguments.args().len() == 1 => {
            if is_angular_test_wrapper(&call_expression.clone().into())
                && call_expression
                    .parent::<JsCallArgumentList>()
                    .and_then(|arguments_list| arguments_list.parent::<JsCallArguments>())
                    .and_then(|arguments| arguments.parent::<self::JsCallExpression>())
                    .map_or(Ok(false), |parent| is_test_call_expression(&parent))?
            {
                return Ok(matches!(
                    argument,
                    JsAnyCallArgument::JsAnyExpression(
                        JsArrowFunctionExpression(_) | JsFunctionExpression(_)
                    )
                ));
            }

            if is_unit_test_set_up_callee(&callee) {
                return Ok(argument
                    .as_js_any_expression()
                    .map_or(false, is_angular_test_wrapper));
            }

            Ok(false)
        }

        // it("description", ..)
        (
            Some(Ok(JsAnyCallArgument::JsAnyExpression(
                JsTemplate(_)
                | JsAnyLiteralExpression(self::JsAnyLiteralExpression::JsStringLiteralExpression(_)),
            ))),
            Some(Ok(second)),
            third,
        ) if arguments.args().len() <= 3 && contains_a_test_pattern(callee.clone())? => {
            // it('name', callback, duration)
            if !matches!(
                third,
                None | Some(Ok(JsAnyCallArgument::JsAnyExpression(
                    JsAnyLiteralExpression(
                        self::JsAnyLiteralExpression::JsNumberLiteralExpression(_)
                    )
                )))
            ) {
                return Ok(false);
            }

            if second
                .as_js_any_expression()
                .map_or(false, |second| is_angular_test_wrapper(second))
            {
                return Ok(true);
            }

            let (parameters, has_block_body) = match second {
                JsAnyCallArgument::JsAnyExpression(JsFunctionExpression(function)) => (
                    function
                        .parameters()
                        .map(JsAnyArrowFunctionParameters::from),
                    true,
                ),
                JsAnyCallArgument::JsAnyExpression(JsArrowFunctionExpression(arrow)) => (
                    arrow.parameters(),
                    arrow.body().map_or(false, |body| {
                        matches!(body, JsAnyFunctionBody::JsFunctionBody(_))
                    }),
                ),
                _ => return Ok(false),
            };

            Ok(arguments.args().len() == 2 || (parameters?.len() <= 1 && has_block_body))
        }
        _ => Ok(false),
    }
}

/// Note: `inject` is used in AngularJS 1.x, `async` and `fakeAsync` in
/// Angular 2+, although `async` is deprecated and replaced by `waitForAsync`
/// since Angular 12.
///
/// example: https://docs.angularjs.org/guide/unit-testing#using-beforeall-
///
/// @param {CallExpression} node
/// @returns {boolean}
///
fn is_angular_test_wrapper(expression: &JsAnyExpression) -> bool {
    use JsAnyExpression::*;
    match expression {
        JsCallExpression(call_expression) => match call_expression.callee() {
            Ok(JsIdentifierExpression(identifier)) => identifier
                .name()
                .and_then(|name| name.value_token())
                .map_or(false, |name| {
                    matches!(
                        name.text_trimmed(),
                        "async" | "inject" | "fakeAsync" | "waitForAsync"
                    )
                }),
            _ => false,
        },
        _ => false,
    }
}

fn is_unit_test_set_up_callee(callee: &JsAnyExpression) -> bool {
    match callee {
        JsAnyExpression::JsIdentifierExpression(identifier) => identifier
            .name()
            .and_then(|name| name.value_token())
            .map_or(false, |name| {
                matches!(
                    name.text_trimmed(),
                    "beforeEach" | "beforeAll" | "afterEach" | "afterAll"
                )
            }),
        _ => false,
    }
}

/// This function checks if a call expressions has one of the following members:
/// - `it`
/// - `it.only`
/// - `it.skip`
/// - `describe`
/// - `describe.only`
/// - `describe.skip`
/// - `test`
/// - `test.only`
/// - `test.skip`
/// - `test.step`
/// - `test.describe`
/// - `test.describe.only`
/// - `test.describe.parallel`
/// - `test.describe.parallel.only`
/// - `test.describe.serial`
/// - `test.describe.serial.only`
/// - `skip`
/// - `xit`
/// - `xdescribe`
/// - `xtest`
/// - `fit`
/// - `fdescribe`
/// - `ftest`
///
/// Based on this [article]
///
/// [article]: https://craftinginterpreters.com/scanning-on-demand.html#tries-and-state-machines
fn contains_a_test_pattern(callee: JsAnyExpression) -> SyntaxResult<bool> {
    let mut members = CalleeNamesIterator::new(callee);

    let texts: [Option<SyntaxTokenText>; 5] = [
        members.next(),
        members.next(),
        members.next(),
        members.next(),
        members.next(),
    ];

    let mut rev = texts.iter().rev().flatten();

    let first = rev.next().map(|t| t.text());
    let second = rev.next().map(|t| t.text());
    let third = rev.next().map(|t| t.text());
    let fourth = rev.next().map(|t| t.text());
    let fifth = rev.next().map(|t| t.text());

    Ok(match first {
        Some("it" | "describe") => match second {
            None => true,
            Some("only" | "skip") => third.is_none(),
            _ => false,
        },
        Some("test") => match second {
            None => true,
            Some("only" | "skip" | "step") => third.is_none(),
            Some("describe") => match third {
                None => true,
                Some("only") => true,
                Some("parallel" | "serial") => match fourth {
                    None => true,
                    Some("only") => fifth.is_none(),
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        },
        Some("skip" | "xit" | "xdescribe" | "xtest" | "fit" | "fdescribe" | "ftest") => true,
        _ => false,
    })
}

/// Iterator that returns the callee names in "top down order"
///
/// # Examples
///
/// ```javascript
/// it.only() -> [`only`, `it`]
/// ```
struct CalleeNamesIterator {
    next: Option<JsAnyExpression>,
}

impl CalleeNamesIterator {
    fn new(callee: JsAnyExpression) -> Self {
        Self {
            next: Some(callee.into()),
        }
    }
}

impl Iterator for CalleeNamesIterator {
    type Item = SyntaxTokenText;

    fn next(&mut self) -> Option<Self::Item> {
        use JsAnyExpression::*;

        let current = self.next.take()?;

        match current {
            JsIdentifierExpression(identifier) => identifier
                .name()
                .and_then(|reference| reference.value_token())
                .ok()
                .map(|value| value.token_text_trimmed()),
            JsStaticMemberExpression(member_expression) => match member_expression.member() {
                Ok(JsAnyName::JsName(name)) => {
                    self.next = member_expression.object().ok();
                    name.value_token()
                        .ok()
                        .map(|name| name.token_text_trimmed())
                }
                _ => None,
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::contains_a_test_pattern;
    use rome_diagnostics::file::FileId;
    use rome_js_parser::parse;
    use rome_js_syntax::{JsCallExpression, SourceType};
    use rome_rowan::AstNodeList;

    fn extract_call_expression(src: &str) -> JsCallExpression {
        let source_type = SourceType::js_module();
        let result = parse(src, FileId::zero(), source_type);
        let module = result
            .tree()
            .as_js_module()
            .unwrap()
            .items()
            .first()
            .unwrap();

        module
            .as_js_any_statement()
            .unwrap()
            .as_js_expression_statement()
            .unwrap()
            .expression()
            .unwrap()
            .as_js_call_expression()
            .unwrap()
            .clone()
    }

    #[test]
    fn matches_simple_call() {
        let call_expression = extract_call_expression("test();");
        assert_eq!(
            contains_a_test_pattern(call_expression.callee().unwrap()),
            Ok(true)
        );

        let call_expression = extract_call_expression("it();");
        assert_eq!(
            contains_a_test_pattern(call_expression.callee().unwrap()),
            Ok(true)
        );
    }

    #[test]
    fn matches_static_member_expression() {
        let call_expression = extract_call_expression("test.only();");
        assert_eq!(
            contains_a_test_pattern(call_expression.callee().unwrap()),
            Ok(true)
        );
    }

    #[test]
    fn matches_static_member_expression_deep() {
        let call_expression = extract_call_expression("test.describe.parallel.only();");
        assert_eq!(
            contains_a_test_pattern(call_expression.callee().unwrap()),
            Ok(true)
        );
    }

    #[test]
    fn doesnt_static_member_expression_deep() {
        let call_expression = extract_call_expression("test.describe.parallel.only.AHAHA();");
        assert_eq!(
            contains_a_test_pattern(call_expression.callee().unwrap()),
            Ok(false)
        );
    }
}
