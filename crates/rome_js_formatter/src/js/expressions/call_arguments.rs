use crate::builders::{format_close_delimiter, format_open_delimiter};
use crate::prelude::*;
use crate::utils::{format_separated_for_call_arguments, is_simple_expression, token_has_comments};
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::{
    JsAnyCallArgument, JsAnyExpression, JsAnyFunctionBody, JsArrayExpression,
    JsArrowFunctionExpression, JsCallArgumentList, JsCallArguments, JsCallArgumentsFields,
    JsFunctionExpression, JsObjectExpression, JsSyntaxNode, TsAsExpression,
    TsTypeAssertionExpression,
};
use rome_rowan::{AstSeparatedList, SyntaxResult};

impl FormatNodeFields<JsCallArguments> for FormatNodeRule<JsCallArguments> {
    fn fmt_fields(node: &JsCallArguments, f: &mut JsFormatter) -> FormatResult<()> {
        let JsCallArgumentsFields {
            l_paren_token,
            args,
            r_paren_token,
        } = node.as_fields();

        let l_paren_token = l_paren_token?;
        let r_paren_token = r_paren_token?;

        // we create open a close delimiters
        let open_delimiter = format_open_delimiter(&l_paren_token);
        let close_delimiter = format_close_delimiter(&r_paren_token);

        // we now extracts the formatted version of trivias and tokens of the delimiters and
        // we cache them. This is needed because `[rome_formatter::best_fitting]` will try to
        // print each version first

        // tokens on the left
        let l_leading_trivia = open_delimiter.as_leading_trivia_fmt().memoized();
        let l_paren = open_delimiter.as_token_fmt().memoized();
        let l_trailing_trivia = open_delimiter.as_trailing_trivia_fmt().memoized();

        // tokens on the right
        let r_leading_trivia = close_delimiter.as_leading_trivia_fmt().memoized();
        let r_paren = close_delimiter.as_token_fmt().memoized();
        let r_trailing_trivia = close_delimiter.as_trailing_trivia_fmt().memoized();

        // particular formatting for hooks
        if is_react_hook_with_deps_array(&args)? {
            let mut list = args.elements();
            // SAFETY: the function `is_react_hook_with_deps_array` already checks the presence of the
            // first two arguments, so it's safe un unwrap them
            let first_argument = list.next().unwrap();
            let second_argument = list.next().unwrap();

            // SAFETY: function is_react_hook_with_deps_array checks if there aren't any
            // comments. If there are comments, we don't fall in this branch of the condition,
            // so it's safe to not print them
            return write!(
                f,
                [
                    &l_paren,
                    first_argument.node().format(),
                    first_argument.trailing_separator().format(),
                    space_token(),
                    second_argument.node().format(),
                    format_with(|f| {
                        // we don't want to print the trailing separator, so if it's present, we replace it
                        // with an empty element
                        if let Some(separator) = second_argument.trailing_separator()? {
                            return write!(f, [format_replaced(separator, &empty_element())]);
                        }

                        Ok(())
                    }),
                    &r_paren
                ]
            );
        }

        let should_group_first_argument = should_group_first_argument(&args)?;
        let should_group_last_argument = should_group_last_argument(&args)?;

        // if the first or last groups needs grouping, then we prepare some special formatting
        if should_group_first_argument || should_group_last_argument {
            // We finished the "simple cases", we now need to use `best_fitting`.
            // We now need to allocate a new vector with cached nodes, this is needed because
            // we can't attempt to print the same node twice without incur in "printed token twice" errors.
            // We also disallow the trailing separator, we are interested in doing it manually.
            let separated: Vec<_> = args
                .format_separated(token(","))
                .with_options(
                    FormatSeparatedOptions::default()
                        .with_trailing_separator(TrailingSeparator::Elide),
                )
                .map(|e| e.memoized())
                .collect();

            let formatted = format_with(|f| {
                // `should_group_first_argument` and `should_group_last_argument` are mutually exclusive
                // which means that if one is `false`, then the other is `true`.
                // This means that in this branch we format the case where `should_group_first_argument`,
                // in the else branch we format the case where `should_group_last_argument` is `true`.
                if should_group_first_argument {
                    write!(
                        f,
                        [
                            l_leading_trivia,
                            l_paren,
                            l_trailing_trivia,
                            format_with(|f| {
                                // special formatting of the first element
                                let mut iter = separated.iter();
                                // SAFETY: check on the existence of at least one argument are done before
                                let first = iter.next().unwrap();
                                f.join_with(&space_token())
                                    .entry(&format_with(|f| {
                                        write!(
                                            f,
                                            [group_elements(&format_args![first, expand_parent()])]
                                        )
                                    }))
                                    .entries(iter)
                                    .finish()
                            }),
                            r_leading_trivia,
                            r_paren,
                            r_trailing_trivia
                        ]
                    )
                } else {
                    write!(
                        f,
                        [
                            l_leading_trivia,
                            l_paren,
                            l_trailing_trivia,
                            format_with(|f| {
                                // special formatting of the last element
                                let mut iter = separated.iter();
                                // SAFETY: check on the existence of at least one argument are done before
                                let last = iter.next_back().unwrap();

                                f.join_with(&space_token())
                                    .entries(iter)
                                    .entry(&format_with(|f| {
                                        write!(
                                            f,
                                            [group_elements(&format_args![last, expand_parent()])]
                                        )
                                    }))
                                    .finish()
                            }),
                            r_leading_trivia,
                            r_paren,
                            r_trailing_trivia
                        ]
                    )
                }
            });

            // This is the version of where all the arguments are broken out
            let all_arguments_expanded = format_with(|f| {
                // this formatting structure replicates what we have inside the `format_delimited`
                // function, but here we use a different way to print the trailing separator
                write!(
                    f,
                    [
                        &l_leading_trivia,
                        &l_paren,
                        &group_elements(&format_args![format_with(|f| {
                            write!(
                                f,
                                [
                                    &soft_block_indent(&format_args![
                                        &l_trailing_trivia,
                                        format_with(|f| {
                                            format_separated_for_call_arguments(
                                                separated.iter(),
                                                separated.len(),
                                                f,
                                                false,
                                            )
                                        }),
                                        r_leading_trivia,
                                        soft_line_break()
                                    ]),
                                    &r_paren
                                ]
                            )
                        })]),
                        &r_trailing_trivia
                    ]
                )
            })
            .memoized();

            write!(
                f,
                [best_fitting![
                    format_args![
                        l_leading_trivia,
                        l_paren,
                        l_trailing_trivia,
                        group_elements(&format_args![format_with(|f| {
                            format_separated_for_call_arguments(
                                separated.iter(),
                                separated.len(),
                                f,
                                false,
                            )
                        })]),
                        r_leading_trivia,
                        r_paren,
                        r_trailing_trivia
                    ],
                    format_args![formatted],
                    format_args![all_arguments_expanded]
                ]]
            )
        } else {
            let separated: Vec<_> = args
                .format_separated(token(","))
                .with_options(
                    FormatSeparatedOptions::default()
                        .with_trailing_separator(TrailingSeparator::Elide),
                )
                .map(|e| e.memoized())
                .collect();
            write!(
                f,
                [group_elements(&format_args![
                    l_leading_trivia,
                    l_paren,
                    l_trailing_trivia,
                    &if_group_breaks(&format_args![
                        indent(&format_args![
                            soft_line_break(),
                            &format_with(|f| {
                                format_separated_for_call_arguments(
                                    separated.iter(),
                                    args.len(),
                                    f,
                                    false,
                                )
                            }),
                        ]),
                        soft_line_break(),
                    ]),
                    &if_group_fits_on_line(&format_args![&format_with(|f| {
                        format_separated_for_call_arguments(separated.iter(), args.len(), f, true)
                    }),]),
                    r_leading_trivia,
                    r_paren,
                    r_trailing_trivia
                ]),]
            )
        }
    }
}

/// Returns true if the passed [JsCallArguments] has a single argument
/// that is a simple function expression, array expression or object expression
fn is_simple_function_arguments(node: &JsCallArguments) -> SyntaxResult<bool> {
    let JsCallArgumentsFields {
        l_paren_token,
        args,
        r_paren_token,
    } = node.as_fields();

    if token_has_comments(&l_paren_token?) || token_has_comments(&r_paren_token?) {
        return Ok(false);
    }

    if args.len() > 1 {
        return Ok(false);
    }

    for item in args.elements() {
        if let Some(separator) = item.trailing_separator()? {
            if token_has_comments(separator) {
                return Ok(false);
            }
        }

        match item.node() {
            Ok(JsAnyCallArgument::JsAnyExpression(expr)) => {
                if !is_simple_expression(expr)? {
                    return Ok(false);
                }
            }
            _ => return Ok(false),
        }
    }

    Ok(true)
}

/// Checks if the the first argument requires grouping
fn should_group_first_argument(list: &JsCallArgumentList) -> SyntaxResult<bool> {
    if list.len() != 2 {
        return Ok(false);
    }
    let mut iter = list.iter();
    // SAFETY: checked at the beginning of the function
    let first = iter.next().unwrap()?;
    let second = iter.next().unwrap()?;

    let has_comments = first.syntax().has_comments_direct();

    let is_function_like = if let JsAnyCallArgument::JsAnyExpression(expression) = first {
        match expression {
            JsAnyExpression::JsFunctionExpression(_) => true,
            JsAnyExpression::JsArrowFunctionExpression(arrow) => {
                let body = arrow.body()?;
                matches!(body, JsAnyFunctionBody::JsFunctionBody(_))
            }
            _ => false,
        }
    } else {
        false
    };

    let second_arg_is_function_like =
        if let JsAnyCallArgument::JsAnyExpression(ref expression) = second {
            matches!(
                expression,
                JsAnyExpression::JsFunctionExpression(_)
                    | JsAnyExpression::JsArrowFunctionExpression(_)
                    | JsAnyExpression::JsConditionalExpression(_)
            )
        } else {
            false
        };

    Ok(!has_comments
        && is_function_like
        && !second_arg_is_function_like
        && !can_group_argument(second.syntax())?)
}

/// Checks if the last group requires grouping
fn should_group_last_argument(list: &JsCallArgumentList) -> SyntaxResult<bool> {
    let mut iter = list.iter().rev();
    // SAFETY: checked at the beginning of the function
    let last = iter.next();
    let penultimate = iter.next();

    if let Some(last) = last {
        // SAFETY: guaranteed by the syntax factory
        let last = last.unwrap();
        let check_with_penultimate = if let Some(penultimate) = penultimate {
            // SAFETY: guaranteed by the syntax factory
            let penultimate = penultimate.unwrap();
            (last.syntax().kind() != penultimate.syntax().kind())
                && !JsArrayExpression::can_cast(penultimate.syntax().kind())
                || !JsArrowFunctionExpression::can_cast(last.syntax().kind())
        } else {
            true
        };

        Ok(!last.syntax().has_comments_direct()
            && can_group_argument(last.syntax())?
            && check_with_penultimate)
    } else {
        Ok(false)
    }
}

/// Checks if the current argument requires grouping.
fn can_group_argument(argument: &JsSyntaxNode) -> SyntaxResult<bool> {
    let result = if let Some(object_expression) = JsObjectExpression::cast(argument.clone()) {
        object_expression.members().len() > 0
            || object_expression.syntax().has_comments_at_the_edges()
    } else if let Some(array_expression) = JsArrayExpression::cast(argument.clone()) {
        array_expression.elements().len() > 0
            || array_expression.syntax().has_comments_at_the_edges()
    } else if let Some(assertion_expression) = TsTypeAssertionExpression::cast(argument.clone()) {
        can_group_argument(assertion_expression.expression()?.syntax())?
    } else if let Some(as_expression) = TsAsExpression::cast(argument.clone()) {
        can_group_argument(as_expression.expression()?.syntax())?
    } else {
        JsFunctionExpression::can_cast(argument.kind())
    };

    Ok(result)
}

/// This function is used to check if the code is a hook-like code:
///
/// ```js
/// useMemo(() => {}, [])
/// ```
fn is_react_hook_with_deps_array(node: &JsCallArgumentList) -> SyntaxResult<bool> {
    let enough_arguments = node.len() == 2;
    let result = if enough_arguments {
        let mut iter = node.elements();
        // SAFETY: covered by enough_arguments
        let first = iter.next().unwrap().into_node()?;
        let second = iter.next().unwrap().into_node()?;
        let first_node_matches = if let JsAnyCallArgument::JsAnyExpression(
            JsAnyExpression::JsArrowFunctionExpression(arrow_function),
        ) = first
        {
            let no_parameters = arrow_function.parameters()?.len() == 0;
            let body = arrow_function.body()?;
            let is_block = matches!(body, JsAnyFunctionBody::JsFunctionBody(_));

            no_parameters && is_block
        } else {
            false
        };

        let second_node_matches = JsArrayExpression::can_cast(second.syntax().kind());

        let no_comments = !node.syntax().has_comments_at_the_edges();

        first_node_matches && second_node_matches && no_comments
    } else {
        false
    };

    Ok(result)
}
