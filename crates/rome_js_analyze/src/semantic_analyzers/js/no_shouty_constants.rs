use crate::{semantic_services::Semantic, utils::batch::JsBatchMutation, JsRuleAction};
use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_semantic::{AllReferencesExtensions, IsExportedCanBeQueried, Reference};
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsIdentifierBinding, JsIdentifierExpression,
    JsStringLiteralExpression, JsVariableDeclaration, JsVariableDeclarator,
    JsVariableDeclaratorList, SyntaxNodeText, TextSize,
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
    /// ```js
    /// export const FOO = "FOO";
    /// console.log(FOO);
    /// ```
    pub(crate) NoShoutyConstants {
        version: "0.7.0",
        name: "noShoutyConstants",
        recommended: true,
    }
}

/// Check for
/// A = "A" (true)
/// a = "b" (false)
/// export const A = "A" (false)
fn is_shouty_constants(
    declarator: &JsVariableDeclarator,
    ctx: &RuleContext<NoShoutyConstants>,
) -> Option<(JsIdentifierBinding, JsStringLiteralExpression)> {
    let model = ctx.model();

    let id = declarator.id().ok()?;
    let id = id.as_js_any_binding()?.as_js_identifier_binding()?;
    // For example,
    // ```js
    // export const ACTION_TYPE = "ACTION_TYPE";
    // ```
    // this is a common pattern when we write `redux`
    if id.is_exported(model) {
        return None;
    }
    let id_text = id.syntax().text_trimmed();

    let expression = declarator.initializer()?.expression().ok()?;
    let literal = expression
        .as_js_any_literal_expression()?
        .as_js_string_literal_expression()?;
    let literal_text = literal.inner_string_text();

    if id_text.len() == literal_text.len()
        && is_upper_case_of_source(id_text, literal_text).unwrap_or(false)
    {
        Some((id.clone(), literal.clone()))
    } else {
        None
    }
}

/// second param(target) is the upper-case version of its first param (source).
fn is_upper_case_of_source(source: SyntaxNodeText, text: SyntaxNodeText) -> Option<bool> {
    let len = source.len();
    let len: u32 = len.try_into().ok()?;
    for i in 0..len as usize {
        let source_char = source.char_at(TextSize::from(i as u32))?;
        let text_char = text.char_at(TextSize::from(i as u32))?;
        if source_char.to_ascii_uppercase() != text_char {
            return Some(false);
        }
    }
    Some(true)
}

pub struct State {
    literal: JsStringLiteralExpression,
    references: Vec<Reference>,
}

impl Rule for NoShoutyConstants {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Semantic<JsVariableDeclarator>;
    type State = State;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let declarator = ctx.query();
        let declaration = declarator
            .parent::<JsVariableDeclaratorList>()?
            .parent::<JsVariableDeclaration>()?;

        if declaration.is_const() {
            if let Some((binding, literal)) = is_shouty_constants(declarator, ctx) {
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
            declarator.syntax().text_trimmed_range(),
            markup! {
                "Redundant constant declaration."
            },
        );

        for reference in state.references.iter() {
            let node = reference.node();
            diag = diag.secondary(node.text_trimmed_range(), "Used here.")
        }

        let diag = diag.footer_note(
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
            category: ActionCategory::Refactor,
            applicability: Applicability::Unspecified,
            message: markup! { "Use the constant value directly" }.to_owned(),
            mutation: batch,
        })
    }
}
