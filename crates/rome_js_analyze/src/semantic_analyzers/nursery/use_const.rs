use crate::{semantic_services::Semantic, JsRuleAction};
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Rule, RuleDiagnostic};
use rome_console::markup;

use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_semantic::{AllReferencesExtensions, Scope, SemanticModel, SemanticScopeExtensions};
use rome_js_syntax::*;
use rome_rowan::{declare_node_union, AstNode, BatchMutationExt};

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

declare_node_union! {
    pub(crate) DestructuringHost = JsVariableDeclarator | JsAssignmentExpression
}

pub(crate) struct ConstantBinding {
    binding: JsIdentifierBinding,
    fix: bool,
}

impl Rule for UseConst {
    type Query = Semantic<VarDecl>;
    type State = Vec<Option<ConstantBinding>>;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let declaration = ctx.query();
        let model = ctx.model();

        if !declaration.is_let() {
            return None;
        }

        let mut signals = Vec::new();
        for declarator in declaration.declarators() {
            for binding in declarator_bindings(&declarator) {
                let info = ConstantBinding::from_binding(binding, declaration, &declarator, model);
                signals.push(info);
            }
        }
        Some(signals)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let decl = ctx.query();
        let kind = decl.kind_token()?;
        let mut diag = RuleDiagnostic::new(
            rule_category!(),
            kind.text_trimmed_range(),
            markup! {
                "Use 'const' when variables are not reassigned."
            },
        );

        let mut found = false;
        for info in state.iter().flat_map(Option::as_ref) {
            found = true;
            let binding = info.binding.name_token().ok()?;
            diag = diag.detail(
                binding.text_trimmed_range(),
                markup! {
                    "'"{ binding.text_trimmed() }"' is never reassigned."
                },
            );
        }

        if found {
            Some(diag)
        } else {
            None
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let decl = ctx.query();
        if state
            .iter()
            .all(|it| matches!(it, Some(ConstantBinding { fix: true, .. })))
        {
            let mut batch = ctx.root().begin();
            batch.replace_token(decl.kind_token()?, make::token(JsSyntaxKind::CONST_KW));
            Some(JsRuleAction {
                category: ActionCategory::QuickFix,
                applicability: Applicability::MaybeIncorrect,
                message: markup! { "Use 'const' instead." }.to_owned(),
                mutation: batch,
            })
        } else {
            None
        }
    }
}

impl ConstantBinding {
    fn from_binding(
        binding: JsIdentifierBinding,
        decl: &VarDecl,
        declarator: &JsVariableDeclarator,
        model: &SemanticModel,
    ) -> Option<Self> {
        // Inside a for-loop init
        if decl.parent::<JsForStatement>().is_some() {
            return None;
        }

        let mut writes = binding.all_writes(model);

        // In a for-in or for-of loop or if it has an initializer
        if matches!(decl, VarDecl::JsForVariableDeclaration(..))
            || declarator.initializer().is_some()
        {
            return if writes.len() == 0 {
                Some(ConstantBinding { binding, fix: true })
            } else {
                None
            };
        }

        // If no initializer and one assignment in same scope
        let write = match (writes.next(), writes.next()) {
            (Some(v), None) if v.scope() == binding.scope(model) => v,
            _ => return None,
        };

        let host = write.node().ancestors().find_map(DestructuringHost::cast)?;
        if host.has_member_expr_assignment() || host.has_outer_variables(write.scope()) {
            return None;
        }

        if host.can_become_variable_declaration().unwrap_or(false) {
            Some(ConstantBinding {
                binding,
                fix: false,
            })
        } else {
            None
        }
    }
}

impl VarDecl {
    fn declarators(&self) -> impl Iterator<Item = JsVariableDeclarator> {
        self.syntax()
            .descendants()
            .filter_map(JsVariableDeclarator::cast)
    }

    fn kind_token(&self) -> Option<JsSyntaxToken> {
        match self {
            VarDecl::JsVariableDeclaration(x) => x.kind().ok(),
            VarDecl::JsForVariableDeclaration(x) => x.kind_token().ok(),
        }
    }

    fn is_let(&self) -> bool {
        match self {
            VarDecl::JsVariableDeclaration(it) => it.is_let(),
            VarDecl::JsForVariableDeclaration(it) => it
                .kind_token()
                .map_or(false, |it| it.kind() == JsSyntaxKind::LET_KW),
        }
    }
}

fn declarator_bindings(decl: &JsVariableDeclarator) -> impl Iterator<Item = JsIdentifierBinding> {
    decl.id()
        .into_iter()
        .flat_map(|it| it.syntax().descendants())
        .filter_map(JsIdentifierBinding::cast)
}

impl DestructuringHost {
    fn can_become_variable_declaration(&self) -> Option<bool> {
        match self {
            Self::JsVariableDeclarator(_) => Some(true),
            Self::JsAssignmentExpression(e) => {
                let mut parent = e.syntax().parent()?;
                while parent.kind() == JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION {
                    parent = parent.parent()?;
                }

                if parent.kind() == JsSyntaxKind::JS_EXPRESSION_STATEMENT {
                    parent = parent.parent()?;
                    Some(
                        parent.kind() == JsSyntaxKind::JS_STATEMENT_LIST
                            || parent.kind() == JsSyntaxKind::JS_MODULE_ITEM_LIST,
                    )
                } else {
                    None
                }
            }
        }
    }

    fn has_member_expr_assignment(&self) -> bool {
        match self {
            Self::JsAssignmentExpression(it) => it
                .left()
                .map_or(false, |pat| has_member_expr_in_assign_pat(pat)),
            _ => false,
        }
    }

    fn has_outer_variables(&self, scope: Scope) -> bool {
        match self {
            Self::JsVariableDeclarator(it) => it
                .id()
                .map_or(false, |pat| has_outer_variables_in_binding_pat(pat, scope)),
            Self::JsAssignmentExpression(it) => it
                .left()
                .map_or(false, |pat| has_outer_variables_in_assign_pat(pat, scope)),
        }
    }
}

fn has_outer_variables_in_binding_pat(pat: JsAnyBindingPattern, scope: Scope) -> bool {
    pat.syntax()
        .descendants()
        .filter_map(JsIdentifierBinding::cast)
        .any(|it| is_outer_variable_in_binding(it, &scope))
}

fn has_outer_variables_in_assign_pat(pat: JsAnyAssignmentPattern, scope: Scope) -> bool {
    pat.syntax()
        .descendants()
        .filter_map(JsIdentifierAssignment::cast)
        .any(|it| is_outer_variable_in_assignment(it, &scope))
}

fn has_member_expr_in_assign_pat(pat: JsAnyAssignmentPattern) -> bool {
    pat.syntax()
        .descendants()
        .filter_map(JsAnyAssignment::cast)
        .any(is_member_expr_assignment)
}

fn is_member_expr_assignment(mut assignment: JsAnyAssignment) -> bool {
    use JsAnyAssignment::*;
    while let JsParenthesizedAssignment(p) = assignment {
        if let Ok(p) = p.assignment() {
            assignment = p
        } else {
            return false;
        }
    }
    matches!(
        assignment,
        JsComputedMemberAssignment(_) | JsStaticMemberAssignment(_)
    )
}

fn is_outer_variable_in_binding(binding: JsIdentifierBinding, scope: &Scope) -> bool {
    binding
        .name_token()
        .map_or(false, |name| is_binding_in_outer_scopes(scope, name))
}

fn is_outer_variable_in_assignment(assignment: JsIdentifierAssignment, scope: &Scope) -> bool {
    assignment
        .name_token()
        .map_or(false, |name| is_binding_in_outer_scopes(scope, name))
}

fn is_binding_in_outer_scopes(scope: &Scope, name: JsSyntaxToken) -> bool {
    let text = name.text_trimmed();
    scope
        .ancestors()
        .skip(1) // Skip current scope
        .any(|scope| scope.get_binding(text).is_some())
}
