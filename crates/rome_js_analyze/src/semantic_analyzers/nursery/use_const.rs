use crate::semantic_services::Semantic;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;

use rome_js_semantic::{AllReferencesExtensions, Scope, SemanticModel, SemanticScopeExtensions};
use rome_js_syntax::*;
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

declare_node_union! {
    pub(crate) DestructuringHost = JsVariableDeclarator | JsAssignmentExpression
}

impl Rule for UseConst {
    type Query = Semantic<JsIdentifierBinding>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let binding = ctx.query();
        let model = ctx.model();

        let should_be_const = should_binding_be_const(binding, model).unwrap_or(false);
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
}

fn should_binding_be_const(binding: &JsIdentifierBinding, model: &SemanticModel) -> Option<bool> {
    let declarator = binding
        .syntax()
        .ancestors()
        .find_map(JsVariableDeclarator::cast)?;

    let decl = declarator.syntax().ancestors().find_map(VarDecl::cast)?;

    // Not a let or inside a for-loop init
    if !decl.is_let() || decl.parent::<JsForStatement>().is_some() {
        return None;
    }

    let mut writes = binding.all_writes(model);

    // In a for-in or for-of loop
    if matches!(decl, VarDecl::JsForVariableDeclaration(..)) {
        return Some(writes.len() == 0);
    }

    // If it has initializer.
    if declarator.initializer().is_some() {
        return Some(writes.len() == 0);
    }

    // If no initializer and one assignment in same scope
    let write = match (writes.next(), writes.next()) {
        (Some(v), None) => v,
        _ => return None,
    };

    let host = write.node().ancestors().find_map(DestructuringHost::cast)?;
    if host.has_member_expr_assignment() || host.has_outer_variables(write.scope()) {
        return Some(false);
    }

    Some(
        write.scope() == binding.scope(model)
            && host.can_become_variable_declaration().unwrap_or(false),
    )
}

impl DestructuringHost {
    fn can_become_variable_declaration(&self) -> Option<bool> {
        match self {
            DestructuringHost::JsVariableDeclarator(_) => Some(true),
            DestructuringHost::JsAssignmentExpression(e) => {
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
        let DestructuringHost::JsAssignmentExpression(it) = self else { return false };
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
            DestructuringHost::JsVariableDeclarator(it) => {
                has_outer_variables_in_var_declarator(it, scope)
            }
            DestructuringHost::JsAssignmentExpression(it) => {
                has_outer_variables_in_assignment_expr(it, scope)
            }
        }
    }
}

fn has_outer_variables_in_var_declarator(declarator: &JsVariableDeclarator, scope: Scope) -> bool {
    declarator
        .id()
        .map_or(false, |it| has_outer_variables_in_binding_pat(it, &scope))
}

fn has_outer_variables_in_binding_pat(pat: JsAnyBindingPattern, scope: &Scope) -> bool {
    match pat {
        JsAnyBindingPattern::JsArrayBindingPattern(it) => {
            for element in it.elements().into_iter().filter_map(Result::ok) {
                if has_outer_variable_in_array_binding_pat(element, scope) {
                    return true;
                }
            }
        }
        JsAnyBindingPattern::JsObjectBindingPattern(it) => {
            for property in it.properties().into_iter().filter_map(Result::ok) {
                if has_outer_variables_in_object_binding_pat(property, scope) {
                    return true;
                }
            }
        }
        JsAnyBindingPattern::JsAnyBinding(it) => {
            return is_outer_binding_in_destructuring(it, scope)
        }
    }

    false
}

fn has_outer_variables_in_object_binding_pat(
    property: JsAnyObjectBindingPatternMember,
    scope: &Scope,
) -> bool {
    match property {
        JsAnyObjectBindingPatternMember::JsObjectBindingPatternProperty(it) => {
            if it
                .pattern()
                .map_or(false, |it| has_outer_variables_in_binding_pat(it, scope))
            {
                return true;
            }
        }
        JsAnyObjectBindingPatternMember::JsObjectBindingPatternRest(it) => {
            if it
                .binding()
                .map_or(false, |it| is_outer_binding_in_destructuring(it, scope))
            {
                return true;
            }
        }
        JsAnyObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(it) => {
            if it
                .identifier()
                .map_or(false, |it| is_outer_binding_in_destructuring(it, scope))
            {
                return true;
            }
        }
        JsAnyObjectBindingPatternMember::JsUnknownBinding(_) => (),
    }
    false
}

fn has_outer_variable_in_array_binding_pat(
    element: JsAnyArrayBindingPatternElement,
    scope: &Scope,
) -> bool {
    match element {
        JsAnyArrayBindingPatternElement::JsAnyBindingPattern(it) => {
            if has_outer_variables_in_binding_pat(it, scope) {
                return true;
            }
        }
        JsAnyArrayBindingPatternElement::JsArrayBindingPatternRestElement(it) => {
            if it
                .pattern()
                .map_or(false, |it| has_outer_variables_in_binding_pat(it, scope))
            {
                return true;
            }
        }
        JsAnyArrayBindingPatternElement::JsArrayHole(_) => (),
        JsAnyArrayBindingPatternElement::JsBindingPatternWithDefault(it) => {
            if it
                .pattern()
                .map_or(false, |it| has_outer_variables_in_binding_pat(it, scope))
            {
                return true;
            }
        }
    }
    false
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
        JsAnyAssignmentPattern::JsObjectAssignmentPattern(obj) => {
            for member in obj.properties().into_iter().flat_map(Result::ok) {
                match member {
                    JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(it) => {
                        if it
                            .pattern()
                            .map_or(false, |it| has_outer_variables_in_assignment_pat(it, scope))
                        {
                            return true;
                        }
                    }
                    JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(it) => {
                        if it
                            .target()
                            .map_or(false, |it| is_outer_variable_in_assignment(it, scope))
                        {
                            return true;
                        }
                    }
                    JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(it) => {
                        if it
                            .identifier()
                            .map_or(false, |it| it.name_token().map_or(false, |name| is_binding_in_outer_scopes(scope, name)))
                        {
                            return true;
                        }
                    }
                    _ => (),
                }
            }
            false
        }
        JsAnyAssignmentPattern::JsArrayAssignmentPattern(arr) => {
            for member in arr.elements().into_iter().flat_map(Result::ok) {
                match member {
                    JsAnyArrayAssignmentPatternElement::JsAnyAssignmentPattern(it) => {
                        if has_outer_variables_in_assignment_pat(it, scope) {
                            return true;
                        }
                    }
                    JsAnyArrayAssignmentPatternElement::JsArrayAssignmentPatternRestElement(it) => {
                        if it
                            .pattern()
                            .map_or(false, |it| has_outer_variables_in_assignment_pat(it, scope))
                        {
                            return true;
                        }
                    }
                    JsAnyArrayAssignmentPatternElement::JsAssignmentWithDefault(it) => {
                        if it
                            .pattern()
                            .map_or(false, |it| has_outer_variables_in_assignment_pat(it, scope))
                        {
                            return true;
                        }
                    }
                    _ => (),
                }
            }
            false
        }
        JsAnyAssignmentPattern::JsAnyAssignment(p) => is_outer_variable_in_assignment(p, scope),
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
        JsAnyAssignmentPattern::JsObjectAssignmentPattern(obj) => {
            for member in obj.properties().into_iter().flat_map(Result::ok) {
                match member {
                    JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(it) => {
                        if it.pattern().map_or(false, has_member_expr_assignment) {
                            return true;
                        }
                    }
                    JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(it) => {
                        if it.target().map_or(false, is_member_expr_assignment) {
                            return true;
                        }
                    }
                    _ => (),
                }
            }
            false
        }
        JsAnyAssignmentPattern::JsArrayAssignmentPattern(arr) => {
            for member in arr.elements().into_iter().flat_map(Result::ok) {
                match member {
                    JsAnyArrayAssignmentPatternElement::JsAnyAssignmentPattern(it) => {
                        if has_member_expr_assignment(it) {
                            return true;
                        }
                    }
                    JsAnyArrayAssignmentPatternElement::JsArrayAssignmentPatternRestElement(it) => {
                        if it.pattern().map_or(false, has_member_expr_assignment) {
                            return true;
                        }
                    }
                    JsAnyArrayAssignmentPatternElement::JsAssignmentWithDefault(it) => {
                        if it.pattern().map_or(false, has_member_expr_assignment) {
                            return true;
                        }
                    }
                    _ => (),
                }
            }
            false
        }
        JsAnyAssignmentPattern::JsAnyAssignment(p) => is_member_expr_assignment(p),
    }
}

fn is_member_expr_assignment(e: JsAnyAssignment) -> bool {
    match e {
        JsAnyAssignment::JsComputedMemberAssignment(_)
        | JsAnyAssignment::JsStaticMemberAssignment(_) => true,
        JsAnyAssignment::JsParenthesizedAssignment(it) => {
            it.assignment().map_or(false, is_member_expr_assignment)
        }
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
    for scope in scope.ancestors().skip(1) {
        if scope.get_binding(text).is_some() {
            return true;
        }
    }
    false
}
