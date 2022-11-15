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
    /// // `a` is redefined (not reassigned) on each loop step.
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
    /// ```js,expect_diagnostic
    /// let a = 3;
    /// {
    ///     let a = 4;
    ///     a = 2;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let a = 1, b = 2;
    /// b = 3;
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
    pub(crate) VariableDeclaration = JsVariableDeclaration | JsForVariableDeclaration
}

declare_node_union! {
    pub(crate) DestructuringHost = JsVariableDeclarator | JsAssignmentExpression
}

pub(crate) struct ConstBindings {
    can_be_const: Vec<JsIdentifierBinding>,
    can_fix: bool,
}

enum ConstCheckResult {
    Fix,
    Report,
}

impl Rule for UseConst {
    type Query = Semantic<VariableDeclaration>;
    type State = ConstBindings;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let declaration = ctx.query();
        let model = ctx.model();

        // Not a let declaration or inside a for-loop init
        if !declaration.is_let() || declaration.parent::<JsForStatement>().is_some() {
            return None;
        }

        ConstBindings::new(declaration, model)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let declaration = ctx.query();
        let kind = declaration.kind_token()?;
        let title = if state.can_be_const.len() == 1 {
            "This 'let' declares a variable which is never re-assigned."
        } else {
            "This 'let' declares some variables which are never re-assigned."
        };
        let mut diag = RuleDiagnostic::new(rule_category!(), kind.text_trimmed_range(), title);

        for binding in state.can_be_const.iter() {
            let binding = binding.name_token().ok()?;
            diag = diag.detail(
                binding.text_trimmed_range(),
                markup! {
                    "'"{ binding.text_trimmed() }"' is never re-assigned."
                },
            );
        }

        Some(diag)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let declaration = ctx.query();
        if state.can_fix {
            let mut batch = ctx.root().begin();
            batch.replace_token(
                declaration.kind_token()?,
                make::token(JsSyntaxKind::CONST_KW),
            );
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

impl ConstBindings {
    fn new(declaration: &VariableDeclaration, model: &SemanticModel) -> Option<Self> {
        let mut state = Self {
            can_be_const: Vec::new(),
            can_fix: true,
        };
        let in_for_in_or_of_loop = matches!(
            declaration,
            VariableDeclaration::JsForVariableDeclaration(..)
        );
        declaration.for_each_binding(|binding, declarator| {
            let has_initializer = declarator.initializer().is_some();
            let fix =
                check_binding_can_be_const(&binding, in_for_in_or_of_loop, has_initializer, model);
            match fix {
                Some(ConstCheckResult::Fix) => state.can_be_const.push(binding),
                Some(ConstCheckResult::Report) => {
                    state.can_be_const.push(binding);
                    state.can_fix = false;
                }
                None => state.can_fix = false,
            }
        });
        if state.can_be_const.is_empty() {
            None
        } else {
            Some(state)
        }
    }
}

/// Check if a binding can be const
fn check_binding_can_be_const(
    binding: &JsIdentifierBinding,
    in_for_in_or_of_loop: bool,
    has_initializer: bool,
    model: &SemanticModel,
) -> Option<ConstCheckResult> {
    let mut writes = binding.all_writes(model);

    // In a for-in or for-of loop or if it has an initializer
    if in_for_in_or_of_loop || has_initializer {
        return if writes.len() == 0 {
            Some(ConstCheckResult::Fix)
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

    if host.can_become_variable_declaration()? {
        Some(ConstCheckResult::Report)
    } else {
        None
    }
}

impl VariableDeclaration {
    fn for_each_declarator(&self, f: impl FnMut(JsVariableDeclarator)) {
        match self {
            VariableDeclaration::JsVariableDeclaration(x) => x
                .declarators()
                .into_iter()
                .filter_map(Result::ok)
                .for_each(f),
            VariableDeclaration::JsForVariableDeclaration(x) => {
                x.declarator().into_iter().for_each(f)
            }
        }
    }

    fn for_each_binding(&self, mut f: impl FnMut(JsIdentifierBinding, &JsVariableDeclarator)) {
        self.for_each_declarator(|declarator| {
            if let Ok(pattern) = declarator.id() {
                with_binding_pat_identifiers(pattern, &mut |binding| {
                    f(binding, &declarator);
                    false
                });
            }
        });
    }

    fn kind_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsVariableDeclaration(x) => x.kind().ok(),
            Self::JsForVariableDeclaration(x) => x.kind_token().ok(),
        }
    }

    fn is_let(&self) -> bool {
        match self {
            Self::JsVariableDeclaration(it) => it.is_let(),
            Self::JsForVariableDeclaration(it) => it.is_let(),
        }
    }
}

/// Visit [JsIdentifierBinding] in the given [JsAnyBindingPattern].
///
/// Traversal stops if the given function returns true.
fn with_binding_pat_identifiers(
    pat: JsAnyBindingPattern,
    f: &mut impl FnMut(JsIdentifierBinding) -> bool,
) -> bool {
    match pat {
        JsAnyBindingPattern::JsAnyBinding(id) => with_binding_identifier(id, f),
        JsAnyBindingPattern::JsArrayBindingPattern(p) => with_array_binding_pat_identifiers(p, f),
        JsAnyBindingPattern::JsObjectBindingPattern(p) => with_object_binding_pat_identifiers(p, f),
    }
}

fn with_object_binding_pat_identifiers(
    pat: JsObjectBindingPattern,
    f: &mut impl FnMut(JsIdentifierBinding) -> bool,
) -> bool {
    pat.properties()
        .into_iter()
        .filter_map(Result::ok)
        .any(|it| {
            use JsAnyObjectBindingPatternMember as P;
            match it {
                P::JsObjectBindingPatternProperty(p) => p
                    .pattern()
                    .map_or(false, |it| with_binding_pat_identifiers(it, f)),
                P::JsObjectBindingPatternRest(p) => p
                    .binding()
                    .map_or(false, |it| with_binding_identifier(it, f)),
                P::JsObjectBindingPatternShorthandProperty(p) => p
                    .identifier()
                    .map_or(false, |it| with_binding_identifier(it, f)),
                P::JsUnknownBinding(_) => false,
            }
        })
}

fn with_array_binding_pat_identifiers(
    pat: JsArrayBindingPattern,
    f: &mut impl FnMut(JsIdentifierBinding) -> bool,
) -> bool {
    pat.elements().into_iter().filter_map(Result::ok).any(|it| {
        use JsAnyArrayBindingPatternElement as P;
        match it {
            P::JsAnyBindingPattern(p) => with_binding_pat_identifiers(p, f),
            P::JsArrayBindingPatternRestElement(p) => p
                .pattern()
                .map_or(false, |it| with_binding_pat_identifiers(it, f)),
            P::JsArrayHole(_) => false,
            P::JsBindingPatternWithDefault(p) => p
                .pattern()
                .map_or(false, |it| with_binding_pat_identifiers(it, f)),
        }
    })
}

fn with_binding_identifier(
    binding: JsAnyBinding,
    f: &mut impl FnMut(JsIdentifierBinding) -> bool,
) -> bool {
    match binding {
        JsAnyBinding::JsIdentifierBinding(id) => f(id),
        JsAnyBinding::JsUnknownBinding(_) => false,
    }
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
                    // example: while(a = b) {}
                    None
                }
            }
        }
    }

    fn has_member_expr_assignment(&self) -> bool {
        match self {
            Self::JsAssignmentExpression(it) => {
                it.left().map_or(false, has_member_expr_in_assign_pat)
            }
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
                .map_or(false, |pat| has_outer_variables_in_assign_pat(pat, &scope)),
        }
    }
}

fn has_outer_variables_in_binding_pat(pat: JsAnyBindingPattern, scope: Scope) -> bool {
    with_binding_pat_identifiers(pat, &mut |it| is_outer_variable_in_binding(it, &scope))
}

fn is_outer_variable_in_binding(binding: JsIdentifierBinding, scope: &Scope) -> bool {
    binding
        .name_token()
        .map_or(false, |name| is_binding_in_outer_scopes(scope, name))
}

fn has_member_expr_in_assign_pat(pat: JsAnyAssignmentPattern) -> bool {
    use JsAnyAssignmentPattern as P;
    match pat {
        P::JsAnyAssignment(p) => is_member_expr_assignment(p),
        P::JsArrayAssignmentPattern(p) => has_member_expr_in_array_pat(p),
        P::JsObjectAssignmentPattern(p) => has_member_expr_in_object_assign_pat(p),
    }
}

fn has_member_expr_in_object_assign_pat(pat: JsObjectAssignmentPattern) -> bool {
    pat.properties()
        .into_iter()
        .filter_map(Result::ok)
        .any(|it| {
            use JsAnyObjectAssignmentPatternMember as P;
            match it {
                P::JsObjectAssignmentPatternProperty(p) => {
                    p.pattern().map_or(false, has_member_expr_in_assign_pat)
                }
                P::JsObjectAssignmentPatternRest(p) => {
                    p.target().map_or(false, is_member_expr_assignment)
                }
                P::JsObjectAssignmentPatternShorthandProperty(_) | P::JsUnknownAssignment(_) => {
                    false
                }
            }
        })
}

fn has_member_expr_in_array_pat(pat: JsArrayAssignmentPattern) -> bool {
    pat.elements()
        .into_iter()
        .filter_map(Result::ok)
        .any(|it| it.pattern().map_or(false, has_member_expr_in_assign_pat))
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

fn has_outer_variables_in_assign_pat(pat: JsAnyAssignmentPattern, scope: &Scope) -> bool {
    use JsAnyAssignmentPattern as P;
    match pat {
        P::JsAnyAssignment(p) => is_outer_variable_in_assignment(p, scope),
        P::JsArrayAssignmentPattern(p) => has_outer_variables_in_object_assign_pat(p, scope),
        P::JsObjectAssignmentPattern(p) => has_outer_variables_in_array_assign_pat(p, scope),
    }
}

fn has_outer_variables_in_array_assign_pat(pat: JsObjectAssignmentPattern, scope: &Scope) -> bool {
    pat.properties()
        .into_iter()
        .filter_map(Result::ok)
        .any(|it| {
            use JsAnyObjectAssignmentPatternMember as P;
            match it {
                P::JsObjectAssignmentPatternProperty(p) => p
                    .pattern()
                    .map_or(false, |it| has_outer_variables_in_assign_pat(it, scope)),
                P::JsObjectAssignmentPatternRest(p) => p
                    .target()
                    .map_or(false, |it| is_outer_variable_in_assignment(it, scope)),
                P::JsObjectAssignmentPatternShorthandProperty(p) => p
                    .identifier()
                    .map_or(false, |it| is_outer_ident_in_assignment(it, scope)),
                P::JsUnknownAssignment(_) => false,
            }
        })
}

fn has_outer_variables_in_object_assign_pat(pat: JsArrayAssignmentPattern, scope: &Scope) -> bool {
    pat.elements().into_iter().filter_map(Result::ok).any(|it| {
        it.pattern()
            .map_or(false, |p| has_outer_variables_in_assign_pat(p, scope))
    })
}

fn is_outer_variable_in_assignment(e: JsAnyAssignment, scope: &Scope) -> bool {
    match e {
        JsAnyAssignment::JsIdentifierAssignment(it) => is_outer_ident_in_assignment(it, scope),
        _ => false,
    }
}

fn is_outer_ident_in_assignment(assignment: JsIdentifierAssignment, scope: &Scope) -> bool {
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
