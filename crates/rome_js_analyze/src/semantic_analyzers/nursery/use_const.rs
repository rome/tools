use crate::{semantic_services::Semantic, JsRuleAction};
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;

use rome_js_semantic::{AllReferencesExtensions, SemanticModel};
use rome_js_syntax::{
    JsForStatement, JsForVariableDeclaration, JsIdentifierBinding, JsSyntaxKind,
    JsVariableDeclaration, JsVariableDeclarator,
};
use rome_rowan::{declare_node_union, AstNode};

declare_rule! {
    /// Require `const` declarations for variables that are never reassigned after declared.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let a = 3;
    /// console.log(a);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// // `i` is redefined (not reassigned) on each loop step.
    /// for (let a of [1, 2, 3]) {
    ///     console.log(a);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// // `a` is redefined (not reassigned) on each loop step.
    /// for (let a in [1, 2, 3]) {
    ///     console.log(a);
    /// }
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// let a = 2;
    /// a = 3;
    /// console.log(a);
    /// ```
    pub(crate) UseConst {
        version: "11.0.0",
        name: "useConst",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) VarDecl = JsVariableDeclaration | JsForVariableDeclaration
}

impl Rule for UseConst {
    type Query = Semantic<JsIdentifierBinding>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let binding = ctx.query();
        let model = ctx.model();

        let should_be_const = should_change_to_const(binding, model).unwrap_or(false);
        if should_be_const {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let binding = ctx.query();
        let token = binding.name_token().ok()?;

        let diag = RuleDiagnostic::new(
            rule_category!(),
            token.text_trimmed_range(),
            markup! {
                "'"{ token.text_trimmed() }"' is never reassigned. Use 'const' instead."
            },
        );

        Some(diag)
    }

    fn action(_ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        None
    }
}

fn should_change_to_const(binding: &JsIdentifierBinding, model: &SemanticModel) -> Option<bool> {
    let declarator = binding
        .syntax()
        .ancestors()
        .find_map(JsVariableDeclarator::cast)?;

    let decl = declarator.syntax().ancestors().find_map(VarDecl::cast)?;

    match decl {
        VarDecl::JsVariableDeclaration(decl) => {
            if !decl.is_let()
                || decl.parent::<JsForStatement>().is_some()
                || declarator.initializer().is_none()
            {
                return None;
            }
        }
        VarDecl::JsForVariableDeclaration(decl) => {
            if decl.kind_token().ok()?.kind() != JsSyntaxKind::LET_KW {
                return None;
            }
        }
    }

    Some(binding.all_writes(model).len() == 0)
}
