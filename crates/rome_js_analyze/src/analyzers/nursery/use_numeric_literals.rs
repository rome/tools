use crate::{ast_utils, JsRuleAction};
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsCallExpression, JsSyntaxElement, JsSyntaxKind,
    JsSyntaxToken, TriviaPieceKind,
};
use rome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TriviaPiece};

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
    pub(crate) UseNumericLiterals {
        version: "11.0.0",
        name: "useNumericLiterals",
        recommended: false,
    }
}

pub struct CallInfo {
    callee: &'static str,
    text: String,
    radix: Radix,
}

impl Rule for UseNumericLiterals {
    type Query = Ast<JsCallExpression>;
    type State = CallInfo;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        CallInfo::try_from_node(node)
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

    fn action(ctx: &RuleContext<Self>, call: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let number = call.to_numeric_literal()?;
        let number = attach_trivia(number, node);

        mutation.replace_node_discard_trivia(
            JsAnyExpression::JsCallExpression(node.clone()),
            JsAnyExpression::JsAnyLiteralExpression(
                JsAnyLiteralExpression::JsNumberLiteralExpression(
                    make::js_number_literal_expression(number),
                ),
            ),
        );

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! { "Replace with "{call.radix.description()}" literals" }.to_owned(),
            mutation,
        })
    }
}

impl CallInfo {
    fn try_from_node(expr: &JsCallExpression) -> Option<CallInfo> {
        let args = expr.arguments().ok()?.args();
        if args.len() != 2 {
            return None;
        }
        let mut args = args.into_iter();
        let text = args.next()?.ok()?;
        let radix = args.next()?.ok()?;
        let callee = get_callee(expr)?;
        let text = ast_utils::as_static_text(&text)?;
        let radix = ast_utils::as_number(&radix)?;
        Some(CallInfo {
            callee,
            text,
            radix: Radix::from_f64(radix)?,
        })
    }

    fn to_numeric_literal(&self) -> Option<String> {
        i128::from_str_radix(&self.text, self.radix as u32).ok()?;
        let number = format!("{}{}", self.radix.prefix(), self.text);
        Some(number)
    }
}

fn attach_trivia(number: String, source: &JsCallExpression) -> JsSyntaxToken {
    let mut text = String::new();
    let node = source.syntax();
    let mut leading_trivia = vec![];
    let mut trailing_trivia = vec![];

    if let Some(token) = node.first_token() {
        for t in token.leading_trivia().pieces() {
            text.push_str(t.text());
            leading_trivia.push(TriviaPiece::new(t.kind(), t.text_len()));
        }
    }
    add_whitespace(&mut leading_trivia, &mut text, node.prev_sibling_or_token());
    text.push_str(&number);
    if let Some(token) = node.last_token() {
        for t in token.trailing_trivia().pieces() {
            text.push_str(t.text());
            trailing_trivia.push(TriviaPiece::new(t.kind(), t.text_len()));
        }
    }
    add_whitespace(
        &mut trailing_trivia,
        &mut text,
        node.next_sibling_or_token(),
    );

    JsSyntaxToken::new_detached(
        JsSyntaxKind::JS_NUMBER_LITERAL,
        &text,
        leading_trivia,
        trailing_trivia,
    )
}

fn add_whitespace(
    trivia: &mut Vec<TriviaPiece>,
    text: &mut String,
    element: Option<JsSyntaxElement>,
) {
    if !trivia.is_empty() {
        return;
    }
    match element {
        Some(JsSyntaxElement::Token(token))
            if !token.kind().is_trivia() && !token.kind().is_punct() =>
        {
            text.push(' ');
            trivia.push(TriviaPiece::new(TriviaPieceKind::Whitespace, 1));
        }
        _ => (),
    }
}

fn get_callee(expr: &JsCallExpression) -> Option<&'static str> {
    let callee = expr.callee().ok()?;
    if ast_utils::is_specific_id(&callee, "parseInt") {
        return Some("parseInt()");
    }
    if ast_utils::is_specific_member_access(&callee, "Number", "parseInt") {
        return Some("Number.parseInt()");
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
