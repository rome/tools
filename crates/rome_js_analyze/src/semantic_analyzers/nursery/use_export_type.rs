use crate::{semantic_services::Semantic, JsRuleAction};
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    AnyJsExportNamedSpecifier, JsExportNamedClause, JsExportNamedSpecifierList, JsFileSource, T,
};
use rome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TextRange, TriviaPieceKind};

declare_rule! {
    /// Promotes the use of `export type` for type-only types.
    ///
    /// _TypeScript_ allows specifying a `type` keyword on an `export` to indicate that
    /// the `export` doesn't exist at runtime.
    /// This will enable transpilers to safely drop exports of types without looking for its definition.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// interface I {}
    /// export { I };
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type T = number;
    /// export { T };
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// import type { T } from "./mod.js";
    /// export { T };
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// class C {}
    /// function f() {}
    /// export { C, f };
    /// ```
    ///
    /// This rules applies only to identifiers locally defined.
    /// It doesn't warn against a type exported as a value in re-export clause such as:
    ///
    /// ```ts
    /// export { TypeA } from "./mod.ts"
    /// ```
    pub(crate) UseExportType {
        version: "next",
        name: "useExportType",
        recommended: true,
    }
}

impl Rule for UseExportType {
    type Query = Semantic<AnyJsExportNamedSpecifier>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source_type = ctx.source_type::<JsFileSource>();
        if !source_type.language().is_typescript() {
            return None;
        }
        let specifier = ctx.query();
        let model = ctx.model();
        if specifier.is_type_only() {
            return None;
        }
        let reference = specifier.local_name().ok()?;
        let binding = model.binding(&reference)?;
        let binding = binding.tree();
        if binding.is_type_only() {
            return Some(binding.range());
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, declaration: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This export is only a type and should thus use "<Emphasis>"export type"</Emphasis>"."
                },
            ).detail(declaration, markup! {
                "The type is defined here."
            })
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let specifier = ctx.query();
        let mut mutation = ctx.root().begin();
        let type_token =
            Some(make::token(T![type]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]));
        if let Some(specifier_list) = specifier.parent::<JsExportNamedSpecifierList>() {
            if specifier_list.iter().count() == 1 {
                if let Some(export) = specifier_list.parent::<JsExportNamedClause>() {
                    let new_export = export.clone().with_type_token(type_token);
                    mutation.replace_node(export, new_export);
                    return Some(JsRuleAction {
                        category: ActionCategory::QuickFix,
                        applicability: Applicability::Always,
                        message: markup! { "Use "<Emphasis>"export type"</Emphasis>"." }.to_owned(),
                        mutation,
                    });
                }
            }
        }
        mutation.replace_node_discard_trivia(
            specifier.clone(),
            specifier.clone().with_type_token(type_token),
        );
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! { "Use an inline type export." }.to_owned(),
            mutation,
        })
    }
}
