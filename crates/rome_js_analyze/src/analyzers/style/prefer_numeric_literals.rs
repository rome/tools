use crate::JsRuleAction;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyCallArgument, JsAnyExpression, JsAnyLiteralExpression, JsCallExpression, JsSyntaxToken,
};
use rome_rowan::{AstNode, AstNodeList, AstSeparatedList, BatchMutationExt};

declare_rule! {
    /// Disallow `parseInt()` and `Number.parseInt()` in favor of binary, octal, and hexadecimal literals
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// parseInt("111110111", 2) === 503;
    /// parseInt(`111110111`, 2) === 503;
    /// parseInt("767", 8) === 503;
    /// parseInt("1F7", 16) === 503;
    /// Number.parseInt("111110111", 2) === 503;
    /// Number.parseInt("767", 8) === 503;
    /// Number.parseInt("1F7", 16) === 503;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// parseInt(1);
    /// parseInt(1, 3);
    /// Number.parseInt(1);
    /// Number.parseInt(1, 3);
    ///
    /// 0b111110111 === 503;
    /// 0o767 === 503;
    /// 0x1F7 === 503;
    ///
    /// a[parseInt](1,2);
    ///
    /// parseInt(foo);
    /// parseInt(foo, 2);
    /// Number.parseInt(foo);
    /// Number.parseInt(foo, 2);
    /// ```
    pub(crate) PreferNumericLiterals {
        version: "0.7.0",
        name: "preferNumericLiterals",
        recommended: true,
    }
}

pub struct TextAndRadix {
    text_token: JsSyntaxToken,
    radix: Radix,
}

impl Rule for PreferNumericLiterals {
    type Query = Ast<JsCallExpression>;
    type State = TextAndRadix;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        if !is_callee_parse_int_fn(node).unwrap_or(false) {
            return None;
        }
        get_text_and_radix_args(node)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! { "Use "{state.radix.description()}" literals instead of parseInt()" }
                .to_owned(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let text = state.text_token.text_trimmed();
        let text = &text[1..text.len() - 1];
        i128::from_str_radix(text, state.radix as u32).ok()?;
        let suggested = format!("{}{text}", state.radix.prefix());

        mutation.replace_node(
            JsAnyExpression::JsCallExpression(node.clone()),
            JsAnyExpression::JsAnyLiteralExpression(
                JsAnyLiteralExpression::JsNumberLiteralExpression(
                    make::js_number_literal_expression(make::js_number_literal(&suggested)),
                ),
            ),
        );

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! { "Replace with "{state.radix.description()}" literals" }.to_owned(),
            mutation,
        })
    }
}

fn get_text_and_radix_args(expr: &JsCallExpression) -> Option<TextAndRadix> {
    let args = expr.arguments().ok()?.args();
    if args.len() != 2 {
        return None;
    }
    let mut args = args.into_iter();
    let text = args.next()?.ok().and_then(get_text)?;
    let radix = args.next()?.ok().and_then(get_number)?;
    Some(TextAndRadix {
        text_token: text,
        radix: Radix::from_f64(radix)?,
    })
}

fn get_text(arg: JsAnyCallArgument) -> Option<JsSyntaxToken> {
    match arg.as_js_any_expression()? {
        JsAnyExpression::JsTemplate(t) => {
            if t.tag().is_some() {
                return None;
            }

            let elements = t.elements();
            if elements.len() != 1 {
                return None;
            }

            let elem = elements.into_iter().next()?;
            let chunk = elem.as_js_template_chunk_element()?;
            return chunk.template_chunk_token().ok();
        }
        JsAnyExpression::JsAnyLiteralExpression(
            JsAnyLiteralExpression::JsStringLiteralExpression(s),
        ) => s.value_token().ok(),
        _ => None,
    }
}

fn get_number(arg: JsAnyCallArgument) -> Option<f64> {
    let arg = arg
        .as_js_any_expression()?
        .as_js_any_literal_expression()?
        .as_js_number_literal_expression()?
        .as_number()?;
    Some(arg)
}

fn is_callee_parse_int_fn(expr: &JsCallExpression) -> Option<bool> {
    let callee = expr.callee().ok()?;

    match callee {
        JsAnyExpression::JsIdentifierExpression(ident) => {
            Some(ident.name().ok()?.syntax().text_trimmed() == "parseInt")
        }
        JsAnyExpression::JsStaticMemberExpression(expr) => Some(
            expr.object()
                .ok()?
                .as_js_identifier_expression()?
                .syntax()
                .text_trimmed()
                == "Number"
                && expr.member().ok()?.syntax().text_trimmed() == "parseInt",
        ),
        _ => None,
    }
}

enum Radix {
    Binary = 2,
    Octal = 8,
    Hexadecimal = 16,
}

impl Radix {
    fn from_f64(v: f64) -> Option<Self> {
        Some(if v == 2.0 {
            Self::Binary
        } else if v == 8.0 {
            Self::Octal
        } else if v == 16.0 {
            Self::Hexadecimal
        } else {
            return None;
        })
    }

    fn prefix(&self) -> &'static str {
        match self {
            Radix::Binary => "0b",
            Radix::Octal => "0o",
            Radix::Hexadecimal => "0x",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Radix::Binary => "binary",
            Radix::Octal => "octal",
            Radix::Hexadecimal => "hexadecimal",
        }
    }
}
