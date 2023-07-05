use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::{markup, MarkupBuf};
use rome_js_syntax::{
    AnyJsClass, AnyJsExpression, JsAssignmentOperator, JsConstructorClassMember, JsLogicalOperator,
};
use rome_rowan::{AstNode, AstNodeList, TextRange};

declare_rule! {
    /// Prevents the incorrect use of `super()` inside classes.
    /// It also checks whether a call `super()` is missing from classes that extends other constructors.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class A {
    ///     constructor() {
    ///         super();
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class A extends undefined {
    ///     constructor() {
    ///         super();
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// export default class A extends B {
    ///     constructor() {
    ///         super();
    ///     }
    /// }
    /// ```
    ///
    /// ```js
    /// export class A {
    ///     constructor() {}
    /// }
    /// ```
    ///
    pub(crate) NoInvalidConstructorSuper {
        version: "10.0.0",
        name: "noInvalidConstructorSuper",
        recommended: true,
    }
}

pub(crate) enum NoInvalidConstructorSuperState {
    UnexpectedSuper(TextRange),
    BadExtends {
        extends_range: TextRange,
        super_range: TextRange,
    },
}

impl NoInvalidConstructorSuperState {
    fn range(&self) -> &TextRange {
        match self {
            NoInvalidConstructorSuperState::UnexpectedSuper(range) => range,
            NoInvalidConstructorSuperState::BadExtends { super_range, .. } => super_range,
        }
    }

    fn message(&self) -> MarkupBuf {
        match self {
            NoInvalidConstructorSuperState::UnexpectedSuper(_) => {
                (markup! { "This class should not have a "<Emphasis>"super()"</Emphasis>" call. You should remove it." }).to_owned()
            }

            NoInvalidConstructorSuperState::BadExtends { .. } => {
                (markup! { "This class calls "<Emphasis>"super()"</Emphasis>", but the class extends from a non-constructor." }).to_owned()
            }
        }
    }

    fn detail(&self) -> Option<(&TextRange, MarkupBuf)> {
        match self {
            NoInvalidConstructorSuperState::BadExtends { extends_range, .. } => Some((
                extends_range,
                markup! { "This is where the non-constructor is used." }.to_owned(),
            )),
            _ => None,
        }
    }
}

impl Rule for NoInvalidConstructorSuper {
    type Query = Ast<JsConstructorClassMember>;
    type State = NoInvalidConstructorSuperState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        // we check first if `super()` is part of a constructor class member
        let super_expression = node.body().ok()?.statements().iter().find_map(|statement| {
            statement
                .as_js_expression_statement()?
                .expression()
                .ok()?
                .as_js_call_expression()?
                .callee()
                .ok()?
                .as_js_super_expression()
                .cloned()
        });

        let extends_clause = node
            .syntax()
            .ancestors()
            .find_map(|node| AnyJsClass::cast(node)?.extends_clause());

        match (super_expression, extends_clause) {
            (Some(super_expression), Some(extends_clause)) => {
                let super_class = extends_clause.super_class().ok()?;
                if let Some(is_valid) = is_valid_constructor(super_class.clone()) {
                    if !is_valid {
                        return Some(NoInvalidConstructorSuperState::BadExtends {
                            super_range: super_expression.syntax().text_trimmed_range(),
                            extends_range: super_class.syntax().text_trimmed_range(),
                        });
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            (Some(super_expression), None) => Some(
                NoInvalidConstructorSuperState::UnexpectedSuper(super_expression.range()),
            ),
            _ => None,
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diagnostic = RuleDiagnostic::new(rule_category!(), state.range(), state.message());
        if let Some((range, text)) = state.detail() {
            diagnostic = diagnostic.detail(range, text);
        }
        Some(diagnostic)
    }
}

fn is_valid_constructor(expression: AnyJsExpression) -> Option<bool> {
    match expression.omit_parentheses() {
        AnyJsExpression::JsAwaitExpression(await_expression) => {
            is_valid_constructor(await_expression.argument().ok()?)
        }
        AnyJsExpression::JsThisExpression(_)
        | AnyJsExpression::JsFunctionExpression(_)
        | AnyJsExpression::JsCallExpression(_)
        | AnyJsExpression::JsImportCallExpression(_)
        | AnyJsExpression::JsImportMetaExpression(_)
        | AnyJsExpression::JsYieldExpression(_)
        | AnyJsExpression::JsNewExpression(_)
        | AnyJsExpression::JsNewTargetExpression(_)
        | AnyJsExpression::JsStaticMemberExpression(_)
        | AnyJsExpression::JsClassExpression(_) => Some(true),
        AnyJsExpression::JsIdentifierExpression(identifier) => {
            Some(!identifier.name().ok()?.is_undefined())
        }
        AnyJsExpression::JsAssignmentExpression(assignment) => {
            let operator = assignment.operator().ok()?;
            if matches!(
                operator,
                JsAssignmentOperator::Assign
                    | JsAssignmentOperator::LogicalAndAssign
                    | JsAssignmentOperator::LogicalOrAssign
                    | JsAssignmentOperator::NullishCoalescingAssign
            ) {
                return is_valid_constructor(assignment.right().ok()?);
            }
            Some(false)
        }
        AnyJsExpression::JsLogicalExpression(expression) => {
            let operator = expression.operator().ok()?;
            if matches!(operator, JsLogicalOperator::LogicalAnd) {
                return is_valid_constructor(expression.right().ok()?);
            }
            is_valid_constructor(expression.left().ok()?)
                .or_else(|| is_valid_constructor(expression.right().ok()?))
        }
        AnyJsExpression::JsConditionalExpression(conditional_expression) => {
            is_valid_constructor(conditional_expression.alternate().ok()?)
                .or_else(|| is_valid_constructor(conditional_expression.consequent().ok()?))
        }
        AnyJsExpression::JsSequenceExpression(sequence_expression) => {
            is_valid_constructor(sequence_expression.right().ok()?)
        }
        AnyJsExpression::JsTemplateExpression(template_expression) => {
            // Tagged templates can return anything
            Some(template_expression.tag().is_some())
        }
        AnyJsExpression::TsInstantiationExpression(instantiation_expression) => {
            is_valid_constructor(instantiation_expression.expression().ok()?)
        }
        AnyJsExpression::TsAsExpression(type_assertion) => {
            is_valid_constructor(type_assertion.expression().ok()?)
        }
        AnyJsExpression::TsNonNullAssertionExpression(type_assertion) => {
            is_valid_constructor(type_assertion.expression().ok()?)
        }
        AnyJsExpression::TsSatisfiesExpression(type_assertion) => {
            is_valid_constructor(type_assertion.expression().ok()?)
        }
        AnyJsExpression::TsTypeAssertionExpression(type_assertion) => {
            is_valid_constructor(type_assertion.expression().ok()?)
        }
        AnyJsExpression::JsComputedMemberExpression(_)
        | AnyJsExpression::AnyJsLiteralExpression(_)
        | AnyJsExpression::JsArrayExpression(_)
        | AnyJsExpression::JsArrowFunctionExpression(_)
        | AnyJsExpression::JsBinaryExpression(_)
        | AnyJsExpression::JsBogusExpression(_)
        | AnyJsExpression::JsInstanceofExpression(_)
        | AnyJsExpression::JsObjectExpression(_)
        | AnyJsExpression::JsPostUpdateExpression(_)
        | AnyJsExpression::JsPreUpdateExpression(_)
        | AnyJsExpression::JsSuperExpression(_)
        | AnyJsExpression::JsUnaryExpression(_)
        | AnyJsExpression::JsxTagExpression(_) => Some(false),
        AnyJsExpression::JsInExpression(_) => None,
        // Should not be triggered because we called `omit_parentheses`
        AnyJsExpression::JsParenthesizedExpression(_) => None,
    }
}
