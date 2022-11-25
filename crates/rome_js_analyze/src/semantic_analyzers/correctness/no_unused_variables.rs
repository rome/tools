use crate::JsRuleAction;
use crate::{semantic_services::Semantic, utils::rename::RenameSymbolExtensions};
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_semantic::{ReferencesExtensions, SemanticScopeExtensions};
use rome_js_syntax::{
    binding_ext::{JsAnyBindingDeclaration, JsAnyIdentifierBinding, JsAnyParameterParentFunction},
    JsClassExpression, JsFunctionDeclaration, JsFunctionExpression, JsSyntaxKind, JsSyntaxNode,
    JsVariableDeclarator, TsPropertyParameter,
};
use rome_rowan::{AstNode, BatchMutationExt};

declare_rule! {
    /// Disallow unused variables.
    ///
    /// There are two exceptions to this rule:
    /// 1. variables that starts with underscore, ex: `let _something;`
    /// 2. the `React` variable;
    ///
    /// The pattern of having an underscore as prefix of a name of variable is a very diffuse
    /// pattern among programmers, and Rome decided to follow it.
    ///
    /// Importing the `React` variable was a mandatory pattern until some time ago:
    ///
    /// For the time being this rule will ignore it, but this **might change in the future releases**.
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
    /// function foo() {
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
    /// const foo = () => {
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
    /// function foo(b) {
    ///     console.log(b)
    /// };
    /// foo();
    /// ```
    ///
    /// ```js
    /// function foo(_unused) {
    /// };
    /// foo();
    /// ```
    ///
    /// ```jsx
    /// import React from 'react';
    /// function foo() {
    ///     return <div />;
    /// };
    /// foo();
    /// ```
    ///
    /// ```ts
    /// function used_overloaded(): number;
    /// function used_overloaded(s: string): string;
    /// function used_overloaded(s?: string) {
    ///     return s;
    /// }
    /// used_overloaded();
    /// ```
    pub(crate) NoUnusedVariables {
        version: "0.9.0",
        name: "noUnusedVariables",
        recommended: false,
    }
}

/// Suggestion if the bindnig is unused
#[derive(Copy, Clone)]
pub enum SuggestedFix {
    /// No suggestion will be given
    NoSuggestion,
    /// Suggest to prefix the name of the binding with underscore
    PrefixUnderscore,
}

fn is_ok_to_be_unused(parent_function: Option<JsAnyParameterParentFunction>) -> bool {
    matches!(
        parent_function,
        Some(
            // bindings in signatures are ok to not be  used
            JsAnyParameterParentFunction::TsMethodSignatureClassMember(_)
            | JsAnyParameterParentFunction::TsCallSignatureTypeMember(_)
            | JsAnyParameterParentFunction::TsConstructSignatureTypeMember(_)
            | JsAnyParameterParentFunction::TsConstructorSignatureClassMember(_)
            | JsAnyParameterParentFunction::TsMethodSignatureTypeMember(_)
            | JsAnyParameterParentFunction::TsSetterSignatureClassMember(_)
            | JsAnyParameterParentFunction::TsSetterSignatureTypeMember(_)
            // bindings in function types are ok to not be used
            | JsAnyParameterParentFunction::TsFunctionType(_)
            // binding in declare are ok to not be used
            | JsAnyParameterParentFunction::TsDeclareFunctionDeclaration(_)
        )
    )
}

fn is_public_or_private(parameter: TsPropertyParameter) -> Option<bool> {
    for modifier in parameter.modifiers().into_iter() {
        if let Some(modifier) = modifier.as_ts_accessibility_modifier() {
            match modifier.modifier_token().ok()?.kind() {
                // with modifiers are ok to not be used
                JsSyntaxKind::PRIVATE_KW | JsSyntaxKind::PUBLIC_KW => return Some(true),
                // no midifiers, need to be check further
                _ => {}
            }
        }
    }

    Some(false)
}

fn is_under_declare(node: &JsSyntaxNode) -> bool {
    node.ancestors()
        .any(|x| x.kind() == JsSyntaxKind::TS_DECLARE_STATEMENT)
}

fn suggestion_for_binding(binding: &JsAnyIdentifierBinding) -> Option<SuggestedFix> {
    if binding.is_under_object_pattern_binding()? {
        Some(SuggestedFix::NoSuggestion)
    } else {
        Some(SuggestedFix::PrefixUnderscore)
    }
}

// It is ok in some Typescripts constructs for a parameter to be unused.
// Returning None means is ok to be unused
fn suggested_fix_if_unused(binding: &JsAnyIdentifierBinding) -> Option<SuggestedFix> {
    match binding.declaration()? {
        // ok to not be used
        JsAnyBindingDeclaration::TsIndexSignatureParameter(_)
        | JsAnyBindingDeclaration::TsDeclareFunctionDeclaration(_)
        | JsAnyBindingDeclaration::TsTypeAliasDeclaration(_)
        | JsAnyBindingDeclaration::JsClassExpression(_)
        | JsAnyBindingDeclaration::JsFunctionExpression(_) => None,

        // Some parameters are ok to not be used
        JsAnyBindingDeclaration::TsPropertyParameter(parameter) => {
            let is_binding_ok =
                is_ok_to_be_unused(parameter.parent_function()) || is_public_or_private(parameter)?;
            if !is_binding_ok {
                suggestion_for_binding(binding)
            } else {
                None
            }
        }
        JsAnyBindingDeclaration::JsFormalParameter(parameter) => {
            let is_binding_ok = is_ok_to_be_unused(parameter.parent_function());
            if !is_binding_ok {
                suggestion_for_binding(binding)
            } else {
                None
            }
        }
        JsAnyBindingDeclaration::JsRestParameter(parameter) => {
            let is_binding_ok = is_ok_to_be_unused(parameter.parent_function());
            if !is_binding_ok {
                suggestion_for_binding(binding)
            } else {
                None
            }
        }

        // declarations need to be check if they are under `declare`
        node @ JsAnyBindingDeclaration::JsVariableDeclarator(_) => {
            let is_binding_ok = is_under_declare(&node.syntax().clone());
            if !is_binding_ok {
                suggestion_for_binding(binding)
            } else {
                None
            }
        }
        node @ JsAnyBindingDeclaration::JsClassDeclaration(_)
        | node @ JsAnyBindingDeclaration::JsFunctionDeclaration(_)
        | node @ JsAnyBindingDeclaration::TsInterfaceDeclaration(_)
        | node @ JsAnyBindingDeclaration::TsEnumDeclaration(_) => {
            if is_under_declare(&node.syntax().clone()) {
                None
            } else {
                Some(SuggestedFix::PrefixUnderscore)
            }
        }
        node @ JsAnyBindingDeclaration::TsModuleDeclaration(_)
        | node @ JsAnyBindingDeclaration::TsImportEqualsDeclaration(_) => {
            if is_under_declare(&node.syntax().clone()) {
                None
            } else {
                Some(SuggestedFix::NoSuggestion)
            }
        }

        // Bindings under unknown parameter are never ok to be unused
        JsAnyBindingDeclaration::JsUnknownParameter(_) => Some(SuggestedFix::NoSuggestion),

        // Bindings under catch are never ok to be unused
        JsAnyBindingDeclaration::JsCatchDeclaration(_) => Some(SuggestedFix::PrefixUnderscore),

        // Imports are never ok to be unused
        JsAnyBindingDeclaration::JsImportDefaultClause(_)
        | JsAnyBindingDeclaration::JsImportNamespaceClause(_)
        | JsAnyBindingDeclaration::JsShorthandNamedImportSpecifier(_)
        | JsAnyBindingDeclaration::JsNamedImportSpecifier(_)
        | JsAnyBindingDeclaration::JsUnknownNamedImportSpecifier(_)
        | JsAnyBindingDeclaration::JsDefaultImportSpecifier(_)
        | JsAnyBindingDeclaration::JsNamespaceImportSpecifier(_) => {
            Some(SuggestedFix::NoSuggestion)
        }

        // exports with binding are ok to be unused
        JsAnyBindingDeclaration::JsClassExportDefaultDeclaration(_)
        | JsAnyBindingDeclaration::JsFunctionExportDefaultDeclaration(_)
        | JsAnyBindingDeclaration::TsDeclareFunctionExportDefaultDeclaration(_) => {
            Some(SuggestedFix::NoSuggestion)
        }
    }
}

impl Rule for NoUnusedVariables {
    type Query = Semantic<JsAnyIdentifierBinding>;
    type State = SuggestedFix;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let binding = ctx.query();

        let name = match binding {
            JsAnyIdentifierBinding::JsIdentifierBinding(binding) => binding.name_token().ok()?,
            JsAnyIdentifierBinding::TsIdentifierBinding(binding) => binding.name_token().ok()?,
        };

        let name = name.token_text_trimmed();
        let name = name.text();

        // Old code import React but do not used directly
        // only indirectly after transpiling JSX.
        if name.starts_with('_') || name == "React" {
            return None;
        }

        // Ignore expressions
        if binding.parent::<JsFunctionExpression>().is_some()
            || binding.parent::<JsClassExpression>().is_some()
        {
            return None;
        }

        let Some(suggestion) = suggested_fix_if_unused(binding) else {
            return None;
        };

        let model = ctx.model();
        if model.is_exported(binding) {
            return None;
        }

        let all_references = binding.all_references(model);
        if all_references.count() == 0 {
            Some(suggestion)
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
                    if r.syntax().ancestors().any(|n| n == *node) {
                        continue;
                    }
                }

                references_outside += 1;
                break;
            }

            if references_outside == 0 {
                Some(suggestion)
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
            JsSyntaxKind::JS_CLASS_DECLARATION => "class",
            JsSyntaxKind::TS_INTERFACE_DECLARATION => "interface",
            JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION => "type alias",
            _ => "variable",
        };

        let diag = RuleDiagnostic::new(
            rule_category!(),
            binding.syntax().text_trimmed_range(),
            markup! {
                "This " {symbol_type} " is unused."
            },
        );

        let diag = diag.note(
            markup! {"Unused variables usually are result of incomplete refactoring, typos and other source of bugs."},
        );

        Some(diag)
    }

    fn action(ctx: &RuleContext<Self>, suggestion: &Self::State) -> Option<JsRuleAction> {
        match suggestion {
            SuggestedFix::NoSuggestion => None,
            SuggestedFix::PrefixUnderscore => {
                let binding = ctx.query();
                let mut mutation = ctx.root().begin();

                let name = match binding {
                    JsAnyIdentifierBinding::JsIdentifierBinding(binding) => {
                        binding.name_token().ok()?
                    }
                    JsAnyIdentifierBinding::TsIdentifierBinding(binding) => {
                        binding.name_token().ok()?
                    }
                };
                let name_trimmed = name.text_trimmed();
                let new_name = format!("_{}", name_trimmed);

                let model = ctx.model();
                mutation.rename_node_declaration(model, binding.clone(), &new_name);

                Some(JsRuleAction {
                    mutation,
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::MaybeIncorrect,
                    message: markup! { "If this is intentional, prepend "<Emphasis>{name_trimmed}</Emphasis>" with an underscore." }
                        .to_owned(),
                })
            }
        }
    }
}
