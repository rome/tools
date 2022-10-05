use crate::{
    semantic_services::Semantic,
    utils::{rename::RenameSymbolExtensions, ToCamelCase},
    JsRuleAction,
};
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_semantic::{AllReferencesExtensions, IsExportedCanBeQueried, SemanticModel};
use rome_js_syntax::{
    JsFormalParameter, JsFunctionDeclaration, JsFunctionExportDefaultDeclaration,
    JsGetterClassMember, JsIdentifierBinding, JsLiteralMemberName, JsMethodClassMember,
    JsPrivateClassMemberName, JsPropertyClassMember, JsSetterClassMember, JsSyntaxKind,
    JsVariableDeclaration, JsVariableDeclarator, JsVariableDeclaratorList, JsxReferenceIdentifier,
};
use rome_rowan::{declare_node_union, AstNode, BatchMutationExt};
use std::{borrow::Cow, iter::once};

declare_rule! {
    /// Enforce camel case naming convention.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let snake_case;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let PascalCase;
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// let camelCase;
    /// ```
    pub(crate) UseCamelCase {
        version: "0.8.0",
        name: "useCamelCase",
        recommended: false,
    }
}

pub struct State {
    new_name: String,
}

fn check_is_camel(name: &str) -> Option<State> {
    if name.starts_with('_') {
        return None;
    }

    match name.to_camel_case() {
        Cow::Borrowed(_) => None,
        Cow::Owned(new_name) => Some(State { new_name }),
    }
}

// It is OK to be non camel case when:
// 1. it's a const variable (eg: const THIS_IS_OK);
// 2. it's a function used in a new expression (eg: new PascalCase());
// 3. it's a exported function.
fn is_non_camel_ok(binding: &JsIdentifierBinding, model: &SemanticModel) -> Option<bool> {
    use JsSyntaxKind::*;
    match binding.syntax().parent()?.kind() {
        JS_VARIABLE_DECLARATOR => {
            let declarator = binding.parent::<JsVariableDeclarator>()?;
            let is_ok = match declarator.syntax().parent().map(|parent| parent.kind()) {
                Some(JS_VARIABLE_DECLARATOR_LIST) => declarator
                    .parent::<JsVariableDeclaratorList>()?
                    .parent::<JsVariableDeclaration>()?
                    .is_const(),
                _ => false,
            };
            Some(is_ok)
        }
        JS_FUNCTION_DECLARATION => {
            if binding.is_exported(model) {
                return Some(true);
            }

            for reference in binding.all_reads(model) {
                let greatparent = reference.node().grand_parent()?;
                if let JS_NEW_EXPRESSION = greatparent.kind() {
                    return Some(true);
                }
            }

            Some(false)
        }
        _ => Some(false),
    }
}

impl Rule for UseCamelCase {
    type Query = Semantic<JsAnyCamelCaseName>;
    type State = State;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let name = ctx.query();
        let model = ctx.model();

        match name {
            JsAnyCamelCaseName::JsIdentifierBinding(binding) => {
                let is_non_camel_ok = is_non_camel_ok(binding, model);
                match is_non_camel_ok {
                    Some(false) | None => {
                        let is_variable = binding.parent::<JsVariableDeclarator>().is_some();
                        let is_parameter = binding.parent::<JsFormalParameter>().is_some();
                        let is_function = binding.parent::<JsFunctionDeclaration>().is_some();
                        let is_exported_function = binding
                            .parent::<JsFunctionExportDefaultDeclaration>()
                            .is_some();

                        if is_variable || is_parameter || is_function || is_exported_function {
                            let name = binding.name_token().ok()?;
                            let is_camel_case = check_is_camel(name.text_trimmed());
                            if is_camel_case.is_some() {
                                let is_jsx_component = model.all_reads(binding).any(|reference| {
                                    JsxReferenceIdentifier::can_cast(reference.node().kind())
                                });
                                if !is_jsx_component {
                                    return is_camel_case;
                                }
                            }
                        }

                        None
                    }
                    _ => None,
                }
            }
            JsAnyCamelCaseName::JsLiteralMemberName(name) => {
                let is_method_class = name.parent::<JsMethodClassMember>().is_some();
                let is_getter = name.parent::<JsGetterClassMember>().is_some();
                let is_setter = name.parent::<JsSetterClassMember>().is_some();
                let is_property = name.parent::<JsPropertyClassMember>().is_some();
                if is_method_class || is_getter || is_setter || is_property {
                    let name = name.text();
                    check_is_camel(&name)
                } else {
                    None
                }
            }
            JsAnyCamelCaseName::JsPrivateClassMemberName(name) => {
                let is_property = name.parent::<JsPropertyClassMember>().is_some();
                if is_property {
                    let name = name.text();
                    check_is_camel(&name)
                } else {
                    None
                }
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let binding = ctx.query();

        let symbol_type = match binding.syntax().parent().unwrap().kind() {
            JsSyntaxKind::JS_FORMAL_PARAMETER => "parameters",
            JsSyntaxKind::JS_FUNCTION_DECLARATION => "functions",
            JsSyntaxKind::JS_GETTER_CLASS_MEMBER
            | JsSyntaxKind::JS_SETTER_CLASS_MEMBER
            | JsSyntaxKind::JS_METHOD_CLASS_MEMBER => "methods",
            JsSyntaxKind::JS_PROPERTY_CLASS_MEMBER | JsSyntaxKind::JS_PRIVATE_CLASS_MEMBER_NAME => {
                "properties"
            }
            _ => "variables",
        };

        let diag = RuleDiagnostic::new(
            rule_category!(),
            binding.syntax().text_trimmed_range(),
            markup! {
                "Prefer " {symbol_type} " names in camel case."
            },
        );

        Some(diag)
    }

    fn action(ctx: &RuleContext<Self>, State { new_name }: &Self::State) -> Option<JsRuleAction> {
        let model = ctx.model();
        let mut batch = ctx.root().begin();

        let candidates = (2..).map(|i| format!("{}{}", new_name, i).into());
        let candidates = once(Cow::from(new_name)).chain(candidates);

        match ctx.query() {
            JsAnyCamelCaseName::JsIdentifierBinding(binding) => {
                let renamed =
                    batch.rename_node_declaration_with_retry(model, binding.clone(), candidates);
                if renamed {
                    Some(JsRuleAction {
                        category: ActionCategory::Refactor,
                        applicability: Applicability::Always,
                        message: markup! { "Rename this symbol to camel case" }.to_owned(),
                        mutation: batch,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

declare_node_union! {
    pub(crate) JsAnyCamelCaseName = JsIdentifierBinding | JsLiteralMemberName | JsPrivateClassMemberName
}
