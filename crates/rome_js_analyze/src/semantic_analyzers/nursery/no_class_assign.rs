use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_js_semantic::ReferencesExtensions;
use rome_js_syntax::{AnyJsClass, JsIdentifierBinding};
use rome_rowan::AstNode;

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

impl Rule for NoClassAssign {
    type Query = Semantic<AnyJsClass>;
    type State = JsIdentifierBinding;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let binding = node.id().ok()??;
        let binding = binding.as_js_identifier_binding()?;

        if binding.all_writes(model).count() > 0 {
            Some(binding.clone())
        } else {
            None
        }
    }

    fn diagnostic(_: &RuleContext<Self>, class_id: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            class_id.range(),
            "Don't reassign classes.",
        ))
    }
}
