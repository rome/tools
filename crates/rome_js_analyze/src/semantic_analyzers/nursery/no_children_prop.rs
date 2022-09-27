use crate::react::is_react_create_element;
use crate::semantic_services::Semantic;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{JsCallExpression, JsPropertyObjectMember, JsxAttribute, JsxName};
use rome_rowan::{declare_node_union, AstNode};
declare_rule! {
    /// Prevent passing of **children** as props.
    ///
    /// When using JSX, the children should be nested between the opening and closing tags.
    /// When not using JSX, the children should be passed as additional arguments to `React.createElement`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// <FirstComponent children={'foo'} />
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// React.createElement('div', { children: 'foo' });
    /// ```
    pub(crate) NoChildrenProp {
        version: "0.10.0",
        name: "noChildrenProp",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) NoChildrenPropQuery = JsxAttribute | JsCallExpression
}

pub(crate) enum NoChildrenPropState {
    JsxProp(JsxName),
    MemberProp(JsPropertyObjectMember),
}

impl Rule for NoChildrenProp {
    const CATEGORY: RuleCategory = RuleCategory::Syntax;
    type Query = Semantic<NoChildrenPropQuery>;
    type State = NoChildrenPropState;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        match node {
            NoChildrenPropQuery::JsxAttribute(attribute) => {
                let name = attribute.name().ok()?;
                let name = name.as_jsx_name()?;
                if name.value_token().ok()?.text() == "children" {
                    return Some(NoChildrenPropState::JsxProp(name.clone()));
                }

                None
            }
            NoChildrenPropQuery::JsCallExpression(call_expression) => {
                let model = ctx.model();
                if let Some(react_create_element) = is_react_create_element(call_expression, model)
                {
                    let children_prop = react_create_element.find_prop_by_name("children");

                    if let Some(children_prop) = children_prop {
                        return Some(NoChildrenPropState::MemberProp(children_prop));
                    }
                }
                None
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (range, footer_help) = match state {
            NoChildrenPropState::JsxProp(name) => {
                (
                    name.syntax().text_trimmed_range(),
                    (markup! {
                     "The canonical way to pass children in React is to use JSX elements"
                    }).to_owned()
                )
            }
            NoChildrenPropState::MemberProp(children_prop) => (
                children_prop.name().ok()?.syntax().text_trimmed_range(),
                (markup! {
                     "The canonical way to pass children in React is to use additional arguments to React.createElement"
                }).to_owned()
            ),
        };

        Some(
            RuleDiagnostic::new(
                range,
                markup! {
                    "Avoid passing "<Emphasis>"children"</Emphasis>" using a prop"
                },
            )
            .footer_note(footer_help),
        )
    }
}
