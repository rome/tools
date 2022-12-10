use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_js_semantic::{Reference, ReferencesExtensions};
use rome_js_syntax::{JsClassDeclaration, JsClassExpression, JsIdentifierBinding};
use rome_rowan::{declare_node_union, AstNode};

use crate::semantic_services::Semantic;

declare_rule! {
    /// Disallow reassigning class members.
    ///
    /// A class declaration creates a variable that we can modify, however, the modification is a mistake in most cases.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class A {}
    /// A = 0;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// A = 0;
    /// class A {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class A {
    /// 	b() {
    /// 		A = 0;
    /// 	}
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let A = class A {
    /// 	b() {
    /// 		A = 0;
    /// 		// `let A` is shadowed by the class name.
    /// 	}
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// let A = class A {}
    /// A = 0; // A is a variable.
    /// ```
    ///
    /// ```js
    /// let A = class {
    ///     b() {
    ///         A = 0; // A is a variable.
    ///     }
    /// }
    /// ```
    ///
    /// ```js
    /// class A {
    /// 	b(A) {
    /// 		A = 0; // A is a parameter.
    /// 	}
    /// }
    /// ```
    ///
    pub(crate) NoClassAssign {
        version: "12.0.0",
        name: "noClassAssign",
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) AnyClass = JsClassDeclaration | JsClassExpression
}

impl Rule for NoClassAssign {
    type Query = Semantic<JsIdentifierBinding>;
    type State = Vec<Reference>;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        let model = ctx.model();

        // ensures that we only verifies class bindings
        binding.parent::<AnyClass>()?;

        let all_writes: Vec<Reference> = binding.all_writes(model).collect();

        if all_writes.is_empty() {
            None
        } else {
            Some(all_writes)
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, references: &Self::State) -> Option<RuleDiagnostic> {
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            "Don't reassign classes.",
        );

        for reference in references.iter() {
            diagnostic =
                diagnostic.detail(reference.syntax().text_trimmed_range(), "Reassigned here.");
        }

        Some(diagnostic)
    }
}
