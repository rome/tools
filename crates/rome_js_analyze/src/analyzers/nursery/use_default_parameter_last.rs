use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{JsFormalParameter, JsInitializerClause, JsSyntaxToken, TsPropertyParameter};
use rome_rowan::{declare_node_union, AstNode, BatchMutationExt, Direction};

use crate::JsRuleAction;

declare_rule! {
    /// Enforce default function parameters and optional parameters to be last.
    ///
    /// Default and optional parameters that precede a required parameter cannot be omitted at call site.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function f(a = 0, b) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function f(a, b = 0, c) {}
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function f(a: number, b?: number, c: number) {}
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// class Foo {
    ///     constructor(readonly a = 10, readonly b: number) {}
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function f(a, b = 0) {}
    /// ```
    ///
    /// ```ts
    /// function f(a: number, b?: number, c = 0) {}
    /// ```
    ///
    /// ```ts
    /// function f(a: number, b = 0, c?: number) {}
    /// ```
    ///
    pub(crate) UseDefaultParameterLast {
        version: "11.0.0",
        name: "useDefaultParameterLast",
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) AnyFormalParameter = JsFormalParameter | TsPropertyParameter
}

impl AnyFormalParameter {
    pub(crate) fn is_optional(&self) -> bool {
        self.question_mark_token().is_some() || self.initializer().is_some()
    }

    pub(crate) fn initializer(&self) -> Option<JsInitializerClause> {
        match self {
            AnyFormalParameter::JsFormalParameter(x) => x.initializer(),
            AnyFormalParameter::TsPropertyParameter(x) => x
                .formal_parameter()
                .ok()?
                .as_js_formal_parameter()?
                .initializer(),
        }
    }

    pub(crate) fn question_mark_token(&self) -> Option<JsSyntaxToken> {
        match self {
            AnyFormalParameter::JsFormalParameter(x) => x.question_mark_token(),
            AnyFormalParameter::TsPropertyParameter(x) => x
                .formal_parameter()
                .ok()?
                .as_js_formal_parameter()?
                .question_mark_token(),
        }
    }
}

impl Rule for UseDefaultParameterLast {
    type Query = Ast<AnyFormalParameter>;
    type State = AnyFormalParameter;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let formal_param = ctx.query();
        if formal_param.is_optional() {
            let next_required_param = formal_param
                .syntax()
                .siblings(Direction::Next)
                .filter_map(AnyFormalParameter::cast)
                .find(|x| !x.is_optional());
            next_required_param
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, required_param: &Self::State) -> Option<RuleDiagnostic> {
        let formal_param = ctx.query();
        let param_kind = if formal_param.question_mark_token().is_some() {
            "optional"
        } else {
            "default"
        };
        Some(RuleDiagnostic::new(
            rule_category!(),
            formal_param.range(),
            markup! {
                "The "<Emphasis>{param_kind}" parameter"</Emphasis>" should follow the "<Emphasis>"required parameter"</Emphasis>" or should be a "<Emphasis>"required parameter"</Emphasis>"."
            },
        ).detail(
            required_param.range(),
            markup! {
                "The "<Emphasis>"required parameter"</Emphasis>" is here:"
            },
        ).note(
            markup! {
                "A "<Emphasis>{param_kind}" parameter"</Emphasis>" that precedes a "<Emphasis>"required parameter"</Emphasis>" cannot be omitted at call site."
            }
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let opt_param = ctx.query();
        let mut mutation = ctx.root().begin();
        if opt_param.question_mark_token().is_some() {
            let question_mark = opt_param.question_mark_token()?;
            let next_token = question_mark.next_token()?;
            let new_next_token = next_token.with_leading_trivia_pieces(
                question_mark
                    .leading_trivia()
                    .pieces()
                    .chain(question_mark.trailing_trivia().pieces())
                    .chain(next_token.leading_trivia().pieces())
                    .collect::<Vec<_>>(),
            );
            mutation.replace_token_discard_trivia(next_token, new_next_token);
            mutation.remove_token(question_mark);
        } else {
            let initializer = opt_param.initializer()?;
            let first_token = initializer.syntax().first_token()?;
            let last_token = initializer.syntax().last_token()?;
            let prev_token = first_token.prev_token()?;
            let new_prev_token = prev_token.with_trailing_trivia_pieces(
                prev_token
                    .trailing_trivia()
                    .pieces()
                    .chain(first_token.leading_trivia().pieces())
                    .chain(last_token.trailing_trivia().pieces())
                    .collect::<Vec<_>>(),
            );
            mutation.replace_token_discard_trivia(prev_token, new_prev_token);
            mutation.remove_node(initializer);
        }
        Some(JsRuleAction {
            mutation,
            message:
                markup! {"Turn the parameter into a "<Emphasis>"required parameter"</Emphasis>"."}
                    .to_owned(),
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
        })
    }
}
