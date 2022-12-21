use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::TsEnumDeclaration;
use rome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow TypeScript `const enum`
    ///
    /// Const enums are enums that should be inlined at use sites.
    /// Const enums are not supported by bundlers and are incompatible with the `isolatedModules` mode.
    /// Their use can lead to import inexistent values (because const enums are erased).
    ///
    /// Thus, library authors and bundler users should not use const enums.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// const enum Status {
    ///   Open,
    ///   Close,
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// enum Status {
    ///   Open,
    ///   Close,
    /// }
    /// ```
    pub(crate) NoConstEnum {
        version: "11.0.0",
        name: "noConstEnum",
        recommended: true,
    }
}

impl Rule for NoConstEnum {
    type Query = Ast<TsEnumDeclaration>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let enum_decl = ctx.query();
        enum_decl.const_token().and(Some(()))
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let enum_decl = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            enum_decl.range(),
            markup! {
                "The "<Emphasis>"enum declaration"</Emphasis>" should not be "<Emphasis>"const"</Emphasis>
            },
        ).note(
            "Const enums are not supported by bundlers and are incompatible with the 'isolatedModules' mode. Their use can lead to import inexistent values."
        ).note(markup! {
            "See "<Hyperlink href="https://www.typescriptlang.org/docs/handbook/enums.html#const-enum-pitfalls">"TypeSCript Docs"</Hyperlink>" for more details."
        }))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let enum_decl = ctx.query();
        let mut mutation = ctx.root().begin();
        let const_token = enum_decl.const_token()?;
        let enum_token = enum_decl.enum_token().ok()?;
        let transferred_trivia = const_token
            .leading_trivia()
            .pieces()
            .chain(
                const_token
                    .trailing_trivia()
                    .pieces()
                    .skip_while(|x| x.is_whitespace() || x.is_whitespace()),
            )
            .chain(enum_token.leading_trivia().pieces())
            .collect::<Vec<_>>();
        let new_enum_token = enum_token.with_leading_trivia_pieces(transferred_trivia);
        mutation.remove_token(const_token);
        mutation.replace_token_discard_trivia(enum_token, new_enum_token);
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! {
                "Turn the "<Emphasis>"const enum"</Emphasis>" into a regular "<Emphasis>"enum"</Emphasis>"."
            }.to_owned(),
            mutation,
        })
    }
}
