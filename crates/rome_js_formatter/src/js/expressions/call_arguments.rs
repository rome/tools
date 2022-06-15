use crate::builders::{format_close_delimiter, format_open_delimiter};
use crate::prelude::*;
use crate::utils::{fmt_arguments_multi_line, is_call_like_expression};
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsSyntaxKind::JS_EMPTY_STATEMENT;
use rome_js_syntax::{
    JsAnyCallArgument, JsAnyExpression, JsAnyFunctionBody, JsArrayExpression,
    JsArrowFunctionExpression, JsCallArgumentList, JsCallArguments, JsCallArgumentsFields,
    JsConditionalExpression, JsFunctionBody, JsFunctionExpression, JsObjectExpression,
    JsSyntaxNode, TsAsExpression, TsReferenceType, TsTypeAssertionExpression,
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

        // we now extracts the formatted version of trivias and tokens of the delimiters

        if args.len() == 0 {
            return write!(
                f,
                [
                    l_paren_token.format(),
                    args.format(),
                    r_paren_token.format()
                ]
            );
        }

        // particular formatting for hooks
        if is_react_hook_with_deps_array(&args)? {
            let mut list = args.elements();
            // SAFETY: the function `is_react_hook_with_deps_array` already checks the presence of the
            // first two arguments, so it's safe un unwrap them
            let first_argument = list.next().unwrap();
            let second_argument = list.next().unwrap();

            return write!(
                f,
                [
                    l_paren_token.format(),
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
                    r_paren_token.format()
                ]
            );
        }

        // we create open a close delimiters
        let open_delimiter = format_open_delimiter(&l_paren_token);
        let close_delimiter = format_close_delimiter(&r_paren_token);

        // tokens on the left
        let l_leading_trivia = open_delimiter.as_leading_trivia_fmt();
        let l_paren = open_delimiter.as_token_fmt();
        let l_trailing_trivia = open_delimiter.as_trailing_trivia_fmt();

        // tokens on the right
        let r_leading_trivia = close_delimiter.as_leading_trivia_fmt();
        let r_paren = close_delimiter.as_token_fmt();
        let r_trailing_trivia = close_delimiter.as_trailing_trivia_fmt();

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

            // We now cache them the delimiters tokens. This is needed because `[rome_formatter::best_fitting]` will try to
            // print each version first
            // tokens on the left
            let l_leading_trivia = l_leading_trivia.memoized();
            let l_paren = l_paren.memoized();
            let l_trailing_trivia = l_trailing_trivia.memoized();

            // tokens on the right
            let r_leading_trivia = r_leading_trivia.memoized();
            let r_paren = r_paren.memoized();
            let r_trailing_trivia = r_trailing_trivia.memoized();

            let edge_arguments_do_not_break = format_with(|f| {
                // `should_group_first_argument` and `should_group_last_argument` are mutually exclusive
                // which means that if one is `false`, then the other is `true`.
                // This means that in this branch we format the case where `should_group_first_argument`,
                // in the else branch we format the case where `should_group_last_argument` is `true`.

                write!(f, [l_leading_trivia, l_paren, l_trailing_trivia,])?;
                if should_group_first_argument {
                    write!(
                        f,
                        [format_with(|f| {
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
                        }),]
                    )?;
                } else {
                    write!(
                        f,
                        [format_with(|f| {
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
                        }),]
                    )?;
                }

                write!(f, [r_leading_trivia, r_paren, r_trailing_trivia])
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
                        &group_elements(&format_with(|f| {
                            write!(
                                f,
                                [
                                    &soft_block_indent(&format_args![
                                        &l_trailing_trivia,
                                        format_with(|f| {
                                            fmt_arguments_multi_line(separated.iter(), f)
                                        }),
                                        &r_leading_trivia,
                                        soft_line_break()
                                    ]),
                                    &r_paren
                                ]
                            )
                        })),
                        &r_trailing_trivia
                    ]
                )
            });

            write!(
                f,
                [best_fitting![
                    format_args![
                        l_leading_trivia,
                        l_paren,
                        l_trailing_trivia,
                        group_elements(&format_args![format_with(|f| {
                            fmt_arguments_multi_line(separated.iter(), f)
                        })]),
                        r_leading_trivia,
                        r_paren,
                        r_trailing_trivia
                    ],
                    edge_arguments_do_not_break,
                    all_arguments_expanded
                ]]
            )
        } else {
            write!(
                f,
                [
                    l_leading_trivia,
                    &group_elements(&format_args![
                        l_paren,
                        l_trailing_trivia,
                        &soft_block_indent(&format_with(|f| {
                            let separated = args.format_separated(token(",")).with_options(
                                FormatSeparatedOptions::default()
                                    .with_trailing_separator(TrailingSeparator::Elide),
                            );
                            fmt_arguments_multi_line(separated, f)
                        }),),
                        r_leading_trivia,
                        r_paren,
                    ],),
                    r_trailing_trivia
                ]
            )
        }
    }
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
                matches!(arrow.body()?, JsAnyFunctionBody::JsFunctionBody(_))
            }
            _ => false,
        }
    } else {
        false
    };

    let second_arg_is_function_like = matches!(
        second,
        JsAnyCallArgument::JsAnyExpression(
            JsAnyExpression::JsFunctionExpression(_)
                | JsAnyExpression::JsArrowFunctionExpression(_)
                | JsAnyExpression::JsConditionalExpression(_)
        )
    );
    Ok(!has_comments
        && is_function_like
        && !second_arg_is_function_like
        && !could_group_argument(second.syntax(), false)?)
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
            && could_group_argument(last.syntax(), false)?
            && check_with_penultimate)
    } else {
        Ok(false)
    }
}

/// Checks if the current argument could be grouped
fn could_group_argument(argument: &JsSyntaxNode, is_arrow_recursion: bool) -> SyntaxResult<bool> {
    let result = if let Some(object_expression) = JsObjectExpression::cast(argument.clone()) {
        object_expression.members().len() > 0
            || object_expression.syntax().has_comments_at_the_edges()
    } else if let Some(array_expression) = JsArrayExpression::cast(argument.clone()) {
        array_expression.elements().len() > 0
            || array_expression.syntax().has_comments_at_the_edges()
    } else if let Some(assertion_expression) = TsTypeAssertionExpression::cast(argument.clone()) {
        could_group_argument(assertion_expression.expression()?.syntax(), false)?
    } else if let Some(as_expression) = TsAsExpression::cast(argument.clone()) {
        could_group_argument(as_expression.expression()?.syntax(), false)?
    } else if let Some(arrow_function) = JsArrowFunctionExpression::cast(argument.clone()) {
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
            !return_type_annotation
                .and_then(|rty| rty.ty().ok())
                .map_or(false, |any_type| {
                    TsReferenceType::can_cast(any_type.syntax().kind())
                        || JsFunctionBody::cast(body.syntax().clone()).map_or(
                            true,
                            |function_body| {
                                function_body
                                    .statements()
                                    .iter()
                                    .any(|st| st.syntax().kind() == JS_EMPTY_STATEMENT)
                            },
                        )
                });

        let body_is_delimited = matches!(
            body,
            JsAnyFunctionBody::JsFunctionBody(_)
                | JsAnyFunctionBody::JsAnyExpression(JsAnyExpression::JsObjectExpression(_))
                | JsAnyFunctionBody::JsAnyExpression(JsAnyExpression::JsArrayExpression(_))
        );

        if let JsAnyFunctionBody::JsAnyExpression(any_expression) = body.clone() {
            let is_nested_arrow_function =
                JsArrowFunctionExpression::cast(any_expression.syntax().clone())
                    .and_then(|arrow_function_expression| {
                        arrow_function_expression
                            .body()
                            .ok()
                            .and_then(|body| could_group_argument(body.syntax(), true).ok())
                    })
                    .unwrap_or(false);

            body_is_delimited
                && is_nested_arrow_function
                && can_group_type
                && (!is_arrow_recursion
                    && (is_call_like_expression(&any_expression)
                        || JsConditionalExpression::can_cast(body.syntax().kind())))
        } else {
            body_is_delimited && can_group_type
        }
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
    let result = if node.len() == 2 {
        let mut iter = node.elements();
        // SAFETY: covered by the previous if check
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
