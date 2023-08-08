use crate::{control_flow::AnyJsControlFlowRoot, semantic_services::Semantic, JsRuleAction};
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_semantic::ReferencesExtensions;
use rome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, AnyJsExpression, JsArrowFunctionExpression,
    JsAssignmentExpression, JsExpressionStatement, JsIdentifierBinding, JsIdentifierExpression,
    JsThisExpression, JsVariableDeclaration, JsVariableDeclarator, T,
};
use rome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};

declare_rule! {
    /// Disallow useless `this` aliasing.
    ///
    /// Arrow functions inherits `this` from their enclosing scope.
    /// This makes `this` aliasing useless in this situation.
    ///
    /// Credits: https://typescript-eslint.io/rules/no-this-alias/
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class A {
    ///     method() {
    ///         const self = this;
    ///         return () => {
    ///             return self;
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// class A {
    ///     method() {
    ///         const self = this;
    ///         return function() {
    ///             this.g();
    ///             return self;
    ///         }
    ///     }
    /// }
    /// ```
    ///
    pub(crate) NoUselessThisAlias {
        version: "next",
        name: "noUselessThisAlias",
        recommended: true,
    }
}

impl Rule for NoUselessThisAlias {
    type Query = Semantic<JsVariableDeclarator>;
    type State = JsIdentifierBinding;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let declarator = ctx.query();
        let model = ctx.model();
        let mut is_this_alias = false;
        if let Some(initializer) = declarator.initializer() {
            let initializer = initializer.expression().ok()?.omit_parentheses();
            if !JsThisExpression::can_cast(initializer.syntax().kind()) {
                return None;
            }
            is_this_alias = true;
        };
        let Ok(AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(id))) = declarator.id() else {
            // Ignore destructuring
            return None;
        };
        let this_scope = declarator
            .syntax()
            .ancestors()
            .find_map(AnyJsControlFlowRoot::cast)?;
        for write in id.all_writes(model) {
            let assign = JsAssignmentExpression::cast(write.syntax().parent()?)?;
            let assign_right = assign.right().ok()?.omit_parentheses();
            if !JsThisExpression::can_cast(assign_right.syntax().kind()) {
                return None;
            }
            is_this_alias = true;
        }
        if !is_this_alias {
            return None;
        }
        for reference in id.all_references(model) {
            let current_this_scope = reference
                .syntax()
                .ancestors()
                .filter(|x| !JsArrowFunctionExpression::can_cast(x.kind()))
                .find_map(AnyJsControlFlowRoot::cast)?;
            if this_scope != current_this_scope {
                // The aliasing is required because they have not the same `this` scope.
                return None;
            }
        }
        Some(id)
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let declarator = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                declarator.range(),
                markup! {
                    "This aliasing of "<Emphasis>"this"</Emphasis>" is unnecessary."
                },
            )
            .note(markup! {
                "This note will give you more information."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, id: &Self::State) -> Option<JsRuleAction> {
        let declarator = ctx.query();
        let model = ctx.model();
        let Some(var_decl) = declarator.syntax().ancestors().find_map(JsVariableDeclaration::cast) else {
            return None;
        };
        let mut mutation = ctx.root().begin();
        let this_expr = AnyJsExpression::from(make::js_this_expression(make::token(T![this])));
        for read in id.all_reads(model) {
            let syntax = read.syntax();
            let syntax = syntax.parent()?;
            let Some(expr) = JsIdentifierExpression::cast(syntax) else {
                return None;
            };
            mutation.replace_node(expr.into(), this_expr.clone());
        }
        for write in id.all_writes(model) {
            let syntax = write.syntax();
            let syntax = syntax.parent()?;
            let Some(statement) = JsExpressionStatement::cast(syntax.parent()?) else {
                return None;
            };
            mutation.remove_node(statement);
        }
        let var_declarator_list = var_decl.declarators();
        if var_declarator_list.len() == 1 {
            mutation.remove_node(var_decl);
        } else {
            let mut deleted_comma = None;
            for (current_declarator, current_comma) in var_declarator_list
                .iter()
                .zip(var_declarator_list.separators())
            {
                deleted_comma = current_comma.ok();
                let current_declarator = current_declarator.ok()?;
                if &current_declarator == declarator {
                    break;
                }
            }
            mutation.remove_node(declarator.clone());
            mutation.remove_token(deleted_comma?);
        }
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! {
                "Use "<Emphasis>"this"</Emphasis>" instead of an alias."
            }
            .to_owned(),
            mutation,
        })
    }
}
