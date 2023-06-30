use bpaf::Bpaf;
use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_deserialize::{
    json::{has_only_known_keys, VisitJsonNode},
    DeserializationDiagnostic, VisitNode,
};
use rome_js_syntax::{
    AnyJsExpression, AnyJsFunction, AnyJsFunctionBody, AnyJsStatement, JsBlockStatement,
    JsStatementList, JsSwitchCaseList,
};
use rome_json_syntax::{JsonLanguage, JsonSyntaxNode};
use rome_rowan::AstNode;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

declare_rule! {
    /// The more complex
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// Add a link to the corresponding ESLint rule (if any):
    ///
    /// Sources:
    ///
    /// * https://github.com/SonarSource/eslint-plugin-sonarjs/blob/HEAD/docs/rules/cognitive-complexity.md
    /// * https://eslint.org/docs/latest/rules/complexity (note this uses "cyclomatic complexity" instead)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// var a = 1;
    /// ```
    ///
    pub(crate) NoExcessiveComplexity {
        version: "next",
        name: "noExcessiveComplexity",
        recommended: false,
    }
}

impl Rule for NoExcessiveComplexity {
    type Query = Ast<AnyJsFunction>;
    type State = ComplexityScore;
    type Signals = Vec<Self::State>;
    type Options = ComplexityOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let Ok(body) = ctx.query().body() else {
            return Vec::new();
        };

        let calculated_score = calculate_cognitive_complexity(&body);
        if calculated_score > ctx.options().max_allowed_complexity {
            vec![ComplexityScore { calculated_score }]
        } else {
            Vec::new()
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, score: &Self::State) -> Option<RuleDiagnostic> {
        let ComplexityScore { calculated_score } = score;
        let ComplexityOptions {
            max_allowed_complexity,
        } = ctx.options();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                match ctx.query().id() {
                    Ok(Some(id)) => id.range(),
                    // This `unwrap()` is safe because we know there is a body,
                    // otherwise `run()` wouldn't have signalled anything.
                    _ => ctx.query().body().unwrap().range()
                },
                markup!("Excessive complexity detected."),
            )
            .note(markup! {
                "Please refactor this code to reduce its complexity from "{calculated_score}" to "{max_allowed_complexity}"."
            }),
        )
    }
}

pub struct ComplexityScore {
    calculated_score: usize,
}

/// Options for the rule `noNestedModuleImports`.
#[derive(Deserialize, Serialize, Debug, Clone, Bpaf)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ComplexityOptions {
    /// The maximum complexity score that we allow. Anything higher is considered excessive.
    pub max_allowed_complexity: usize,
}

impl Default for ComplexityOptions {
    fn default() -> Self {
        Self {
            max_allowed_complexity: 10,
        }
    }
}

impl FromStr for ComplexityOptions {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self::default())
    }
}

impl VisitJsonNode for ComplexityOptions {}
impl VisitNode<JsonLanguage> for ComplexityOptions {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, &["maxAllowedComplexity"], diagnostics)
    }

    fn visit_map(
        &mut self,
        key: &JsonSyntaxNode,
        value: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        if name_text == "maxAllowedComplexity" {
            if let Some(value) = value
                .as_json_number_value()
                .and_then(|number_value| usize::from_str(&number_value.syntax().to_string()).ok())
                .filter(|&number| number > 0)
            {
                self.max_allowed_complexity = value;
            } else {
                diagnostics.push(DeserializationDiagnostic::new(markup! {
                    "The field "<Emphasis>"maxAllowedComplexity"</Emphasis>" must contain a positive integer"
                })
                .with_range(value.range()));
            }
        }
        Some(())
    }
}

fn calculate_cognitive_complexity(body: &AnyJsFunctionBody) -> usize {
    if let Some(body) = body.as_js_function_body() {
        calculate_for_block(&body.statements(), 0)
    } else if let Some(expression) = body.as_any_js_expression() {
        calculate_for_expression(expression, 0)
    } else {
        0
    }
}

fn calculate_for_block(statements: &JsStatementList, nesting_score: usize) -> usize {
    statements
        .into_iter()
        .map(|statement| calculate_for_statement(&statement, nesting_score))
        .sum()
}

fn calculate_for_expression(expression: &AnyJsExpression, nesting_score: usize) -> usize {
    0 // FIXME: Implement this.
}

fn calculate_for_statement(statement: &AnyJsStatement, nesting_score: usize) -> usize {
    if let Some(block) = statement.as_js_block_statement() {
        return calculate_for_block(&block.statements(), nesting_score);
    } else if let Some(_continue_statement) = statement.as_js_continue_statement() {
        return 1;
    } else if let Some(for_statement) = statement.as_js_for_statement() {
        if let Ok(body) = for_statement.body() {
            return 1 + nesting_score + calculate_for_statement(&body, nesting_score + 1);
        }
    } else if let Some(for_in_statement) = statement.as_js_for_in_statement() {
        if let Ok(body) = for_in_statement.body() {
            return 1 + nesting_score + calculate_for_statement(&body, nesting_score + 1);
        }
    } else if let Some(for_of_statement) = statement.as_js_for_of_statement() {
        if let Ok(body) = for_of_statement.body() {
            return 1 + nesting_score + calculate_for_statement(&body, nesting_score + 1);
        }
    } else if let Some(if_statement) = statement.as_js_if_statement() {
        if let Ok(consequent) = if_statement.consequent() {
            return 1
                + nesting_score
                + calculate_for_statement(&consequent, nesting_score + 1)
                + if_statement
                    .else_clause()
                    .and_then(|else_clause| else_clause.alternate().ok())
                    .map(|alternate| calculate_for_statement(&alternate, nesting_score))
                    .unwrap_or_default();
        }
    } else if let Some(labeled_statement) = statement.as_js_labeled_statement() {
        if let Ok(body) = labeled_statement.body() {
            return calculate_for_statement(&body, nesting_score);
        }
    } else if let Some(switch_statement) = statement.as_js_switch_statement() {
        return 1
            + nesting_score
            + calculate_for_switch_cases(&switch_statement.cases(), nesting_score + 1);
    } else if let Some(try_statement) = statement.as_js_try_statement() {
        if let Ok(block) = try_statement.body() {
            return calculate_for_block(&block.statements(), nesting_score)
                + try_statement
                    .catch_clause()
                    .and_then(|catch_clause| catch_clause.body())
                    .map(|catch| {
                        1 + nesting_score
                            + calculate_for_block(&catch.statements(), nesting_score + 1)
                    })
                    .unwrap_or_default();
        }
    }

    0
}

fn calculate_for_switch_cases(cases: &JsSwitchCaseList, nesting_score: usize) -> usize {
    cases
        .into_iter()
        .map(|case| calculate_for_block(&case.consequent(), nesting_score))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::calculate_cognitive_complexity;
    use rome_js_parser::{parse_script, JsParserOptions};
    use rome_js_syntax::AnyJsFunctionBody;
    use rome_rowan::AstNodeList;

    #[test]
    fn test_cognitive_complexity_of_simple_branches() {
        let body = parse_function(
            "function simpleBranches() {
                if (firstCondition) {                 // +1
                    return 1;
                } else if (secondCondition) {         // +1
                    return 2;
                } else {
                    return 3;
                }
            }",
        );
        assert_eq!(calculate_cognitive_complexity(&body), 2);
    }

    #[test]
    fn test_cognitive_complexity_of_sum_of_primes() {
        let body = parse_function(
            "function sumOfPrimes(max) {
                let total = 0;
                OUT: for (let i = 1; i <= max; ++i) { // +1
                    for (let j = 2; j < i; ++j) {     // +2
                        if (i % j == 0) {             // +3
                            continue OUT;             // +1
                        }
                    }
                    total += 1;
                }
                return total;
            }",
        );
        assert_eq!(calculate_cognitive_complexity(&body), 7);
    }

    #[test]
    fn test_cognitive_complexity_of_get_words() {
        let body = parse_function(
            r#"function getWords(num) {
                switch (num) {                        // +1
                    case 1:
                        return "one";
                    case 2:
                        return "a couple";
                    case 3:
                        return "a few";
                    default:
                        return "lots";
                }
            }"#,
        );
        assert_eq!(calculate_cognitive_complexity(&body), 1);
    }

    fn parse_function(text: &str) -> AnyJsFunctionBody {
        let node = parse_script(text, JsParserOptions::default())
            .tree()
            .statements()
            .first()
            .unwrap();
        let declaration = node.as_js_function_declaration().unwrap();
        let body = declaration.body().unwrap();

        AnyJsFunctionBody::JsFunctionBody(body)
    }
}
