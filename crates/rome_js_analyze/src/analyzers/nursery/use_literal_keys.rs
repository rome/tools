use crate::JsRuleAction;
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make::{ident, js_name, js_static_member_expression, token};
use rome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsName, JsComputedMemberAssignment,
    JsComputedMemberExpression, T,
};
use rome_rowan::{declare_node_union, AstNode, BatchMutationExt, SyntaxResult, TextRange};

declare_rule! {
    /// Enforce the usage of a static property access over computed property access.
    ///
    /// Source: https://eslint.org/docs/latest/rules/dot-notation
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// a.b["c"];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// a.c[`d`]
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// a.c[`d`] = "something"
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// a["c" + "d"];
    /// a[d.c];
    /// ```
    ///
    pub(crate) UseLiteralKeys {
        version: "next",
        name: "useLiteralKeys",
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) AnyJsComputedMember = JsComputedMemberExpression | JsComputedMemberAssignment
}

impl AnyJsComputedMember {
    pub(crate) fn member(&self) -> SyntaxResult<AnyJsExpression> {
        match self {
            AnyJsComputedMember::JsComputedMemberExpression(node) => node.member(),
            AnyJsComputedMember::JsComputedMemberAssignment(node) => node.member(),
        }
    }

    pub(crate) fn object(&self) -> SyntaxResult<AnyJsExpression> {
        match self {
            AnyJsComputedMember::JsComputedMemberExpression(node) => node.object(),
            AnyJsComputedMember::JsComputedMemberAssignment(node) => node.object(),
        }
    }
}

impl Rule for UseLiteralKeys {
    type Query = Ast<AnyJsComputedMember>;
    type State = (TextRange, String);
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let computed_expression = ctx.query();

        let member = computed_expression.member().ok()?;
        match member {
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(string_literal),
            ) => {
                let value = string_literal.inner_string_text().ok()?;
                return Some((string_literal.range(), value.to_string()));
            }
            AnyJsExpression::JsTemplateExpression(template_expression) => {
                let mut value = String::new();
                for element in template_expression.elements() {
                    let chunk = element.as_js_template_chunk_element()?;

                    value.push_str(chunk.template_chunk_token().ok()?.text_trimmed());
                }
                return Some((template_expression.range(), value));
            }

            _ => {}
        };
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, (range, _): &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "The computed expression can be simplified without the use of a string literal."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, (_, identifier): &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();

        let mut mutation = ctx.root().begin();

        let object = node.object().ok()?;
        let member = js_name(ident(identifier));
        let static_expression =
            js_static_member_expression(object, token(T![.]), AnyJsName::JsName(member));
        mutation.replace_element(
            node.clone().into_syntax().into(),
            static_expression.into_syntax().into(),
        );
        Some(JsRuleAction {
            mutation,
            applicability: Applicability::MaybeIncorrect,
            category: ActionCategory::QuickFix,
            message: markup! {
                "Replace it with a static expression."
            }
            .to_owned(),
        })
    }
}
