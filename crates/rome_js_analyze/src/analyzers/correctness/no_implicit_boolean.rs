use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyLiteralExpression, JsSyntaxKind, JsxAnyAttributeValue, JsxAttribute, JsxAttributeFields, T,
};
use rome_rowan::{AstNode, AstNodeExt, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow implicit `true` values on JSX boolean attributes
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <input disabled />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <input disabled={false} />
    ///```
    ///
    /// ```jsx
    /// <input disabled={''} />
    ///```
    ///
    /// ```jsx
    /// <input disabled={0} />
    ///```
    ///
    /// ```jsx
    /// <input disabled={undefined} />
    ///```
    ///
    /// ```jsx
    /// <input disabled='false' />
    ///```
    pub(crate) NoImplicitBoolean {
        version: "0.7.0",
        name: "noImplicitBoolean",
        recommended: true,
    }
}

impl Rule for NoImplicitBoolean {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Ast<JsxAttribute>;
    type State = ();
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let n = ctx.query();

        match n.initializer() {
            Some(_) => None,
            None => Some(()),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let n = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            n.range(),
            markup! {
                "Use explicit boolean values for boolean JSX props."
            }
            .to_owned(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let n = ctx.query();
        let mut mutation = ctx.root().begin();

        let JsxAttributeFields {
            name,
            initializer: _,
        } = n.as_fields();

        let name = name.ok()?;
        // we use this variable for constructing `JsxAnyAttributeName` without clone the name, so we pre compute the type here.

        let name_syntax = name.syntax();

        // we need to move trailing_trivia of name_syntax to close_curly_token
        // <div disabled /**test*/ /> ->    <div disabled={true}/**test*/ />
        let last_token_of_name_syntax = name_syntax.last_token()?;
        // drop the trailing trivia of name_syntax, at CST level it means
        // clean the trailing trivia of last token of name_syntax
        let next_last_token_of_name_syntax =
            last_token_of_name_syntax.with_trailing_trivia(std::iter::empty());

        let next_name = name.replace_token_discard_trivia(
            last_token_of_name_syntax,
            next_last_token_of_name_syntax,
        )?;
        let attr_value = make::jsx_expression_attribute_value(
            make::token(JsSyntaxKind::L_CURLY),
            rome_js_syntax::JsAnyExpression::JsAnyLiteralExpression(
                JsAnyLiteralExpression::JsBooleanLiteralExpression(
                    make::js_boolean_literal_expression(make::token(T![true])),
                ),
            ),
            make::token(JsSyntaxKind::R_CURLY),
        );
        let next_attr = make::jsx_attribute(next_name).with_initializer(
            make::jsx_attribute_initializer_clause(
                make::token(T![=]),
                JsxAnyAttributeValue::JsxExpressionAttributeValue(attr_value),
            ),
        );
        let next_attr = next_attr.build();

        mutation.replace_node(n.clone(), next_attr);

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! { "Add explicit `true` literal for this attribute" }.to_owned(),
            mutation,
        })
    }
}
