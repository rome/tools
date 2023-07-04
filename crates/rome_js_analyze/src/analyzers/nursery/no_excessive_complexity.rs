use crate::analyzers::correctness::use_yield::AnyFunctionLike;
use bpaf::Bpaf;
use rome_analyze::{
    context::RuleContext, declare_rule, AddVisitor, Phases, QueryMatch, Queryable, Rule,
    RuleDiagnostic, ServiceBag, Visitor, VisitorContext,
};
use rome_console::markup;
use rome_deserialize::{
    json::{has_only_known_keys, VisitJsonNode},
    DeserializationDiagnostic, VisitNode,
};
use rome_js_syntax::{
    AnyJsFunctionBody, JsBreakStatement, JsCatchClause, JsConditionalExpression,
    JsContinueStatement, JsDoWhileStatement, JsFinallyClause, JsForInStatement, JsForOfStatement,
    JsForStatement, JsIfStatement, JsLanguage, JsLogicalExpression, JsLogicalOperator,
    JsSwitchStatement, JsWhileStatement, JsWithStatement,
};
use rome_json_syntax::{JsonLanguage, JsonSyntaxNode};
use rome_rowan::{AstNode, Language, SyntaxNode, SyntaxResult, TextRange, WalkEvent};
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
    type Query = CognitiveComplexity;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ComplexityOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let calculated_score = ctx.query().score.calculated_score;
        (calculated_score > ctx.options().max_allowed_complexity).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let CognitiveComplexity {
            function_like,
            score: ComplexityScore { calculated_score },
        } = ctx.query();

        let ComplexityOptions {
            max_allowed_complexity,
        } = ctx.options();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                match function_like.id_range() {
                    Some(id_range) => id_range,
                    // This `unwrap()` is safe because we know there is a body,
                    // otherwise the visitor wouldn't have matched anything.
                    _ => function_like.body().unwrap().range()
                },
                markup!("Excessive complexity detected."),
            )
            .note(markup! {
                "Please refactor this function to reduce its complexity score from "{calculated_score}" to "{max_allowed_complexity}"."
            }),
        )
    }
}

#[derive(Clone)]
pub(crate) struct CognitiveComplexity {
    function_like: AnyFunctionLike,
    score: ComplexityScore,
}

impl QueryMatch for CognitiveComplexity {
    fn text_range(&self) -> TextRange {
        self.function_like.range()
    }
}

impl Queryable for CognitiveComplexity {
    type Input = Self;
    type Language = JsLanguage;
    type Output = Self;
    type Services = ();

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        _: &<Self::Language as Language>::Root,
    ) {
        analyzer.add_visitor(Phases::Syntax, CognitiveComplexityVisitor::default);
    }

    fn unwrap_match(_: &ServiceBag, query: &Self::Input) -> Self::Output {
        query.clone()
    }
}

struct CognitiveComplexityFunctionState {
    function_like: AnyFunctionLike,
    score: usize,
    nesting_level: usize,
    last_seen_operator: Option<JsLogicalOperator>,
}

#[derive(Default)]
struct CognitiveComplexityVisitor {
    stack: Vec<CognitiveComplexityFunctionState>,
}

impl Visitor for CognitiveComplexityVisitor {
    type Language = JsLanguage;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        mut ctx: VisitorContext<Self::Language>,
    ) {
        match event {
            WalkEvent::Enter(node) => {
                // When the visitor enters a function node, push a new entry on the stack
                if let Some(function_like) = AnyFunctionLike::cast_ref(node) {
                    self.stack.push(CognitiveComplexityFunctionState {
                        function_like,
                        score: 0,
                        nesting_level: 0,
                        last_seen_operator: None,
                    });
                }

                if let Some(state) = self.stack.last_mut() {
                    if receives_structural_penalty(node) {
                        state.score += 1;

                        if receives_nesting_penalty(node) {
                            state.score += state.nesting_level;
                        }
                    }

                    if increases_nesting(node) {
                        state.last_seen_operator = None;
                        state.nesting_level += 1;
                    } else if let Some(operator) = JsLogicalExpression::cast_ref(node)
                        .and_then(|expression| expression.operator().ok())
                    {
                        if state.last_seen_operator != Some(operator) {
                            state.score += 1;
                            state.last_seen_operator = Some(operator);
                        }
                    } else {
                        // Reset the operator for every other type of node.
                        state.last_seen_operator = None;
                    }
                }
            }
            WalkEvent::Leave(node) => {
                if let Some(exit_node) = AnyFunctionLike::cast_ref(node) {
                    if let Some(function_state) = self.stack.pop() {
                        assert_eq!(function_state.function_like, exit_node);
                        ctx.match_query(CognitiveComplexity {
                            function_like: exit_node,
                            score: ComplexityScore {
                                calculated_score: function_state.score,
                            },
                        });
                    }
                } else if let Some(state) = self.stack.last_mut() {
                    if increases_nesting(node) {
                        state.nesting_level = state.nesting_level.saturating_sub(1);
                    }
                }
            }
        }
    }
}

fn increases_nesting(node: &SyntaxNode<JsLanguage>) -> bool {
    let kind = node.kind();
    is_loop_node(node)
        || JsCatchClause::can_cast(kind)
        || JsConditionalExpression::can_cast(kind)
        || JsIfStatement::can_cast(kind)
        || JsSwitchStatement::can_cast(kind)
}

fn is_loop_node(node: &SyntaxNode<JsLanguage>) -> bool {
    let kind = node.kind();
    JsDoWhileStatement::can_cast(kind)
        || JsForInStatement::can_cast(kind)
        || JsForOfStatement::can_cast(kind)
        || JsForStatement::can_cast(kind)
        || JsWhileStatement::can_cast(kind)
}

fn receives_structural_penalty(node: &SyntaxNode<JsLanguage>) -> bool {
    let kind = node.kind();
    receives_nesting_penalty(node)
        || JsBreakStatement::cast_ref(node)
            .and_then(|js_break| js_break.label_token())
            .is_some()
        || JsContinueStatement::cast_ref(node)
            .and_then(|js_continue| js_continue.label_token())
            .is_some()
        || JsFinallyClause::can_cast(kind)
        || JsWithStatement::can_cast(kind)
}

// Note: This is a strict subset of nodes that receive a structural penalty.
fn receives_nesting_penalty(node: &SyntaxNode<JsLanguage>) -> bool {
    let kind = node.kind();
    is_loop_node(node)
        || JsCatchClause::can_cast(kind)
        || JsConditionalExpression::can_cast(kind)
        || JsIfStatement::can_cast(kind)
        || JsSwitchStatement::can_cast(kind)
}

#[derive(Clone, Default)]
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

impl AnyFunctionLike {
    fn body(&self) -> SyntaxResult<AnyJsFunctionBody> {
        match self {
            AnyFunctionLike::AnyJsFunction(js_function) => js_function.body(),
            AnyFunctionLike::JsMethodObjectMember(js_object_method) => js_object_method
                .body()
                .map(AnyJsFunctionBody::JsFunctionBody),
            AnyFunctionLike::JsMethodClassMember(js_class_method) => js_class_method
                .body()
                .map(AnyJsFunctionBody::JsFunctionBody),
        }
    }

    fn id_range(&self) -> Option<TextRange> {
        match self {
            AnyFunctionLike::AnyJsFunction(js_function) => {
                js_function.id().ok().flatten().map(|id| id.range())
            }
            AnyFunctionLike::JsMethodObjectMember(js_object_method) => {
                js_object_method.name().ok().map(|name| name.range())
            }
            AnyFunctionLike::JsMethodClassMember(js_class_method) => {
                js_class_method.name().ok().map(|name| name.range())
            }
        }
    }
}
