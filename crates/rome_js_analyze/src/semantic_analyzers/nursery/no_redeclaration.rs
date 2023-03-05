use crate::semantic_services::SemanticServices;
use rome_analyze::declare_rule;
use rome_analyze::{context::RuleContext, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_semantic::{Binding, Scope};
use rome_js_syntax::{TextRange, TsMethodSignatureClassMember};
use rome_rowan::AstNode;
use std::{collections::HashMap, vec::IntoIter};

declare_rule! {
    /// Eliminate variables that have multiple declarations in the same scope.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 3;
    /// var a = 10;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class C {
    ///     static {
    ///         var c = 3;
    ///         var c = 10;
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// var a = 3;
    /// a = 10;
    /// ```
    ///
    /// ```ts
    /// class Foo {
    ///     bar(a: A);
    ///     bar(a: A, b: B);
    ///     bar(a: A, b: B) {}
    /// }
    /// ```
    ///
    pub(crate) NoRedeclaration {
        version: "12.0.0",
        name: "noRedeclaration",
        recommended: true,
    }
}

type Duplicates = HashMap<String, Vec<Binding>>;

type Redeclaration = (String, TextRange, Binding);

impl Rule for NoRedeclaration {
    type Query = SemanticServices;
    type State = Redeclaration;
    type Signals = IntoIter<Redeclaration>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut redeclarations = Vec::default();
        for scope in ctx.query().scopes() {
            check_redeclarations_in_single_scope(&scope, &mut redeclarations);
        }
        redeclarations.into_iter()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (name, text_range, binding) = state;
        let diag = RuleDiagnostic::new(
            rule_category!(),
            binding.syntax().text_trimmed_range(),
            markup! {
               "Shouldn't redeclare '"{ name }"'. Consider to delete it or rename it"
            },
        )
        .detail(
            text_range,
            markup! {
               "'"{ name }"' is defined here."
            },
        );
        Some(diag)
    }
}

fn check_redeclarations_in_single_scope(scope: &Scope, redeclarations: &mut Vec<Redeclaration>) {
    let mut duplicates = Duplicates::default();
    let bindings = scope.bindings();
    for binding in bindings {
        let name = binding.tree().text();
        duplicates.entry(name).or_default().push(binding)
    }

    // only keep the actual re-declarations
    duplicates.retain(|_, list| list.len() > 1);

    for (name, list) in duplicates {
        let first_binding_range = list[0].syntax().text_trimmed_range();
        list.into_iter()
            .skip(1) // skip the first binding
            .for_each(|binding| {
                if !binding
                    .syntax()
                    .ancestors()
                    .any(|node| TsMethodSignatureClassMember::can_cast(node.kind()))
                {
                    redeclarations.push((name.clone(), first_binding_range, binding))
                }
            })
    }
}
