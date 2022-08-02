use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Rule, RuleCategory};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_semantic::{AllReferencesExtensions, Reference};
use rome_js_syntax::{
    JsAnyBinding, JsAnyBindingPattern, JsAnyExpression, JsIdentifierExpression,
    JsVariableDeclarator,
};
use rome_rowan::{BatchMutationExt, SyntaxNodeCast};

use crate::{semantic_services::Semantic, utils::remove_declarator, JsRuleAction};

declare_rule! {
    /// Provides a refactor to inline variables
    ///
    /// ## Examples
    ///
    /// ```js
    /// let variable = expression();
    /// statement(variable);
    /// ```
    pub(crate) InlineVariable {
        version: "0.8.0",
        name: "inlineVariable",
        recommended: false,
    }
}

impl Rule for InlineVariable {
    const CATEGORY: RuleCategory = RuleCategory::Action;

    type Query = Semantic<JsVariableDeclarator>;
    type State = (Vec<Reference>, JsAnyExpression);
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let semantic_model = ctx.model();
        let declarator = ctx.query();

        let id = declarator.id().ok()?;
        let binding = match id {
            JsAnyBindingPattern::JsAnyBinding(JsAnyBinding::JsIdentifierBinding(binding)) => {
                binding
            }
            JsAnyBindingPattern::JsAnyBinding(JsAnyBinding::JsUnknownBinding(_))
            | JsAnyBindingPattern::JsArrayBindingPattern(_)
            | JsAnyBindingPattern::JsObjectBindingPattern(_) => return None,
        };

        let mut references = Vec::new();
        for reference in binding.all_references(semantic_model) {
            if reference.is_write() {
                return None;
            }

            references.push(reference);
        }

        let initializer = declarator.initializer()?;
        let expression = initializer.expression().ok()?;
        Some((references, expression))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let (references, expression) = state;

        remove_declarator(&mut mutation, node);

        for reference in references {
            let node = reference
                .node()
                .parent()?
                .cast::<JsIdentifierExpression>()?;

            mutation.replace_node(
                JsAnyExpression::JsIdentifierExpression(node),
                expression.clone(),
            );
        }

        Some(JsRuleAction {
            category: ActionCategory::Refactor,
            applicability: Applicability::Always,
            message: markup! { "Inline variable" }.to_owned(),
            mutation,
        })
    }
}
