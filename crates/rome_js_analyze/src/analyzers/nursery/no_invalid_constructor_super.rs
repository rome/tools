use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::{markup, MarkupBuf};
use rome_js_syntax::{
    JsAnyExpression, JsAssignmentOperator, JsClassDeclaration, JsConstructorClassMember,
    JsLogicalOperator,
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
    /// class A extends B {
    ///     constructor() {}
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class A {
    ///     constructor() {
    ///         super();
    ///     }
    /// }
    /// ```
    ///
    pub(crate) NoInvalidConstructorSuper {
        version: "10.0.0",
        name: "noInvalidConstructorSuper",
        recommended: false,
    }
}

pub(crate) enum NoInvalidConstructorSuperState {
    MissingSuper(TextRange),
    UnexpectedSuper(TextRange),
    BadExtends {
        extends_range: TextRange,
        super_range: TextRange,
    },
}

impl NoInvalidConstructorSuperState {
    fn range(&self) -> &TextRange {
        match self {
            NoInvalidConstructorSuperState::MissingSuper(range) => range,
            NoInvalidConstructorSuperState::UnexpectedSuper(range) => range,
            NoInvalidConstructorSuperState::BadExtends { super_range, .. } => super_range,
        }
    }

    fn message(&self) -> MarkupBuf {
        match self {
            NoInvalidConstructorSuperState::MissingSuper(_) => {
                (markup! { "This class extends another class and a "<Emphasis>"super()"</Emphasis>" call is expected." }).to_owned()
            }
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
            .find_map(|node| JsClassDeclaration::cast(node)?.extends_clause());

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
            (None, Some(extends_clause)) => {
                let super_class = extends_clause.super_class().ok()?;
                if !matches!(super_class, JsAnyExpression::JsAnyLiteralExpression(_,)) {
                    Some(NoInvalidConstructorSuperState::MissingSuper(
                        extends_clause.syntax().text_trimmed_range(),
                    ))
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
            diagnostic = diagnostic.secondary(range, text);
        }
        Some(diagnostic)
    }
}

fn is_valid_constructor(expression: JsAnyExpression) -> Option<bool> {
    match expression {
        JsAnyExpression::JsThisExpression(_)
        | JsAnyExpression::JsFunctionExpression(_)
        | JsAnyExpression::JsCallExpression(_)
        | JsAnyExpression::JsImportCallExpression(_)
        | JsAnyExpression::ImportMeta(_)
        | JsAnyExpression::JsYieldExpression(_)
        | JsAnyExpression::JsNewExpression(_)
        | JsAnyExpression::NewTarget(_)
        | JsAnyExpression::JsClassExpression(_) => Some(true),
        JsAnyExpression::JsIdentifierExpression(identifier) => {
            let name = identifier.name().ok()?;
            return Some(name.value_token().ok()?.text_trimmed() != "undefined");
        }
        JsAnyExpression::JsAssignmentExpression(assignment) => {
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

        JsAnyExpression::JsLogicalExpression(expression) => {
            let operator = expression.operator().ok()?;
            if matches!(operator, JsLogicalOperator::LogicalAnd) {
                return is_valid_constructor(expression.right().ok()?);
            }

            is_valid_constructor(expression.left().ok()?)
                .or_else(|| is_valid_constructor(expression.right().ok()?))
        }
        JsAnyExpression::JsConditionalExpression(conditional_expression) => {
            is_valid_constructor(conditional_expression.alternate().ok()?)
                .or_else(|| is_valid_constructor(conditional_expression.consequent().ok()?))
        }
        JsAnyExpression::JsSequenceExpression(sequence_expression) => {
            is_valid_constructor(sequence_expression.right().ok()?)
        }
        JsAnyExpression::JsParenthesizedExpression(expression) => {
            is_valid_constructor(expression.expression().ok()?)
        }
        _ => Some(false),
    }
}
