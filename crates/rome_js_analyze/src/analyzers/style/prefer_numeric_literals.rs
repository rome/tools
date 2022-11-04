use crate::JsRuleAction;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyCallArgument, JsAnyExpression, JsAnyLiteralExpression, JsCallExpression,
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
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Number.parseInt("767", 8) === 503;
    /// ```
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

pub struct CallInfo {
    callee: &'static str,
    text: String,
    radix: Radix,
}

impl Rule for PreferNumericLiterals {
    type Query = Ast<JsCallExpression>;
    type State = CallInfo;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        get_call_info(node)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! { "Use "{state.radix.description()}" literals instead of "{state.callee} }
                .to_owned(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        i128::from_str_radix(&state.text, state.radix as u32).ok()?;
        let suggested = format!("{}{}", state.radix.prefix(), state.text);

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

fn get_call_info(expr: &JsCallExpression) -> Option<CallInfo> {
    let callee = get_callee(expr)?;
    let args = expr.arguments().ok()?.args();
    if args.len() != 2 {
        return None;
    }
    let mut args = args.into_iter();
    let text = args.next()?.ok()?;
    let radix = args.next()?.ok()?;
    let text_token = get_text(text.as_js_any_expression()?)?;
    let radix = get_number(radix)?;
    Some(CallInfo {
        callee,
        text: text_token,
        radix: Radix::from_f64(radix)?,
    })
}

fn get_text(expression: &JsAnyExpression) -> Option<String> {
    match expression {
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
            chunk
                .template_chunk_token()
                .ok()
                .map(|t| t.text_trimmed().to_owned())
        }
        JsAnyExpression::JsAnyLiteralExpression(
            JsAnyLiteralExpression::JsStringLiteralExpression(s),
        ) => s.value_token().ok().map(|it| {
            let text = it.text_trimmed();
            text[1..text.len() - 1].to_owned()
        }),
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

fn is_ident(object: &JsAnyExpression, text: &str) -> Option<bool> {
    let ident = object.as_js_identifier_expression()?;
    let token = ident.name().ok()?.value_token().ok()?;
    Some(token.text_trimmed() == text)
}

fn get_callee(expr: &JsCallExpression) -> Option<&'static str> {
    let callee = expr.callee().ok()?;

    match &callee {
        JsAnyExpression::JsIdentifierExpression(..) => {
            if is_ident(&callee, "parseInt")? {
                return Some("parseInt()");
            }
        }
        JsAnyExpression::JsStaticMemberExpression(expr) => {
            let object = expr.object().ok()?;
            let member = expr.member().ok()?;
            if is_ident(&object, "Number")? && member.syntax().text_trimmed() == "parseInt" {
                return Some("Number.parseInt()");
            }
        }
        JsAnyExpression::JsComputedMemberExpression(expr) => {
            let object = expr.object().ok()?;
            let member = expr.member().ok()?;
            if is_ident(&object, "Number")? && get_text(&member)? == "parseInt" {
                return Some("Number.parseInt()");
            }
        }
        _ => (),
    }
    None
}

#[derive(Copy, Clone)]
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
