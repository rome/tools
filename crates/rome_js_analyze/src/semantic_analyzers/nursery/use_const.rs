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

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diag: Option<RuleDiagnostic> = None;

        for info in state.iter().flat_map(Option::as_ref) {
            let title = format! {
                "'{}' is never reassigned. Use 'const' instead.", info.binding.syntax().text_trimmed()
            };
            let range = info.binding.range();
            match diag.take() {
                Some(d) => diag = Some(d.detail(range, title)),
                None => diag = Some(RuleDiagnostic::new(rule_category!(), range, title)),
            }
        }

        diag
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
                message: markup! { "Use const instead" }.to_owned(),
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
    fn declarators(&self) -> Vec<JsVariableDeclarator> {
        match self {
            VarDecl::JsVariableDeclaration(declaration) => declaration
                .declarators()
                .into_iter()
                .filter_map(Result::ok)
                .collect(),
            VarDecl::JsForVariableDeclaration(f) => f.declarator().into_iter().collect(),
        }
    }

    fn kind_token(&self) -> Option<JsSyntaxToken> {
        match self {
            VarDecl::JsVariableDeclaration(x) => x.kind().ok(),
            VarDecl::JsForVariableDeclaration(x) => x.kind_token().ok(),
        }
    }
}

fn declarator_bindings(decl: &JsVariableDeclarator) -> Vec<JsIdentifierBinding> {
    let mut bindings = Vec::new();
    if let Ok(pat) = decl.id() {
        get_bindings_in_pat(pat, &mut bindings)
    }
    bindings
}

fn get_bindings_in_pat(pat: JsAnyBindingPattern, out: &mut Vec<JsIdentifierBinding>) {
    use JsAnyBindingPattern as B;
    match pat {
        B::JsAnyBinding(x) => {
            if let JsAnyBinding::JsIdentifierBinding(x) = x {
                out.push(x)
            }
        }
        B::JsArrayBindingPattern(x) => {
            for e in x.elements().into_iter().filter_map(Result::ok) {
                get_bindings_in_array_pat(e, out);
            }
        }
        B::JsObjectBindingPattern(x) => {
            for e in x.properties().into_iter().filter_map(Result::ok) {
                get_bindings_in_object_pat(e, out);
            }
        }
    }
}

fn get_bindings_in_object_pat(
    pat: JsAnyObjectBindingPatternMember,
    out: &mut Vec<JsIdentifierBinding>,
) {
    use JsAnyObjectBindingPatternMember as B;
    match pat {
        B::JsObjectBindingPatternProperty(x) => {
            if let Ok(x) = x.pattern() {
                get_bindings_in_pat(x, out);
            }
        }
        B::JsObjectBindingPatternRest(x) => {
            if let Ok(JsAnyBinding::JsIdentifierBinding(x)) = x.binding() {
                out.push(x)
            }
        }
        B::JsObjectBindingPatternShorthandProperty(x) => {
            if let Ok(JsAnyBinding::JsIdentifierBinding(x)) = x.identifier() {
                out.push(x)
            }
        }
        B::JsUnknownBinding(_) => (),
    }
}

fn get_bindings_in_array_pat(
    pat: JsAnyArrayBindingPatternElement,
    out: &mut Vec<JsIdentifierBinding>,
) {
    use JsAnyArrayBindingPatternElement as B;
    match pat {
        B::JsAnyBindingPattern(x) => get_bindings_in_pat(x, out),
        B::JsArrayBindingPatternRestElement(x) => {
            if let Ok(x) = x.pattern() {
                get_bindings_in_pat(x, out);
            }
        }
        B::JsArrayHole(_) => (),
        B::JsBindingPatternWithDefault(x) => {
            if let Ok(x) = x.pattern() {
                get_bindings_in_pat(x, out);
            }
        }
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
                    None
                }
            }
        }
    }

    fn has_member_expr_assignment(&self) -> bool {
        let Self::JsAssignmentExpression(it) = self else { return false };
        match it.left() {
            Ok(
                pat @ JsAnyAssignmentPattern::JsArrayAssignmentPattern(_)
                | pat @ JsAnyAssignmentPattern::JsObjectAssignmentPattern(_),
            ) => has_member_expr_assignment(pat),
            _ => false,
        }
    }

    fn has_outer_variables(&self, scope: Scope) -> bool {
        match self {
            Self::JsVariableDeclarator(it) => has_outer_variables_in_var_declarator(it, scope),
            Self::JsAssignmentExpression(it) => has_outer_variables_in_assignment_expr(it, scope),
        }
    }
}

fn has_outer_variables_in_var_declarator(declarator: &JsVariableDeclarator, scope: Scope) -> bool {
    declarator
        .id()
        .map_or(false, |it| has_outer_variables_in_binding_pat(it, &scope))
}

fn has_outer_variables_in_binding_pat(pat: JsAnyBindingPattern, scope: &Scope) -> bool {
    use JsAnyBindingPattern::*;
    match pat {
        JsArrayBindingPattern(it) => it
            .elements()
            .into_iter()
            .filter_map(Result::ok)
            .any(|element| has_outer_variable_in_array_binding_pat(element, scope)),
        JsObjectBindingPattern(it) => it
            .properties()
            .into_iter()
            .filter_map(Result::ok)
            .any(|property| has_outer_variables_in_object_binding_pat(property, scope)),
        JsAnyBinding(it) => is_outer_binding_in_destructuring(it, scope),
    }
}

fn has_outer_variables_in_object_binding_pat(
    property: JsAnyObjectBindingPatternMember,
    scope: &Scope,
) -> bool {
    use JsAnyObjectBindingPatternMember::*;
    match property {
        JsObjectBindingPatternProperty(it) => it
            .pattern()
            .map_or(false, |it| has_outer_variables_in_binding_pat(it, scope)),
        JsObjectBindingPatternRest(it) => it
            .binding()
            .map_or(false, |it| is_outer_binding_in_destructuring(it, scope)),
        JsObjectBindingPatternShorthandProperty(it) => it
            .identifier()
            .map_or(false, |it| is_outer_binding_in_destructuring(it, scope)),
        JsUnknownBinding(_) => false,
    }
}

fn has_outer_variable_in_array_binding_pat(
    element: JsAnyArrayBindingPatternElement,
    scope: &Scope,
) -> bool {
    use JsAnyArrayBindingPatternElement::*;
    match element {
        JsAnyBindingPattern(it) => has_outer_variables_in_binding_pat(it, scope),
        JsArrayBindingPatternRestElement(it) => it
            .pattern()
            .map_or(false, |it| has_outer_variables_in_binding_pat(it, scope)),
        JsBindingPatternWithDefault(it) => it
            .pattern()
            .map_or(false, |it| has_outer_variables_in_binding_pat(it, scope)),
        JsArrayHole(_) => false,
    }
}

fn has_outer_variables_in_assignment_expr(
    assignment: &JsAssignmentExpression,
    scope: Scope,
) -> bool {
    assignment.left().map_or(false, |it| {
        matches!(
            it,
            JsAnyAssignmentPattern::JsObjectAssignmentPattern(..)
                | JsAnyAssignmentPattern::JsArrayAssignmentPattern(..)
        ) && has_outer_variables_in_assignment_pat(it, &scope)
    })
}

fn has_outer_variables_in_assignment_pat(pat: JsAnyAssignmentPattern, scope: &Scope) -> bool {
    match pat {
        JsAnyAssignmentPattern::JsObjectAssignmentPattern(obj) => obj
            .properties()
            .into_iter()
            .flat_map(Result::ok)
            .any(|it| has_outer_variables_in_obj_assign_pat(it, scope)),
        JsAnyAssignmentPattern::JsArrayAssignmentPattern(arr) => arr
            .elements()
            .into_iter()
            .flat_map(Result::ok)
            .any(|it| has_outer_variables_in_array_assign_pat(it, scope)),
        JsAnyAssignmentPattern::JsAnyAssignment(p) => is_outer_variable_in_assignment(p, scope),
    }
}

fn has_outer_variables_in_array_assign_pat(
    it: JsAnyArrayAssignmentPatternElement,
    scope: &Scope,
) -> bool {
    use JsAnyArrayAssignmentPatternElement::*;
    match it {
        JsAnyAssignmentPattern(it) => has_outer_variables_in_assignment_pat(it, scope),
        JsArrayAssignmentPatternRestElement(it) => it
            .pattern()
            .map_or(false, |it| has_outer_variables_in_assignment_pat(it, scope)),
        JsAssignmentWithDefault(it) => it
            .pattern()
            .map_or(false, |it| has_outer_variables_in_assignment_pat(it, scope)),
        _ => false,
    }
}

fn has_outer_variables_in_obj_assign_pat(
    it: JsAnyObjectAssignmentPatternMember,
    scope: &Scope,
) -> bool {
    use JsAnyObjectAssignmentPatternMember::*;
    match it {
        JsObjectAssignmentPatternProperty(it) => it
            .pattern()
            .map_or(false, |it| has_outer_variables_in_assignment_pat(it, scope)),
        JsObjectAssignmentPatternRest(it) => it
            .target()
            .map_or(false, |it| is_outer_variable_in_assignment(it, scope)),
        JsObjectAssignmentPatternShorthandProperty(it) => it
            .identifier()
            .and_then(|it| it.name_token())
            .map_or(false, |name| is_binding_in_outer_scopes(scope, name)),
        _ => false,
    }
}

impl VarDecl {
    fn is_let(&self) -> bool {
        match self {
            VarDecl::JsVariableDeclaration(it) => it.is_let(),
            VarDecl::JsForVariableDeclaration(it) => it
                .kind_token()
                .map_or(false, |it| it.kind() == JsSyntaxKind::LET_KW),
        }
    }
}

fn has_member_expr_assignment(pat: JsAnyAssignmentPattern) -> bool {
    match pat {
        JsAnyAssignmentPattern::JsObjectAssignmentPattern(obj) => obj
            .properties()
            .into_iter()
            .flat_map(Result::ok)
            .any(|it| has_member_expr_in_obj_assign_pat(it)),
        JsAnyAssignmentPattern::JsArrayAssignmentPattern(arr) => arr
            .elements()
            .into_iter()
            .flat_map(Result::ok)
            .any(|it| has_member_expr_in_array_assign_pat(it)),
        JsAnyAssignmentPattern::JsAnyAssignment(p) => is_member_expr_assignment(p),
    }
}

fn has_member_expr_in_array_assign_pat(it: JsAnyArrayAssignmentPatternElement) -> bool {
    use JsAnyArrayAssignmentPatternElement::*;
    match it {
        JsAnyAssignmentPattern(it) => has_member_expr_assignment(it),
        JsArrayAssignmentPatternRestElement(it) => {
            it.pattern().map_or(false, has_member_expr_assignment)
        }
        JsAssignmentWithDefault(it) => it.pattern().map_or(false, has_member_expr_assignment),
        _ => false,
    }
}

fn has_member_expr_in_obj_assign_pat(it: JsAnyObjectAssignmentPatternMember) -> bool {
    use JsAnyObjectAssignmentPatternMember::*;
    match it {
        JsObjectAssignmentPatternProperty(it) => {
            it.pattern().map_or(false, has_member_expr_assignment)
        }
        JsObjectAssignmentPatternRest(it) => it.target().map_or(false, is_member_expr_assignment),
        _ => false,
    }
}

fn is_member_expr_assignment(e: JsAnyAssignment) -> bool {
    use JsAnyAssignment::*;
    match e {
        JsComputedMemberAssignment(_) | JsStaticMemberAssignment(_) => true,
        JsParenthesizedAssignment(it) => it.assignment().map_or(false, is_member_expr_assignment),
        _ => false,
    }
}

fn is_outer_binding_in_destructuring(binding: JsAnyBinding, scope: &Scope) -> bool {
    binding
        .as_js_identifier_binding()
        .and_then(|it| it.name_token().ok())
        .map_or(false, |name| is_binding_in_outer_scopes(scope, name))
}

fn is_outer_variable_in_assignment(e: JsAnyAssignment, scope: &Scope) -> bool {
    match e {
        JsAnyAssignment::JsIdentifierAssignment(it) => it
            .name_token()
            .map_or(false, |name| is_binding_in_outer_scopes(scope, name)),
        _ => false,
    }
}

fn is_binding_in_outer_scopes(scope: &Scope, name: JsSyntaxToken) -> bool {
    let text = name.text_trimmed();
    scope
        .ancestors()
        .skip(1) // Skip current scope
        .any(|scope| scope.get_binding(text).is_some())
}
