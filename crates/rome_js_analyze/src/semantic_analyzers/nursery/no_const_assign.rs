use crate::semantic_services::Semantic;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    JsAnyBindingPattern, JsArrayBindingPatternElementList, JsForVariableDeclaration,
    JsIdentifierAssignment, JsObjectBindingPatternShorthandProperty, JsVariableDeclaration,
    JsVariableDeclarator,
};
use rome_rowan::{AstNode, TextRange};

declare_rule! {
    /// Prevents from having `const` variables being re-assigned.
    ///
    /// Trying to assign a value to a `const` will cause an `TypeError` when the code is executed.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const a = 1;
    /// a = 4;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const a = 2;
    /// a += 1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const a = 1;
    /// ++a;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const a = 1, b = 2;
    ///
    /// a = 2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const a = 10;
    /// let b = 10;
    /// b = 20;
    /// ```
    ///
    pub(crate) NoConstAssign {
        version: "10.0.0",
        name: "noConstAssign",
        recommended: false,
    }
}

impl Rule for NoConstAssign {
    type Query = Semantic<JsIdentifierAssignment>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        let declared_binding = model.declaration(node)?;

        let js_any_binding_parent = declared_binding
            .syntax()
            .ancestors()
            .find(|n| !JsAnyBindingPattern::can_cast(n.kind()))?;

        let js_any_binding_parent_kind = js_any_binding_parent.kind();
        if JsVariableDeclarator::can_cast(js_any_binding_parent_kind)
            || JsArrayBindingPatternElementList::can_cast(js_any_binding_parent_kind)
            || JsObjectBindingPatternShorthandProperty::can_cast(js_any_binding_parent_kind)
        {
            for n in js_any_binding_parent.ancestors() {
                if let Some(js_variable_declaration) = JsVariableDeclaration::cast_ref(&n) {
                    if js_variable_declaration.is_const() {
                        return Some(declared_binding.syntax().text_trimmed_range());
                    }
                }
                if let Some(js_for_variable_declaration) = JsForVariableDeclaration::cast_ref(&n) {
                    if js_for_variable_declaration.is_const() {
                        return Some(declared_binding.syntax().text_trimmed_range());
                    }
                }
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let name = node.name_token().ok()?;
        let name = name.text_trimmed();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {"Can't assign "<Emphasis>{name}</Emphasis>" because it's a constant"},
            )
            .detail(
                state,
                markup! {"This is where the variable is defined as constant"},
            ),
        )
    }
}
