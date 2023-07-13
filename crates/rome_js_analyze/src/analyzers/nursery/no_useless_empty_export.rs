use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{AnyJsModuleItem, JsExport, JsModuleItemList, JsSyntaxToken};
use rome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow empty exports that don't change anything in a module file.
    ///
    /// An empty `export {}` is sometimes useful to turn a file that would otherwise be a script into a module.
    /// Per the [TypeScript Handbook Modules page](https://www.typescriptlang.org/docs/handbook/modules.html):
    ///
    /// > In TypeScript, just as in ECMAScript 2015,
    /// > any file containing a top-level import or export is considered a module.
    /// > Conversely, a file without any top-level import or export declarations is treated as a script
    /// > whose contents are available in the global scope.
    ///
    /// However, an `export {}` statement does nothing if there are any other top-level import or export in the file.
    ///
    /// Source: https://typescript-eslint.io/rules/no-useless-empty-export/
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { A } from "module";
    /// export {};
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// export const A = 0;
    /// export {};
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// export {};
    /// ```
    ///
    pub(crate) NoUselessEmptyExport {
        version: "next",
        name: "noUselessEmptyExport",
        recommended: true,
    }
}

impl Rule for NoUselessEmptyExport {
    type Query = Ast<JsExport>;
    /// The first import or export that makes useless the empty export.
    type State = JsSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if is_empty_export(node) {
            let module_item_list = JsModuleItemList::cast(node.syntax().parent()?)?;
            // allow reporting an empty export that precedes another empty export.
            let mut ignore_empty_export = true;
            for module_item in module_item_list {
                match module_item {
                    AnyJsModuleItem::AnyJsStatement(_) => {}
                    AnyJsModuleItem::JsImport(import) => return import.import_token().ok(),
                    AnyJsModuleItem::JsExport(export) => {
                        if !is_empty_export(&export) {
                            return export.export_token().ok();
                        }
                        if !ignore_empty_export {
                            return export.export_token().ok();
                        }
                        if node == &export {
                            ignore_empty_export = false
                        }
                    }
                }
            }
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, token: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This empty "<Emphasis>"export"</Emphasis>" is useless because there's another "<Emphasis>"export"</Emphasis>" or "<Emphasis>"import"</Emphasis>"."
                },
            ).detail(token.text_trimmed_range(), markup! {
                "This "<Emphasis>{token.text_trimmed()}</Emphasis>" makes useless the empty export."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        mutation.remove_node(ctx.query().clone());
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! { "Remove this useless empty export." }.to_owned(),
            mutation,
        })
    }
}

fn is_empty_export(export: &JsExport) -> bool {
    (|| -> Option<bool> {
        Some(
            export
                .export_clause()
                .ok()?
                .as_js_export_named_clause()?
                .specifiers()
                .iter()
                .count()
                == 0,
        )
    })()
    .unwrap_or(false)
}
