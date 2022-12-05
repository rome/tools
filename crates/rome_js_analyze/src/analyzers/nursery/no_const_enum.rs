use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{JsSyntaxToken, TsEnumDeclaration};
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
    type State = JsSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let enum_decl = ctx.query();
        enum_decl.const_token()
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

    fn action(ctx: &RuleContext<Self>, const_token: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        mutation.remove_token(const_token.to_owned());
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
