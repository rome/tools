use crate::{semantic_services::Semantic, utils::batch::JsBatchMutation, JsRuleAction};
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_semantic::{AllReferencesExtensions, Reference};
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsIdentifierBinding, JsIdentifierExpression,
    JsStringLiteralExpression, JsVariableDeclaration, JsVariableDeclarationClause,
    JsVariableDeclarator, JsVariableDeclaratorList,
};
use rome_rowan::{AstNode, BatchMutationExt, SyntaxNodeCast};

declare_rule! {
    /// Disallow the use of constants which its value is the upper-case version of its name.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const FOO = "FOO";
    /// console.log(FOO);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// let FOO = "FOO";
    /// console.log(FOO);
    /// ```
    ///
    /// ```js
    /// export const FOO = "FOO";
    /// console.log(FOO);
    /// ```
    ///
    /// ```js
    /// function f(FOO = "FOO") {
    ///     return FOO;
    /// }
    /// ```
    ///
    pub(crate) NoShoutyConstants {
        version: "0.7.0",
        name: "noShoutyConstants",
        recommended: true,
    }
}

/// Check for
/// a = "a" (true)
/// a = "b" (false)
fn is_id_and_string_literal_inner_text_equal(
    declarator: &JsVariableDeclarator,
) -> Option<(JsIdentifierBinding, JsStringLiteralExpression)> {
    let id = declarator.id().ok()?;
    let id = id.as_js_any_binding()?.as_js_identifier_binding()?;
    let name = id.name_token().ok()?;
    let id_text = name.text_trimmed();

    let expression = declarator.initializer()?.expression().ok()?;
    let literal = expression
        .as_js_any_literal_expression()?
        .as_js_string_literal_expression()?;
    let literal_text = literal.inner_string_text().ok()?;

    for (from_id, from_literal) in id_text.chars().zip(literal_text.chars()) {
        if from_id != from_literal {
            return None;
        }
        if from_id.is_lowercase() || from_literal.is_lowercase() {
            return None;
        }
    }

    Some((id.clone(), literal.clone()))
}

pub struct State {
    literal: JsStringLiteralExpression,
    references: Vec<Reference>,
}

impl Rule for NoShoutyConstants {
    type Query = Semantic<JsVariableDeclarator>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let declarator = ctx.query();
        let declaration = declarator
            .parent::<JsVariableDeclaratorList>()?
            .parent::<JsVariableDeclaration>()?;

        if declaration.is_const() {
            if let Some((binding, literal)) = is_id_and_string_literal_inner_text_equal(declarator)
            {
                let model = ctx.model();
                if model.is_exported(&binding) {
                    return None;
                }

                if binding.all_references(ctx.model()).all(|r| {
                    r.node()
                        .ancestors()
                        .any(|n| JsVariableDeclarationClause::can_cast(n.kind()))
                }) {
                    return None;
                }

                return Some(State {
                    literal,
                    references: binding.all_references(ctx.model()).collect(),
                });
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let declarator = ctx.query();

        let mut diag = RuleDiagnostic::new(
            rule_category!(),
            declarator.syntax().text_trimmed_range(),
            markup! {
                "Redundant constant declaration."
            },
        );

        for reference in state.references.iter() {
            let node = reference.node();
            diag = diag.detail(node.text_trimmed_range(), "Used here.")
        }

        let diag = diag.note(
            markup! {"You should avoid declaring constants with a string that's the same
    value as the variable name. It introduces a level of unnecessary
    indirection when it's only two additional characters to inline."},
        );

        Some(diag)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let root = ctx.root();
        let literal = JsAnyLiteralExpression::JsStringLiteralExpression(state.literal.clone());

        let mut batch = root.begin();

        batch.remove_js_variable_declarator(ctx.query());

        for reference in state.references.iter() {
            let node = reference
                .node()
                .parent()?
                .cast::<JsIdentifierExpression>()?;

            batch.replace_node(
                JsAnyExpression::JsIdentifierExpression(node),
                JsAnyExpression::JsAnyLiteralExpression(literal.clone()),
            );
        }

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Use the constant value directly" }.to_owned(),
            mutation: batch,
        })
    }
}
