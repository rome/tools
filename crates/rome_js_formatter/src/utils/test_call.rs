use crate::prelude::*;
use rome_js_syntax::{
    JsAnyArrowFunctionParameters, JsAnyCallArgument, JsAnyExpression, JsAnyFunctionBody,
    JsAnyLiteralExpression, JsAnyName, JsCallArgumentList, JsCallArguments, JsCallExpression,
    JsSyntaxNode,
};
use rome_rowan::{SyntaxResult, SyntaxTokenText};

/// Returns `Ok(true)` if `maybe_argument` is an argument of a [test call expression](is_test_call_expression).
pub(crate) fn is_test_call_argument(maybe_argument: &JsSyntaxNode) -> SyntaxResult<bool> {
    let call_expression = maybe_argument
        .parent()
        .and_then(JsCallArgumentList::cast)
        .and_then(|args| args.syntax().grand_parent())
        .and_then(JsCallExpression::cast);

    call_expression.map_or(Ok(false), |call| is_test_call_expression(&call))
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
        ) if arguments.args().len() <= 3 && contains_a_test_pattern(&callee)? => {
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
                .map_or(false, is_angular_test_wrapper)
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

/// Tests if the callee is a `beforeEach`, `beforeAll`, `afterEach` or `afterAll` identifier
/// that is commonly used in test frameworks.
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
fn contains_a_test_pattern(callee: &JsAnyExpression) -> SyntaxResult<bool> {
    let mut members = CalleeNamesIterator::new(callee.clone());

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

/// Iterator that returns the callee names in "top down order".
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
        Self { next: Some(callee) }
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
            contains_a_test_pattern(&call_expression.callee().unwrap()),
            Ok(true)
        );

        let call_expression = extract_call_expression("it();");
        assert_eq!(
            contains_a_test_pattern(&call_expression.callee().unwrap()),
            Ok(true)
        );
    }

    #[test]
    fn matches_static_member_expression() {
        let call_expression = extract_call_expression("test.only();");
        assert_eq!(
            contains_a_test_pattern(&call_expression.callee().unwrap()),
            Ok(true)
        );
    }

    #[test]
    fn matches_static_member_expression_deep() {
        let call_expression = extract_call_expression("test.describe.parallel.only();");
        assert_eq!(
            contains_a_test_pattern(&call_expression.callee().unwrap()),
            Ok(true)
        );
    }

    #[test]
    fn doesnt_static_member_expression_deep() {
        let call_expression = extract_call_expression("test.describe.parallel.only.AHAHA();");
        assert_eq!(
            contains_a_test_pattern(&call_expression.callee().unwrap()),
            Ok(false)
        );
    }
}
