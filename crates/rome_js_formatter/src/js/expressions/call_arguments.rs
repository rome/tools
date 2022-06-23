use crate::builders::{format_close_delimiter, format_open_delimiter};
use crate::prelude::*;
use crate::utils::{is_call_like_expression, write_arguments_multi_line};
use rome_formatter::{format_args, write, FormatRuleWithOptions};
use rome_js_syntax::{
    JsAnyCallArgument, JsAnyExpression, JsAnyFunctionBody, JsAnyLiteralExpression, JsAnyName,
    JsAnyStatement, JsArrayExpression, JsArrowFunctionExpression, JsCallArgumentList,
    JsCallArguments, JsCallArgumentsFields, JsLanguage, JsSyntaxKind, TsReferenceType,
};
use rome_rowan::{AstSeparatedElement, AstSeparatedList, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub struct FormatJsCallArguments {
    parent_callee: Option<JsAnyExpression>,
}

impl FormatRuleWithOptions<JsCallArguments> for FormatJsCallArguments {
    type Options = Option<JsAnyExpression>;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.parent_callee = options;
        self
    }
}

impl FormatNodeRule<JsCallArguments> for FormatJsCallArguments {
    fn fmt_fields(&self, node: &JsCallArguments, f: &mut JsFormatter) -> FormatResult<()> {
        let JsCallArgumentsFields {
            l_paren_token,
            args,
            r_paren_token,
        } = node.as_fields();

        let l_paren_token = l_paren_token?;
        let r_paren_token = r_paren_token?;

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

        if let Some(callee) = &self.parent_callee {
            if is_framework_test_call(&args, callee)? {
                return write!(
                    f,
                    [
                        l_paren_token.format(),
                        format_with(|f| {
                            let separated =
                                args.format_separated(JsSyntaxKind::COMMA).with_options(
                                    FormatSeparatedOptions::default()
                                        .with_trailing_separator(TrailingSeparator::Omit),
                                );
                            f.join_with(space_token()).entries(separated).finish()
                        }),
                        r_paren_token.format()
                    ]
                );
            }
        }

        // particular formatting for hooks
        if let Some((first_argument, second_argument)) = is_react_hook_with_deps_array(&args)? {
            write!(
                f,
                [
                    l_paren_token.format(),
                    first_argument.node().format(),
                    first_argument.trailing_separator().format(),
                    space_token(),
                    second_argument.node().format(),
                ]
            )?;

            // we don't want to print the trailing separator, so if it's present, we remove it
            if let Some(separator) = second_argument.trailing_separator()? {
                write!(f, [format_removed(separator)])?;
            }

            return write!(f, [r_paren_token.format()]);
        }

        // we create open a close delimiters
        let open_delimiter = format_open_delimiter(&l_paren_token);
        let close_delimiter = format_close_delimiter(&r_paren_token);

        // we now extracts the formatted version of trivias and tokens of the delimiters
        // tokens on the left
        let l_leading_trivia = open_delimiter.format_leading_trivia();
        let l_paren = open_delimiter.format_token();
        let l_trailing_trivia = open_delimiter.format_trailing_trivia();

        // tokens on the right
        let r_leading_trivia = close_delimiter.format_leading_trivia();
        let r_paren = close_delimiter.format_token();
        let r_trailing_trivia = close_delimiter.format_trailing_trivia();

        let should_group_first_argument = should_group_first_argument(&args)?;
        let should_group_last_argument = should_group_last_argument(&args)?;

        // if the first or last groups needs grouping, then we prepare some special formatting
        if should_group_first_argument || should_group_last_argument {
            // We finished the "simple cases", we now need to use `best_fitting`.
            // We now need to allocate a new vector with cached nodes, this is needed because
            // we can't attempt to print the same node twice without incur in "printed token twice" errors.
            // We also disallow the trailing separator, we are interested in doing it manually.
            let separated: Vec<_> = args
                .format_separated(JsSyntaxKind::COMMA)
                .with_trailing_separator(TrailingSeparator::Omit)
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
                    // special formatting of the first element
                    let mut iter = separated.iter();
                    // SAFETY: check on the existence of at least one argument are done before
                    let first = iter.next().unwrap();
                    f.join_with(&space_token())
                        .entry(&format_with(|f| {
                            write!(f, [&format_args![first, expand_parent()]])
                        }))
                        .entries(iter)
                        .finish()?;
                } else {
                    // special formatting of the last element
                    let mut iter = separated.iter();
                    // SAFETY: check on the existence of at least one argument are done before
                    let last = iter.next_back().unwrap();

                    f.join_with(&space_token())
                        .entries(iter)
                        .entry(&format_with(|f| {
                            write!(f, [&format_args![last, expand_parent()]])
                        }))
                        .finish()?;
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
                                            write_arguments_multi_line(separated.iter(), f)
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
                            write_arguments_multi_line(separated.iter(), f)
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
                            let separated = args
                                .format_separated(JsSyntaxKind::COMMA)
                                .with_trailing_separator(TrailingSeparator::Omit)
                                .nodes_grouped();
                            write_arguments_multi_line(separated, f)
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
        && !could_group_argument(&second, false)?)
}

/// Checks if the last group requires grouping
fn should_group_last_argument(list: &JsCallArgumentList) -> SyntaxResult<bool> {
    let mut iter = list.iter().rev();
    let last = iter.next();
    let penultimate = iter.next();

    if let Some(last) = last {
        let last = last?;
        let check_with_penultimate = if let Some(penultimate) = penultimate {
            let penultimate = penultimate?;
            (last.syntax().kind() != penultimate.syntax().kind())
                && !JsArrayExpression::can_cast(penultimate.syntax().kind())
                || !JsArrowFunctionExpression::can_cast(last.syntax().kind())
        } else {
            true
        };

        Ok(!last.syntax().has_comments_direct()
            && could_group_argument(&last, false)?
            && check_with_penultimate)
    } else {
        Ok(false)
    }
}

/// Checks if the current argument could be grouped
fn could_group_argument(
    argument: &JsAnyCallArgument,
    is_arrow_recursion: bool,
) -> SyntaxResult<bool> {
    let result = if let JsAnyCallArgument::JsAnyExpression(argument) = argument {
        match argument {
            JsAnyExpression::JsObjectExpression(object_expression) => {
                object_expression.members().len() > 0
                    || object_expression
                        .syntax()
                        .first_or_last_token_have_comments()
            }

            JsAnyExpression::JsArrayExpression(array_expression) => {
                array_expression.elements().len() > 0
                    || array_expression
                        .syntax()
                        .first_or_last_token_have_comments()
            }
            JsAnyExpression::TsTypeAssertionExpression(assertion_expression) => {
                could_group_argument(
                    &JsAnyCallArgument::JsAnyExpression(assertion_expression.expression()?),
                    false,
                )?
            }

            JsAnyExpression::TsAsExpression(as_expression) => could_group_argument(
                &JsAnyCallArgument::JsAnyExpression(as_expression.expression()?),
                false,
            )?,
            JsAnyExpression::JsArrowFunctionExpression(arrow_function) => {
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
                let can_group_type = !return_type_annotation.and_then(|rty| rty.ty().ok()).map_or(
                    false,
                    |any_type| {
                        TsReferenceType::can_cast(any_type.syntax().kind())
                            || if let JsAnyFunctionBody::JsFunctionBody(function_body) = &body {
                                function_body
                                    .statements()
                                    .iter()
                                    .any(|st| matches!(st, JsAnyStatement::JsEmptyStatement(_)))
                            } else {
                                true
                            }
                    },
                );

                let body_is_delimited = matches!(
                    body,
                    JsAnyFunctionBody::JsFunctionBody(_)
                        | JsAnyFunctionBody::JsAnyExpression(JsAnyExpression::JsObjectExpression(
                            _
                        ))
                        | JsAnyFunctionBody::JsAnyExpression(JsAnyExpression::JsArrayExpression(_))
                );

                if let JsAnyFunctionBody::JsAnyExpression(any_expression) = body.clone() {
                    let is_nested_arrow_function =
                        if let JsAnyExpression::JsArrowFunctionExpression(
                            arrow_function_expression,
                        ) = &any_expression
                        {
                            arrow_function_expression
                                .body()
                                .ok()
                                .and_then(|body| body.as_js_any_expression().cloned())
                                .and_then(|body| {
                                    could_group_argument(
                                        &JsAnyCallArgument::JsAnyExpression(body),
                                        true,
                                    )
                                    .ok()
                                })
                                .unwrap_or(false)
                        } else {
                            false
                        };

                    body_is_delimited
                        && is_nested_arrow_function
                        && can_group_type
                        && (!is_arrow_recursion
                            && (is_call_like_expression(&any_expression)
                                || matches!(
                                    body,
                                    JsAnyFunctionBody::JsAnyExpression(
                                        JsAnyExpression::JsConditionalExpression(_)
                                    )
                                )))
                } else {
                    body_is_delimited && can_group_type
                }
            }

            JsAnyExpression::JsFunctionExpression(_) => true,
            _ => false,
        }
    } else {
        false
    };

    Ok(result)
}

type HookArgs = (
    AstSeparatedElement<JsLanguage, JsAnyCallArgument>,
    AstSeparatedElement<JsLanguage, JsAnyCallArgument>,
);
/// This function is used to check if the code is a hook-like code:
///
/// ```js
/// useMemo(() => {}, [])
/// ```
fn is_react_hook_with_deps_array(node: &JsCallArgumentList) -> SyntaxResult<Option<HookArgs>> {
    let result = if node.len() == 2 {
        let mut iter = node.elements();
        let first_element = iter.next().unwrap();
        let second_element = iter.next().unwrap();
        // SAFETY: covered by the previous if check
        let first_node = first_element.node()?;
        let second_node = second_element.node()?;
        let first_node_matches = if let JsAnyCallArgument::JsAnyExpression(
            JsAnyExpression::JsArrowFunctionExpression(arrow_function),
        ) = first_node
        {
            let no_parameters = arrow_function.parameters()?.is_empty();
            let body = arrow_function.body()?;
            let is_block = matches!(body, JsAnyFunctionBody::JsFunctionBody(_));

            no_parameters && is_block
        } else {
            false
        };

        let second_node_matches = matches!(second_node, JsAnyCallArgument::JsAnyExpression(_));
        let no_comments = !node.syntax().first_or_last_token_have_comments();
        if first_node_matches && second_node_matches && no_comments {
            Some((first_element, second_element))
        } else {
            None
        }
    } else {
        None
    };

    Ok(result)
}

/// This is a specialised function that checks if the current [call expression]
/// is reminds a call expression usually used by the majority of testing frameworks.
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
fn is_framework_test_call(
    list: &JsCallArgumentList,
    callee: &JsAnyExpression,
) -> SyntaxResult<bool> {
    let arguments_len = list.len();
    if arguments_len == 2 || arguments_len == 3 {
        let mut iter = list.iter();
        // safe
        let first_argument = iter.next().expect("Expected at least one argument")?;
        let first_argument_is_literal_like = matches!(
            first_argument,
            JsAnyCallArgument::JsAnyExpression(
                JsAnyExpression::JsAnyLiteralExpression(
                    JsAnyLiteralExpression::JsStringLiteralExpression(_)
                ) | JsAnyExpression::JsTemplate(_)
            )
        );

        if first_argument_is_literal_like && contains_a_test_pattern(callee)? {
            let second_argument = iter.next().expect("Expected at least two arguments")?;
            let third_argument = iter.next();
            // if the third argument is not a numeric literal, we bail
            // example: `it("name", () => { ... }, 2500)`
            if let Some(third_argument) = third_argument {
                let third_argument = third_argument?;
                if !matches!(
                    third_argument,
                    JsAnyCallArgument::JsAnyExpression(JsAnyExpression::JsAnyLiteralExpression(
                        JsAnyLiteralExpression::JsNumberLiteralExpression(_)
                    ))
                ) {
                    return Ok(false);
                }
            }

            return if arguments_len == 2 {
                Ok(matches!(
                    second_argument,
                    JsAnyCallArgument::JsAnyExpression(
                        JsAnyExpression::JsArrowFunctionExpression(_)
                            | JsAnyExpression::JsFunctionExpression(_)
                    )
                ))
            } else {
                let result = match second_argument {
                    JsAnyCallArgument::JsAnyExpression(JsAnyExpression::JsFunctionExpression(
                        node,
                    )) => {
                        let has_enough_parameters = node.parameters()?.items().len() <= 1;
                        has_enough_parameters
                    }
                    JsAnyCallArgument::JsAnyExpression(
                        JsAnyExpression::JsArrowFunctionExpression(node),
                    ) => {
                        let body = node.body()?;
                        let has_enough_parameters = node.parameters()?.len() <= 1;
                        matches!(body, JsAnyFunctionBody::JsFunctionBody(_))
                            && has_enough_parameters
                    }
                    _ => false,
                };
                Ok(result)
            };
        }
    }

    Ok(false)
}

fn contains_a_test_pattern(callee: &JsAnyExpression) -> SyntaxResult<bool> {
    const TEST_CALLEE_PATTERNS: [&str; 23] = [
        "it",
        "it.only",
        "it.skip",
        "describe",
        "describe.only",
        "describe.skip",
        "test",
        "test.only",
        "test.skip",
        "test.step",
        "test.describe",
        "test.describe.only",
        "test.describe.parallel",
        "test.describe.parallel.only",
        "test.describe.serial",
        "test.describe.serial.only",
        "skip",
        "xit",
        "xdescribe",
        "xtest",
        "fit",
        "fdescribe",
        "ftest",
    ];

    for pattern in TEST_CALLEE_PATTERNS {
        if matches_test_call(pattern, callee)? {
            return Ok(true);
        }
    }
    return Ok(false);
}

/// This is particular used to identify if a [JsCallExpression] has the shape
/// of a call argument coming from a test framework.
///
/// An example are call arguments coming from Mocha, Jest, etc.
///
/// ```js
/// describe("My component", () => {
///     it("should render", () => {
///         
///     });
/// })
///
/// test.only("", testSomething);
/// ```
///
/// This function should accept the `callee` of [JsCallExpression] and the
/// string pattern to test against. For example "test", "test.only"
fn matches_test_call(pattern: &str, callee: &JsAnyExpression) -> SyntaxResult<bool> {
    let paths = pattern.split('.');
    let mut current_node = callee.clone();
    let mut iter = paths.rev().peekable();
    while let Some(path) = iter.next() {
        let is_last = iter.peek().is_none();
        if let JsAnyExpression::JsIdentifierExpression(identifier) = &current_node {
            let same_name_slice = identifier.name()?.value_token()?.text_trimmed() == path;
            if !same_name_slice {
                return Ok(false);
            } else if is_last {
                return Ok(true);
            }
        } else if let JsAnyExpression::JsStaticMemberExpression(member_expression) = &current_node {
            match member_expression.member()? {
                JsAnyName::JsName(name) => {
                    let same_name_slice = name.value_token()?.text_trimmed() == path;
                    if same_name_slice {
                        current_node = member_expression.object()?;
                    } else {
                        return Ok(false);
                    }
                }
                _ => return Ok(false),
            };
        } else {
            return Ok(false);
        }
    }

    Ok(false)
}

#[cfg(test)]
mod test {
    use super::matches_test_call;
    use rome_js_parser::parse;
    use rome_js_syntax::{JsCallExpression, SourceType};
    use rome_rowan::AstNodeList;

    fn extract_call_expression(src: &str) -> JsCallExpression {
        let source_type = SourceType::js_module();
        let result = parse(src, 0, source_type);
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
            matches_test_call("test", &call_expression.callee().unwrap()),
            Ok(true)
        );

        assert_eq!(
            matches_test_call("testttt", &call_expression.callee().unwrap()),
            Ok(false)
        );
    }

    #[test]
    fn matches_static_member_expression() {
        let call_expression = extract_call_expression("test.only();");
        assert_eq!(
            matches_test_call("test.only", &call_expression.callee().unwrap()),
            Ok(true)
        );

        assert_eq!(
            matches_test_call("test.describe", &call_expression.callee().unwrap()),
            Ok(false)
        );
    }

    #[test]
    fn matches_static_member_expression_deep() {
        let call_expression = extract_call_expression("test.describe.only();");
        assert_eq!(
            matches_test_call("test.describe.only", &call_expression.callee().unwrap()),
            Ok(true)
        );

        assert_eq!(
            matches_test_call("test.describe", &call_expression.callee().unwrap()),
            Ok(false)
        );
    }
}
