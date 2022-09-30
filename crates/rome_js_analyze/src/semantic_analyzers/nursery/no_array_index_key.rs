use crate::react::{is_react_call_api, ReactApiCall, ReactCloneElementCall};
use crate::semantic_services::Semantic;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_js_semantic::SemanticModel;
use rome_js_syntax::{
    JsArrowFunctionExpression, JsCallExpression, JsExpressionStatement, JsFunctionDeclaration,
    JsFunctionExpression, JsIdentifierBinding, JsIdentifierExpression, JsMethodClassMember,
    JsMethodObjectMember, JsParameterList, JsPropertyObjectMember, JsReferenceIdentifier,
    JsxAttribute, JsxOpeningElement, JsxSelfClosingElement,
};
use rome_rowan::{declare_node_union, AstNode, AstSeparatedList};

declare_rule! {
    /// Discourage the usage of Array index in keys.
    ///
    /// > We don’t recommend using indexes for keys if the order of items may change.
    /// This can negatively impact performance and may cause issues with component state.
    /// Check out Robin Pokorny’s article for an
    /// [in-depth explanation on the negative impacts of using an index as a key](https://robinpokorny.com/blog/index-as-a-key-is-an-anti-pattern/).
    /// If you choose not to assign an explicit key to list items then React will default to using indexes as keys.
    ///
    /// Source [React documentation](https://reactjs.org/docs/lists-and-keys.html#keys)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// something.forEach((Element, index) => {
    ///     <Component key={index} >foo</Component>
    /// });
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// React.Children.map(this.props.children, (child, index) => (
    ///     React.cloneElement(child, { key: index })
    /// ))
    /// ```
    pub(crate) NoArrayIndexKey {
        version: "0.10.0",
        name: "noArrayIndexKey",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) NoArrayIndexKeyQuery = JsxOpeningElement | JsxSelfClosingElement | JsCallExpression
}

declare_node_union! {
    pub(crate) KeyPropWithArrayIndex = JsxAttribute | JsPropertyObjectMember
}

impl KeyPropWithArrayIndex {
    /// Extracts the reference from the possible invalid prop
    fn as_js_reference_identifier(&self) -> Option<JsReferenceIdentifier> {
        match self {
            KeyPropWithArrayIndex::JsxAttribute(attribute) => attribute
                .initializer()?
                .value()
                .ok()?
                .as_jsx_expression_attribute_value()?
                .expression()
                .ok()?
                .as_js_identifier_expression()?
                .name()
                .ok(),
            KeyPropWithArrayIndex::JsPropertyObjectMember(object_member) => object_member
                .value()
                .ok()?
                .as_js_identifier_expression()?
                .name()
                .ok(),
        }
    }
}

impl NoArrayIndexKeyQuery {
    const fn is_call_expression(&self) -> bool {
        matches!(self, NoArrayIndexKeyQuery::JsCallExpression(_))
    }

    fn find_attribute_with_key_name(&self, model: &SemanticModel) -> Option<KeyPropWithArrayIndex> {
        match self {
            NoArrayIndexKeyQuery::JsxOpeningElement(element) => element
                .find_attribute_by_name("key")
                .ok()?
                .map(KeyPropWithArrayIndex::from),
            NoArrayIndexKeyQuery::JsxSelfClosingElement(element) => element
                .find_attribute_by_name("key")
                .ok()?
                .map(KeyPropWithArrayIndex::from),
            NoArrayIndexKeyQuery::JsCallExpression(expression) => {
                let create_clone_element =
                    ReactCloneElementCall::from_call_expression(expression, model)?;
                create_clone_element
                    .find_prop_by_name("key")
                    .map(KeyPropWithArrayIndex::from)
            }
        }
    }

    fn find_function_like_ancestor(&self) -> Option<FunctionLike> {
        let element = match self {
            NoArrayIndexKeyQuery::JsxOpeningElement(element) => element.syntax(),
            NoArrayIndexKeyQuery::JsxSelfClosingElement(element) => element.syntax(),
            NoArrayIndexKeyQuery::JsCallExpression(expression) => expression.syntax(),
        };
        element.ancestors().find_map(FunctionLike::cast)
    }
}

pub(crate) struct NoArrayIndexKeyState {
    /// The incorrect prop
    incorrect_prop: JsReferenceIdentifier,
    /// Where the incorrect prop was defined
    binding_origin: JsIdentifierBinding,
}

impl Rule for NoArrayIndexKey {
    const CATEGORY: RuleCategory = RuleCategory::Lint;
    type Query = Semantic<NoArrayIndexKeyQuery>;
    type State = NoArrayIndexKeyState;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let attribute_key = node.find_attribute_with_key_name(model)?;
        let reference = attribute_key.as_js_reference_identifier()?;

        let value = reference.value_token().ok()?;
        let function_parent = node.find_function_like_ancestor()?;
        let key_value = value.text_trimmed();

        let is_inside_array_method = is_inside_array_method(&function_parent)?;

        if is_inside_array_method {
            if node.is_call_expression() {
                // 1. We scan the parent and look for a potential `React.Children` method call
                // 2. If found, then we retrieve the incorrect `index`
                // 3. If we fail to find a `React.Children` method call, then we try to find and `Array.` method call
                let binding_origin = find_react_children_function_argument(&function_parent, model)
                    .and_then(|function_argument| {
                        find_array_index_key(key_value, &function_argument)
                    })
                    .or_else(|| find_array_index_key(key_value, &function_parent))?;
                Some(NoArrayIndexKeyState {
                    binding_origin,
                    incorrect_prop: reference,
                })
            } else {
                let binding_origin = find_array_index_key(key_value, &function_parent)?;

                Some(NoArrayIndexKeyState {
                    binding_origin,
                    incorrect_prop: reference,
                })
            }
        } else {
            None
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let NoArrayIndexKeyState {
            binding_origin: incorrect_key,
            incorrect_prop,
        } = state;
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            incorrect_prop.syntax().text_trimmed_range(),
            markup! {"Avoid using the index of an array as key property in an element."},
        )
        .secondary(
            incorrect_key.syntax().text_trimmed_range(),
            markup! {"This is the source of the key value."},
        ).footer_note(
            markup! {"The order of the items may change, and this also affects performances and component state."}
        ).footer_note(
            markup! {
                "Check the "<Hyperlink href="https://reactjs.org/docs/lists-and-keys.html#keys">"React documentation"</Hyperlink>". "
            }
        );

        Some(diagnostic)
    }
}

/// Given a function like node, it navigates the `callee` of its parent
/// and check if the method called by this function belongs to an array method
///
/// ```js
/// Array.map(() => {
///     return <Component />
/// })
/// ```
///
/// Given this example, the input node is the arrow function and we navigate to
/// retrieve the name `map` call and we check if it belongs to an `Array.prototype` method.
fn is_inside_array_method(function_like_node: &FunctionLike) -> Option<bool> {
    let function_parent = function_like_node
        .syntax()
        .ancestors()
        .find_map(|ancestor| JsExpressionStatement::cast_ref(&ancestor))?;

    let name = function_parent
        .expression()
        .ok()?
        .as_js_call_expression()?
        .callee()
        .ok()?
        .as_js_static_member_expression()?
        .member()
        .ok()?
        .as_js_name()?
        .value_token()
        .ok()?;

    Some(matches!(
        name.text_trimmed(),
        "map"
            | "forEach"
            | "filter"
            | "some"
            | "every"
            | "find"
            | "findIndex"
            | "reduce"
            | "reduceRight"
    ))
}

declare_node_union! {
    pub(crate) FunctionLike =  JsFunctionDeclaration
        | JsFunctionExpression
        | JsArrowFunctionExpression
        | JsMethodClassMember
        | JsMethodObjectMember
}

impl FunctionLike {
    fn parameters(&self) -> Option<JsParameterList> {
        let list = match self {
            FunctionLike::JsFunctionDeclaration(node) => node.parameters().ok()?.items(),
            FunctionLike::JsFunctionExpression(node) => node.parameters().ok()?.items(),
            FunctionLike::JsArrowFunctionExpression(node) => {
                node.parameters().ok()?.as_js_parameters()?.items()
            }
            FunctionLike::JsMethodClassMember(node) => node.parameters().ok()?.items(),
            FunctionLike::JsMethodObjectMember(node) => node.parameters().ok()?.items(),
        };
        Some(list)
    }
}
/// It checks if the index binding comes from an array function and has the same name
/// used inside the `"key"` prop.
///
/// ```js
/// Array.forEach((element, index, array) => {
///     <Component key={index}/>
/// });
/// ```
///
/// We scan all the parameters and we check if its name
fn find_array_index_key(
    key_name: &str,
    function_like_node: &FunctionLike,
) -> Option<JsIdentifierBinding> {
    let parameters = function_like_node.parameters()?;
    parameters.iter().find_map(|parameter| {
        let parameter = parameter
            .ok()?
            .as_js_any_formal_parameter()?
            .as_js_formal_parameter()?
            .binding()
            .ok()?;
        let last_parameter = parameter.as_js_any_binding()?.as_js_identifier_binding()?;

        if last_parameter.name_token().ok()?.text_trimmed() == key_name {
            Some(last_parameter.clone())
        } else {
            None
        }
    })
}

/// This function initially  checks if the [JsCallExpression] matches the following cases;
///
/// ```js
/// React.Children.map();
/// React.Children.forEach();
/// Children.map();
/// Children.forEach()
/// ```
/// Then, it navigates the arguments and return the second argument, only if it's
///
/// For example
///
/// ```js
/// React.Children.map(this.props.children); // not correct
/// React.Children.map(this.props.children, (child, index) => {}) // correct, returns arrow function expression
/// React.Children.map(this.props.children, function(child, index) {}) // correct, returns function expression
/// ```
///
fn find_react_children_function_argument(
    call_expression: &FunctionLike,
    model: &SemanticModel,
) -> Option<FunctionLike> {
    let function_parent = call_expression
        .syntax()
        .ancestors()
        .find_map(|ancestor| JsExpressionStatement::cast_ref(&ancestor))?;

    let call_expression = function_parent.expression().ok()?;
    let call_expression = call_expression.as_js_call_expression()?;

    let member_expression = call_expression.callee().ok()?;
    let member_expression = member_expression.as_js_static_member_expression()?;

    let member = member_expression.member().ok()?;
    let array_call = matches!(
        member.as_js_name()?.value_token().ok()?.text_trimmed(),
        "forEach" | "map"
    );

    let object = member_expression.object().ok()?;

    let mut is_react_children = false;
    // case we have `Children`
    if let Some(identifier) = JsIdentifierExpression::cast_ref(object.syntax()) {
        if identifier.name().ok()?.value_token().ok()?.text_trimmed() == "Children" {
            is_react_children = array_call;
        }
    } else {
        // case we have `React.Children`
        is_react_children = is_react_call_api(&object, model, "Children")? && array_call;
    }

    if is_react_children {
        let arguments = call_expression.arguments().ok()?;
        let arguments = arguments.args();
        let mut arguments = arguments.into_iter();
        let _ = arguments.next()?.ok()?;
        let second_argument = arguments.next()?.ok()?;
        let second_argument = second_argument.as_js_any_expression()?;

        FunctionLike::cast(second_argument.clone().into_syntax())
    } else {
        None
    }
}
