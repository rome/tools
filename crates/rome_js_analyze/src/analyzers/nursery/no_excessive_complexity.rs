use bpaf::Bpaf;
use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_deserialize::{
    json::{has_only_known_keys, VisitJsonNode},
    DeserializationDiagnostic, VisitNode,
};
use rome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsForInitializer, AnyJsFunction, AnyJsFunctionBody,
    AnyJsObjectMember, AnyJsStatement, AnyJsTemplateElement, AnyJsxAttribute, AnyJsxAttributeValue,
    AnyJsxChild, AnyJsxTag, JsCallArguments, JsConditionalExpression, JsIfStatement,
    JsLogicalOperator, JsStatementList, JsSwitchCaseList, JsVariableDeclaration, JsxAttributeList,
    JsxChildList, JsxElement,
};
use rome_json_syntax::{JsonLanguage, JsonSyntaxNode};
use rome_rowan::{AstNode, SyntaxResult};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

declare_rule! {
    /// The more complexity a function contains, the harder it is to understand
    /// later on.
    ///
    /// Reducing complexity helps to make code more maintenable, both by making
    /// it easier to understand as well as by reducing chances of accidental
    /// side-effects when making changes.
    ///
    /// This rule calculates a complexity score for every function and signals
    /// those that exceed a configured complexity threshold (default: 10).
    ///
    /// Sources:
    ///
    /// * https://github.com/SonarSource/eslint-plugin-sonarjs/blob/HEAD/docs/rules/cognitive-complexity.md
    /// * https://eslint.org/docs/latest/rules/complexity (note this rule uses "cyclomatic complexity" instead)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function tooComplex() {
    ///     for (let x = 0; x < 10; x++) {
    ///         for (let y = 0; y < 10; y++) {
    ///             if (x % 2 === 0) {
    ///                 if (y % 2 === 0) {
    ///                     console.log(x > y ? `${x} > ${y}` : `${y} > ${x}`);
    ///                 }
    ///             }
    ///         }
    ///     }
    /// }
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
                "Please refactor this function to reduce its complexity score from "{calculated_score}" to "{max_allowed_complexity}"."
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
    calculate_for_fallible_expression(expression, nesting_score, None).unwrap_or_default()
}

fn calculate_for_fallible_expression(
    expression: &AnyJsExpression,
    nesting_score: usize,
    last_seen_logical_operator: Option<JsLogicalOperator>,
) -> SyntaxResult<usize> {
    let score = match expression {
        AnyJsExpression::AnyJsLiteralExpression(_) => 0,
        AnyJsExpression::JsArrayExpression(_) => 0,
        AnyJsExpression::JsArrowFunctionExpression(_) => 0,
        AnyJsExpression::JsAssignmentExpression(js_assignment) => {
            // TODO: calculate_for_expression(&js_assignment.left()?, nesting_score) +
            calculate_for_expression(&js_assignment.right()?, nesting_score)
        }
        AnyJsExpression::JsAwaitExpression(_) => 0,
        AnyJsExpression::JsBinaryExpression(js_binary_expression) => {
            calculate_for_expression(&js_binary_expression.left()?, nesting_score)
                + calculate_for_expression(&js_binary_expression.right()?, nesting_score)
        }
        AnyJsExpression::JsBogusExpression(_) => 0,
        AnyJsExpression::JsCallExpression(js_call_expression) => {
            calculate_for_call_arguments(&js_call_expression.arguments()?, nesting_score)
        }
        AnyJsExpression::JsClassExpression(_) => 0,
        AnyJsExpression::JsComputedMemberExpression(_) => 0,
        AnyJsExpression::JsConditionalExpression(js_conditional) => {
            calculate_for_conditional_expression(js_conditional, nesting_score)
        }
        AnyJsExpression::JsFunctionExpression(_) => 0,
        AnyJsExpression::JsIdentifierExpression(_) => 0,
        AnyJsExpression::JsImportCallExpression(_) => 0,
        AnyJsExpression::JsImportMetaExpression(_) => 0,
        AnyJsExpression::JsInExpression(_) => 0,
        AnyJsExpression::JsInstanceofExpression(_) => 0,
        AnyJsExpression::JsLogicalExpression(js_logical_expression) => {
            let operator = js_logical_expression.operator()?;
            let penalty = if last_seen_logical_operator == Some(operator) {
                0
            } else {
                1
            };

            penalty
                + calculate_for_fallible_expression(
                    &js_logical_expression.left()?,
                    nesting_score,
                    Some(operator),
                )?
                + calculate_for_fallible_expression(
                    &js_logical_expression.right()?,
                    nesting_score,
                    Some(operator),
                )?
        }
        AnyJsExpression::JsNewExpression(js_new) => {
            calculate_for_expression(&js_new.callee()?, nesting_score)
                + js_new
                    .arguments()
                    .map(|arguments| calculate_for_call_arguments(&arguments, nesting_score))
                    .unwrap_or_default()
        }
        AnyJsExpression::JsNewTargetExpression(_) => 0,
        AnyJsExpression::JsObjectExpression(js_object_expression) => js_object_expression
            .members()
            .into_iter()
            .filter_map(Result::ok)
            .map(|member| -> SyntaxResult<usize> {
                let score = match member {
                    AnyJsObjectMember::JsBogusMember(_) => 0,
                    AnyJsObjectMember::JsGetterObjectMember(_) => 0,
                    AnyJsObjectMember::JsMethodObjectMember(_) => 0,
                    AnyJsObjectMember::JsPropertyObjectMember(js_property_object_member) => {
                        calculate_for_expression(&js_property_object_member.value()?, nesting_score)
                    }
                    AnyJsObjectMember::JsSetterObjectMember(_) => 0,
                    AnyJsObjectMember::JsShorthandPropertyObjectMember(_) => 0,
                    AnyJsObjectMember::JsSpread(js_spread) => {
                        calculate_for_expression(&js_spread.argument()?, nesting_score)
                    }
                };

                Ok(score)
            })
            .filter_map(Result::ok)
            .sum(),
        AnyJsExpression::JsParenthesizedExpression(js_parenthesized_expression) => {
            calculate_for_expression(&js_parenthesized_expression.expression()?, nesting_score)
        }
        AnyJsExpression::JsPostUpdateExpression(_) => 0,
        AnyJsExpression::JsPreUpdateExpression(_) => 0,
        AnyJsExpression::JsSequenceExpression(_) => 0,
        AnyJsExpression::JsStaticMemberExpression(_) => 0,
        AnyJsExpression::JsSuperExpression(_) => 0,
        AnyJsExpression::JsTemplateExpression(js_template) => {
            js_template
                .tag()
                .map(|tag| calculate_for_expression(&tag, nesting_score))
                .unwrap_or_default()
                + js_template
                    .elements()
                    .into_iter()
                    .map(|element| match element {
                        AnyJsTemplateElement::JsTemplateChunkElement(_) => 0,
                        AnyJsTemplateElement::JsTemplateElement(js_template_element) => {
                            js_template_element
                                .expression()
                                .map(|expression| {
                                    calculate_for_expression(&expression, nesting_score)
                                })
                                .unwrap_or_default()
                        }
                    })
                    .sum::<usize>()
        }
        AnyJsExpression::JsThisExpression(_) => 0,
        AnyJsExpression::JsUnaryExpression(js_unary_expression) => {
            calculate_for_expression(&js_unary_expression.argument()?, nesting_score)
        }
        AnyJsExpression::JsYieldExpression(js_yield) => js_yield
            .argument()
            .and_then(|argument| argument.expression().ok())
            .map(|expression| calculate_for_expression(&expression, nesting_score))
            .unwrap_or_default(),
        AnyJsExpression::JsxTagExpression(jsx_tag) => {
            calculate_for_jsx_tag(&jsx_tag.tag()?, nesting_score)
        }
        AnyJsExpression::TsAsExpression(ts_as) => {
            calculate_for_expression(&ts_as.expression()?, nesting_score)
        }
        AnyJsExpression::TsInstantiationExpression(_) => 0,
        AnyJsExpression::TsNonNullAssertionExpression(_) => 0,
        AnyJsExpression::TsSatisfiesExpression(_) => 0,
        AnyJsExpression::TsTypeAssertionExpression(_) => 0,
    };

    Ok(score)
}

fn calculate_for_call_arguments(arguments: &JsCallArguments, nesting_score: usize) -> usize {
    arguments
        .args()
        .into_iter()
        .filter_map(Result::ok)
        .map(|arg| match arg {
            AnyJsCallArgument::AnyJsExpression(expression) => {
                calculate_for_expression(&expression, nesting_score)
            }
            AnyJsCallArgument::JsSpread(spread) => spread
                .argument()
                .map(|arg| calculate_for_expression(&arg, nesting_score))
                .unwrap_or_default(),
        })
        .sum()
}

fn calculate_for_conditional_expression(
    js_conditional: &JsConditionalExpression,
    nesting_score: usize,
) -> usize {
    calculate_for_fallible_conditional_expression(js_conditional, nesting_score).unwrap_or_default()
}

fn calculate_for_fallible_conditional_expression(
    js_conditional: &JsConditionalExpression,
    nesting_score: usize,
) -> SyntaxResult<usize> {
    let score = 1
        + nesting_score
        + calculate_for_expression(&js_conditional.test()?, nesting_score)
        + calculate_for_expression(&js_conditional.consequent()?, nesting_score + 1)
        + match js_conditional.alternate()? {
            AnyJsExpression::JsConditionalExpression(conditional_alternate) => {
                calculate_for_conditional_expression(&conditional_alternate, nesting_score)
            }
            alternate => calculate_for_expression(&alternate, nesting_score + 1),
        };

    Ok(score)
}

fn calculate_for_jsx_attributes(attributes: &JsxAttributeList, nesting_score: usize) -> usize {
    attributes
        .into_iter()
        .map(|attribute| match attribute {
            AnyJsxAttribute::JsxAttribute(jsx_attribute) => jsx_attribute
                .initializer()
                .and_then(|initializer| initializer.value().ok())
                .map(|value| match value {
                    AnyJsxAttributeValue::AnyJsxTag(jsx_tag) => {
                        calculate_for_jsx_tag(&jsx_tag, nesting_score)
                    }
                    AnyJsxAttributeValue::JsxExpressionAttributeValue(
                        jsx_expression_attribute_value,
                    ) => jsx_expression_attribute_value
                        .expression()
                        .map(|expression| calculate_for_expression(&expression, nesting_score))
                        .unwrap_or_default(),
                    AnyJsxAttributeValue::JsxString(_) => 0,
                })
                .unwrap_or_default(),
            AnyJsxAttribute::JsxSpreadAttribute(jsx_spread_attribute) => jsx_spread_attribute
                .argument()
                .map(|argument| calculate_for_expression(&argument, nesting_score))
                .unwrap_or_default(),
        })
        .sum()
}

fn calculate_for_jsx_children(children: &JsxChildList, nesting_score: usize) -> usize {
    children
        .into_iter()
        .map(|child| match child {
            AnyJsxChild::JsxElement(jsx_element) => {
                calculate_for_jsx_element(&jsx_element, nesting_score)
            }
            AnyJsxChild::JsxExpressionChild(jsx_expression_child) => jsx_expression_child
                .expression()
                .map(|expression| calculate_for_expression(&expression, nesting_score))
                .unwrap_or_default(),
            AnyJsxChild::JsxFragment(jsx_fragment) => {
                calculate_for_jsx_children(&jsx_fragment.children(), nesting_score)
            }
            AnyJsxChild::JsxSelfClosingElement(_) => todo!(),
            AnyJsxChild::JsxSpreadChild(_) => todo!(),
            AnyJsxChild::JsxText(_) => 0,
        })
        .sum()
}

fn calculate_for_jsx_element(element: &JsxElement, nesting_score: usize) -> usize {
    element
        .opening_element()
        .map(|opening_element| {
            calculate_for_jsx_attributes(&opening_element.attributes(), nesting_score)
        })
        .unwrap_or_default()
        + calculate_for_jsx_children(&element.children(), nesting_score)
}

fn calculate_for_jsx_tag(tag: &AnyJsxTag, nesting_score: usize) -> usize {
    match tag {
        AnyJsxTag::JsxElement(jsx_element) => calculate_for_jsx_element(jsx_element, nesting_score),
        AnyJsxTag::JsxFragment(jsx_fragment) => {
            calculate_for_jsx_children(&jsx_fragment.children(), nesting_score)
        }
        AnyJsxTag::JsxSelfClosingElement(jsx_self_closing_element) => {
            calculate_for_jsx_attributes(&jsx_self_closing_element.attributes(), nesting_score)
        }
    }
}

fn calculate_for_statement(statement: &AnyJsStatement, nesting_score: usize) -> usize {
    calculate_for_fallible_statement(statement, nesting_score).unwrap_or_default()
}

fn calculate_for_fallible_statement(
    statement: &AnyJsStatement,
    nesting_score: usize,
) -> SyntaxResult<usize> {
    let score = match statement {
        AnyJsStatement::JsBlockStatement(js_block) => {
            calculate_for_block(&js_block.statements(), nesting_score)
        }
        AnyJsStatement::JsBogusStatement(_) => 0,
        AnyJsStatement::JsBreakStatement(js_break) => match js_break.label_token() {
            Some(_label) => 1,
            None => 0,
        },
        AnyJsStatement::JsClassDeclaration(_) => 0,
        AnyJsStatement::JsContinueStatement(js_continue) => match js_continue.label_token() {
            Some(_label) => 1,
            None => 0,
        },
        AnyJsStatement::JsDebuggerStatement(_) => 0,
        AnyJsStatement::JsDoWhileStatement(js_do_while) => {
            1 + nesting_score + calculate_for_statement(&js_do_while.body()?, nesting_score + 1)
        }
        AnyJsStatement::JsEmptyStatement(_) => 0,
        AnyJsStatement::JsExpressionStatement(js_expression) => {
            calculate_for_expression(&js_expression.expression()?, nesting_score)
        }
        AnyJsStatement::JsForInStatement(js_for_in) => {
            1 + nesting_score
                + calculate_for_expression(&js_for_in.expression()?, nesting_score)
                + calculate_for_statement(&js_for_in.body()?, nesting_score + 1)
        }
        AnyJsStatement::JsForOfStatement(js_for_of) => {
            1 + nesting_score
                + calculate_for_expression(&js_for_of.expression()?, nesting_score)
                + calculate_for_statement(&js_for_of.body()?, nesting_score + 1)
        }
        AnyJsStatement::JsForStatement(js_for) => {
            1 + nesting_score
                + js_for
                    .initializer()
                    .map(|initializer| match initializer {
                        AnyJsForInitializer::AnyJsExpression(expression) => {
                            calculate_for_expression(&expression, nesting_score)
                        }
                        AnyJsForInitializer::JsVariableDeclaration(declaration) => {
                            calculate_for_variable_declaration(&declaration, nesting_score)
                        }
                    })
                    .unwrap_or_default()
                + js_for
                    .test()
                    .map(|test| calculate_for_expression(&test, nesting_score))
                    .unwrap_or_default()
                + js_for
                    .update()
                    .map(|update| calculate_for_expression(&update, nesting_score))
                    .unwrap_or_default()
                + calculate_for_statement(&js_for.body()?, nesting_score + 1)
        }
        AnyJsStatement::JsFunctionDeclaration(_) => 0,
        AnyJsStatement::JsIfStatement(js_if) => calculate_for_if_statement(js_if, nesting_score),
        AnyJsStatement::JsLabeledStatement(js_labeled_statement) => {
            calculate_for_statement(&js_labeled_statement.body()?, nesting_score)
        }
        AnyJsStatement::JsReturnStatement(js_return) => js_return
            .argument()
            .map(|arg| calculate_for_expression(&arg, nesting_score))
            .unwrap_or_default(),
        AnyJsStatement::JsSwitchStatement(js_switch) => {
            1 + nesting_score
                + calculate_for_expression(&js_switch.discriminant()?, nesting_score)
                + calculate_for_switch_cases(&js_switch.cases(), nesting_score + 1)
        }
        AnyJsStatement::JsThrowStatement(js_throw) => {
            calculate_for_expression(&js_throw.argument()?, nesting_score)
        }
        AnyJsStatement::JsTryFinallyStatement(js_try_finally) => {
            calculate_for_block(&js_try_finally.body()?.statements(), nesting_score)
                + js_try_finally
                    .catch_clause()
                    .and_then(|catch_clause| catch_clause.body().ok())
                    .map(|catch| {
                        1 + nesting_score
                            + calculate_for_block(&catch.statements(), nesting_score + 1)
                    })
                    .unwrap_or_default()
                + calculate_for_block(
                    &js_try_finally.finally_clause()?.body()?.statements(),
                    nesting_score + 1,
                )
        }
        AnyJsStatement::JsTryStatement(js_try) => {
            calculate_for_block(&js_try.body()?.statements(), nesting_score)
                + js_try
                    .catch_clause()
                    .and_then(|catch_clause| catch_clause.body())
                    .map(|catch| {
                        1 + nesting_score
                            + calculate_for_block(&catch.statements(), nesting_score + 1)
                    })
                    .unwrap_or_default()
        }
        AnyJsStatement::JsVariableStatement(js_variable) => {
            calculate_for_variable_declaration(&js_variable.declaration()?, nesting_score)
        }
        AnyJsStatement::JsWhileStatement(js_while) => {
            1 + nesting_score
                + calculate_for_expression(&js_while.test()?, nesting_score)
                + calculate_for_statement(&js_while.body()?, nesting_score + 1)
        }
        AnyJsStatement::JsWithStatement(js_with) => {
            1 + calculate_for_statement(&js_with.body()?, nesting_score)
        }
        AnyJsStatement::TsDeclareFunctionDeclaration(_) => 0,
        AnyJsStatement::TsDeclareStatement(_) => 0,
        AnyJsStatement::TsEnumDeclaration(_) => 0,
        AnyJsStatement::TsExternalModuleDeclaration(_) => 0,
        AnyJsStatement::TsGlobalDeclaration(_) => 0,
        AnyJsStatement::TsImportEqualsDeclaration(_) => 0,
        AnyJsStatement::TsInterfaceDeclaration(_) => 0,
        AnyJsStatement::TsModuleDeclaration(_) => 0,
        AnyJsStatement::TsTypeAliasDeclaration(_) => 0,
    };

    Ok(score)
}

fn calculate_for_if_statement(js_if: &JsIfStatement, nesting_score: usize) -> usize {
    calculate_for_fallible_if_statement(js_if, nesting_score).unwrap_or_default()
}

fn calculate_for_fallible_if_statement(
    js_if: &JsIfStatement,
    nesting_score: usize,
) -> SyntaxResult<usize> {
    let score = 1
        + nesting_score
        + calculate_for_expression(&js_if.test()?, nesting_score)
        + calculate_for_statement(&js_if.consequent()?, nesting_score + 1)
        + js_if
            .else_clause()
            .and_then(|else_clause| else_clause.alternate().ok())
            .map(|alternate| match alternate {
                AnyJsStatement::JsIfStatement(else_if) => {
                    calculate_for_if_statement(&else_if, nesting_score)
                }
                _ => 1 + calculate_for_statement(&alternate, nesting_score + 1),
            })
            .unwrap_or_default();

    Ok(score)
}

fn calculate_for_switch_cases(cases: &JsSwitchCaseList, nesting_score: usize) -> usize {
    cases
        .into_iter()
        .map(|case| calculate_for_block(&case.consequent(), nesting_score))
        .sum()
}

fn calculate_for_variable_declaration(
    declaration: &JsVariableDeclaration,
    nesting_score: usize,
) -> usize {
    declaration
        .declarators()
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|declarator| {
            declarator
                .initializer()
                .and_then(|initializer| initializer.expression().ok())
        })
        .map(|initializer_expression| {
            calculate_for_expression(&initializer_expression, nesting_score)
        })
        .sum()
}
