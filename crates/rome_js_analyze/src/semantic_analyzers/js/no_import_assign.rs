use crate::semantic_services::Semantic;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    JsDefaultImportSpecifier, JsIdentifierAssignment, JsIdentifierBinding, JsImportDefaultClause,
    JsImportNamespaceClause, JsNamedImportSpecifier, JsNamespaceImportSpecifier,
    JsShorthandNamedImportSpecifier,
};

use rome_rowan::{declare_node_union, AstNode};

declare_rule! {
    ///  Disallow assigning to imported bindings
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import x from "y";
    /// x = 1;
    /// ```
    /// ```js,expect_diagnostic
    /// import y from "y";
    /// [y] = 1;
    /// ```
    /// ```js,expect_diagnostic
    /// import z from "y";
    /// ({ z } = 1); /// ```
    /// ```js,expect_diagnostic
    /// import a from "y";
    /// [...a] = 1;
    /// ```
    /// ```js,expect_diagnostic
    /// import b from "y";
    /// ({ ...b } = 1);
    /// ```
    /// ```js,expect_diagnostic
    /// import c from "y";
    /// for (c in y) {};
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import d from "y";
    /// d += 1;
    /// ```
    /// ```js,expect_diagnostic
    /// import * as e from "y";
    /// e = 1;
    /// ```
    pub(crate) NoImportAssign {
        version: "0.9.0",
        name: "noImportAssign",
        recommended: true,
    }
}

impl Rule for NoImportAssign {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Semantic<JsAnyImportLike>;
    /// The first element of the tuple is the invalid `JsIdentifierAssignment`, the second element of the tuple is the imported `JsIdentifierBinding`.
    type State = (JsIdentifierAssignment, JsIdentifierBinding);
    type Signals = Vec<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Vec<Self::State> {
        let label_statement = ctx.query();
        let mut invalid_assign_list = vec![];
        let local_name_binding = match label_statement {
            // `import xx from 'y'`
            JsAnyImportLike::JsImportDefaultClause(clause) => clause.local_name().ok(),
            // `import * as xxx from 'y'`
            JsAnyImportLike::JsImportNamespaceClause(clause) => clause.local_name().ok(),
            // `import {x as xx} from 'y'`
            //          ^^^^^^^
            JsAnyImportLike::JsNamedImportSpecifier(specifier) => specifier.local_name().ok(),
            // `import {x} from 'y'`
            //          ^
            JsAnyImportLike::JsShorthandNamedImportSpecifier(specifier) => {
                specifier.local_name().ok()
            }
            // `import a, * as b from 'y'`
            //            ^^^^^^
            JsAnyImportLike::JsNamespaceImportSpecifier(specifier) => specifier.local_name().ok(),
            // `import a, * as b from 'y'`
            //         ^
            JsAnyImportLike::JsDefaultImportSpecifier(specifier) => specifier.local_name().ok(),
        };
        local_name_binding
            .and_then(|binding| {
                let ident_binding = binding.as_js_identifier_binding()?;
                let model = ctx.model();
                for reference in model.all_writes(ident_binding) {
                    invalid_assign_list.push((
                        JsIdentifierAssignment::cast(reference.node().clone())?,
                        ident_binding.clone(),
                    ));
                }
                Some(invalid_assign_list)
            })
            .unwrap_or_default()
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (invalid_assign, import_binding) = state;
        let name = invalid_assign.syntax().text_trimmed();

        Some(
            RuleDiagnostic::warning(
                invalid_assign.syntax().text_trimmed_range(),
                markup! {
                    "The imported variable "<Emphasis>{name.to_string()}</Emphasis>" is read-only"
                },
            )
            .footer_note(markup! {"Use a local variable instead of reassigning an import."})
            .secondary(
                import_binding.syntax().text_trimmed_range(),
                markup! {
                    "The variable is imported here"
                },
            ),
        )
    }
}

declare_node_union! {
    pub(crate) JsAnyImportLike = JsImportDefaultClause | JsImportNamespaceClause | JsNamedImportSpecifier | JsShorthandNamedImportSpecifier | JsNamespaceImportSpecifier | JsDefaultImportSpecifier
}
