use crate::{semantic_services::Semantic, utils::batch::JsBatchMutation, JsRuleAction};
use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_semantic::{AllReferencesExtensions, Reference, SemanticScopeExtensions};
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsAnyRoot, JsFormalParameter, JsFunctionDeclaration,
    JsIdentifierBinding, JsIdentifierExpression, JsLanguage, JsStringLiteralExpression,
    JsSyntaxKind, JsVariableDeclaration, JsVariableDeclarator, JsVariableDeclaratorList,
    JsVariableStatement,
};
use rome_rowan::{AstNode, AstSeparatedList, BatchMutation, BatchMutationExt, SyntaxNodeCast};

declare_rule! {
    /// Disallow unused variables.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const a = 4;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let a = 4;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function foo(myVar) {
    ///     console.log('foo');
    /// };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function foo(myVar) {
    ///     console.log('foo');
    /// }
    /// foo();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = (b) => {
    ///     console.log('foo');
    /// };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function foo() {
    ///     foo();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = () => {
    ///     foo();
    ///     console.log(this);
    /// };
    /// ```
    ///
    /// # Valid
    ///
    /// ```js
    /// function foo(, b) {
    ///     console.log(b)
    /// };
    /// foo();
    /// ```
    pub(crate) NoUnusedVariables {
        version: "0.8.0",
        name: "noUnusedVariables",
        recommended: true,
    }
}

impl Rule for NoUnusedVariables {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Semantic<JsIdentifierBinding>;
    type State = ();
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let binding = ctx.query();
        let model = ctx.model();

        let all_references = binding.all_references(model);

        if all_references.count() == 0 {
            Some(())
        } else {
            // We need to check if all uses of this binding are somehow recursive

            let function_declaration_scope = binding
                .parent::<JsFunctionDeclaration>()
                .map(|declaration| declaration.scope(model));

            let declarator = binding.parent::<JsVariableDeclarator>();

            let mut references_outside = 0;
            for r in binding.all_references(model) {
                let reference_scope = r.scope();

                // If this binding is a function, and all its references are "inside" this
                // function, we can safely say that this function is not used
                if function_declaration_scope
                    .as_ref()
                    .map(|s| s.is_ancestor_of(&reference_scope))
                    .unwrap_or(false)
                {
                    continue;
                }

                // Another possibility is if all its references are "inside" the same declaration
                if let Some(declarator) = declarator.as_ref() {
                    let node = declarator.syntax();
                    if r.node().ancestors().find(|n| n == node).is_some() {
                        continue;
                    }
                }

                references_outside += 1;
                break;
            }

            if references_outside == 0 {
                Some(())
            } else {
                None
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let binding = ctx.query();

        let symbol_type = match binding.syntax().parent().unwrap().kind() {
            JsSyntaxKind::JS_FORMAL_PARAMETER => "parameter",
            JsSyntaxKind::JS_FUNCTION_DECLARATION => "function",
            _ => "variable",
        };

        let diag = RuleDiagnostic::warning(
            binding.syntax().text_trimmed_range(),
            markup! {
                "This " {symbol_type} " is unused."
            },
        );

        let diag = diag.footer_note(
            markup! {"Unused variables usually are result of incomplete refactoring, typos and other source of bugs."},
        );

        Some(diag)
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let root = ctx.root();
        let binding = ctx.query();

        let mut batch = root.begin();

        // If this is a function, remove the whole declaration
        if let Some(declaration) = binding.parent::<JsFunctionDeclaration>() {
            batch.remove_node(declaration)
        } else if let Some(variable_declarator) = binding.parent::<JsVariableDeclarator>() {
            batch.remove_js_variable_declarator(&variable_declarator);
        } else if let Some(formal_parameter) = binding.parent::<JsFormalParameter>() {
            batch.remove_js_formal_parameter(&formal_parameter);
        }

        Some(JsRuleAction {
            category: ActionCategory::Refactor,
            applicability: Applicability::Unspecified,
            message: markup! { "Remove dead code." }.to_owned(),
            mutation: batch,
        })
    }
}
