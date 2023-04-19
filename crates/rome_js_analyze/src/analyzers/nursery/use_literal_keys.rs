use crate::JsRuleAction;
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make::{
    ident, js_literal_member_name, js_name, js_static_member_expression, token,
};
use rome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsName, JsComputedMemberAssignment,
    JsComputedMemberExpression, JsComputedMemberName, T,
};
use rome_js_unicode_table::{is_id_continue, is_id_start};
use rome_rowan::{declare_node_union, AstNode, BatchMutationExt, SyntaxResult, TextRange};

declare_rule! {
    /// Enforce the usage of a literal access to properties over computed property access.
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
    /// ```js,expect_diagnostic
    /// a = {
    /// 	['b']: d
    /// }
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
    pub(crate) AnyJsComputedMember = AnyJsCompputedExpression | JsComputedMemberName
}

declare_node_union! {
    pub(crate) AnyJsCompputedExpression = JsComputedMemberExpression | JsComputedMemberAssignment
}

impl AnyJsCompputedExpression {
    pub(crate) fn member(&self) -> SyntaxResult<AnyJsExpression> {
        match self {
            AnyJsCompputedExpression::JsComputedMemberExpression(node) => node.member(),
            AnyJsCompputedExpression::JsComputedMemberAssignment(node) => node.member(),
        }
    }

    pub(crate) fn object(&self) -> SyntaxResult<AnyJsExpression> {
        match self {
            AnyJsCompputedExpression::JsComputedMemberExpression(node) => node.object(),
            AnyJsCompputedExpression::JsComputedMemberAssignment(node) => node.object(),
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

        let inner_expression = match computed_expression {
            AnyJsComputedMember::AnyJsCompputedExpression(computed_expression) => {
                computed_expression.member().ok()?
            }
            AnyJsComputedMember::JsComputedMemberName(member) => member.expression().ok()?,
        };

        match inner_expression {
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(string_literal),
            ) => {
                let value = string_literal.inner_string_text().ok()?;
                if can_convert_to_static_member(&value) {
                    return Some((string_literal.range(), value.to_string()));
                }
            }
            AnyJsExpression::JsTemplateExpression(template_expression) => {
                let mut value = String::new();
                for element in template_expression.elements() {
                    let chunk = element.as_js_template_chunk_element()?;

                    value.push_str(chunk.template_chunk_token().ok()?.text_trimmed());
                }
                if can_convert_to_static_member(&value) {
                    return Some((template_expression.range(), value));
                }
            }

            _ => {}
        }

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

        match node {
            AnyJsComputedMember::AnyJsCompputedExpression(node) => {
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
            AnyJsComputedMember::JsComputedMemberName(member) => {
                let literal_member_name = js_literal_member_name(ident(identifier));
                mutation.replace_element(
                    member.clone().into_syntax().into(),
                    literal_member_name.into_syntax().into(),
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
    }
}

// This function check if the key is valid JavaScript identifier.
// Currently, it doesn't check escaped unicode chars.
fn can_convert_to_static_member(key: &str) -> bool {
    key.chars().enumerate().all(|(index, c)| {
        if index == 0 {
            is_id_start(c)
        } else {
            is_id_continue(c)
        }
    })
}
