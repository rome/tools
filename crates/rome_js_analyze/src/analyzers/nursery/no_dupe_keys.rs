use crate::utils::batch::JsBatchMutation;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{JsAnyObjectMember, JsObjectExpression};
use rome_rowan::{AstNode, BatchMutationExt};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;

use crate::JsRuleAction;

declare_rule! {
    /// Prevents object literals having more than one property declaration for the same key.
    /// If an object property with the same key is defined multiple times (except when combining a getter with a setter), only the last definition makes it into the object and previous definitions are ignored, which is likely a mistake.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const obj = {
    ///		a: 1,
    ///		a: 2,
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const obj = {
    ///		set a(v) {},
    ///		a: 2,
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const obj = {
    ///		a: 1,
    ///		b: 2,
    /// }
    /// ```
    ///
    /// ```js
    /// const obj = {
    ///		get a() { return 1; },
    ///		set a(v) {},
    /// }
    /// ```
    ///
    pub(crate) NoDupeKeys {
        version: "10.1.0",
        name: "noDupeKeys",
        recommended: false, // should be once out of nursery
    }
}

enum PropertyType {
    Getter,
    Setter,
    Value,
}
impl Display for PropertyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Getter => "getter",
            Self::Setter => "setter",
            Self::Value => "value",
        })
    }
}
struct PropertyDefinition(PropertyType, JsAnyObjectMember);

#[derive(Clone)]
enum DefinedProperty {
    Getter(JsAnyObjectMember),
    Setter(JsAnyObjectMember),
    GetterSetter(JsAnyObjectMember, JsAnyObjectMember),
    Value(JsAnyObjectMember),
}
impl From<PropertyDefinition> for DefinedProperty {
    fn from(PropertyDefinition(property_type, range): PropertyDefinition) -> Self {
        match property_type {
            PropertyType::Getter => DefinedProperty::Getter(range),
            PropertyType::Setter => DefinedProperty::Setter(range),
            PropertyType::Value => DefinedProperty::Value(range),
        }
    }
}

pub(crate) struct PropertyConflict(DefinedProperty, PropertyDefinition);

impl Rule for NoDupeKeys {
    type Query = Ast<JsObjectExpression>;
    type State = PropertyConflict;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let mut defined_properties: HashMap<String, DefinedProperty> = HashMap::new();
        let mut signals: Self::Signals = Vec::new();

        for member in node
            .members()
            .into_iter()
            .flatten()
            // Note that we iterate from last to first property, so that we highlight properties being overwritten as problems and not those that take effect.
            .rev()
        {
            let property_name = get_property_name(&member);
            if let Some(property_name) = property_name {
                let property_definition = PropertyDefinition(
                    match member {
                        JsAnyObjectMember::JsGetterObjectMember(_) => PropertyType::Getter,
                        JsAnyObjectMember::JsSetterObjectMember(_) => PropertyType::Setter,
                        _ => PropertyType::Value,
                    },
                    member,
                );
                let defined_property = defined_properties.remove(&property_name);
                match (defined_property, property_definition) {
                    // Property not seen before
                    (None, property_definition) => {
                        // Put a new definition.
                        defined_properties
                            .insert(property_name, DefinedProperty::from(property_definition));
                    }
                    // Only get/set counterpart seen before
                    (
                        Some(DefinedProperty::Setter(set_range)),
                        PropertyDefinition(PropertyType::Getter, get_range),
                    )
                    | (
                        Some(DefinedProperty::Getter(get_range)),
                        PropertyDefinition(PropertyType::Setter, set_range),
                    ) => {
                        // Put definition back with the missing get or set filled.
                        defined_properties.insert(
                            property_name,
                            DefinedProperty::GetterSetter(get_range, set_range),
                        );
                    }
                    // Trying to define something that was already defined
                    (
                        Some(defined_property @ DefinedProperty::Getter(_)),
                        property_definition @ (PropertyDefinition(PropertyType::Getter, _)
                        | PropertyDefinition(PropertyType::Value, _)),
                    )
                    | (
                        Some(defined_property @ DefinedProperty::Setter(_)),
                        property_definition @ (PropertyDefinition(PropertyType::Setter, _)
                        | PropertyDefinition(PropertyType::Value, _)),
                    )
                    | (
                        Some(
                            defined_property @ (DefinedProperty::Value(_)
                            | DefinedProperty::GetterSetter(..)),
                        ),
                        property_definition,
                    ) => {
                        // Register the conflict.
                        signals.push(PropertyConflict(
                            defined_property.clone(),
                            property_definition,
                        ));
                        // Put definition back unchanged.
                        defined_properties.insert(property_name, defined_property);
                    }
                }
            }
        }

        signals
    }

    fn diagnostic(
        _ctx: &RuleContext<Self>,
        PropertyConflict(defined_property, PropertyDefinition(property_type, node)): &Self::State,
    ) -> Option<RuleDiagnostic> {
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            format!(
                "This property {} is later overwritten by a property with the same name.",
                property_type
            ),
        );
        diagnostic = match defined_property {
            DefinedProperty::Getter(node) => {
                diagnostic.detail(node.range(), "Overwritten by this getter.")
            }
            DefinedProperty::Setter(node) => {
                diagnostic.detail(node.range(), "Overwritten by this setter.")
            }
            DefinedProperty::Value(node) => {
                diagnostic.detail(node.range(), "Overwritten with this value.")
            }
            DefinedProperty::GetterSetter(getter_node, setter_node) => {
                match property_type {
                    PropertyType::Getter => {
                        diagnostic.detail(getter_node.range(), "Overwritten by this getter.")
                    }
                    PropertyType::Setter => {
                        diagnostic.detail(setter_node.range(), "Overwritten by this setter.")
                    }
                    PropertyType::Value => {
                        match getter_node.range().ordering(setter_node.range()) {
                            Ordering::Less => diagnostic
                                .detail(setter_node.range(), "Overwritten by this setter."),
                            Ordering::Greater => diagnostic
                                .detail(getter_node.range(), "Overwritten by this getter."),
                            Ordering::Equal => {
                                panic!("The ranges of the property getter and property setter cannot overlap.")
                            }
                        }
                    }
                }
            }
        };
        diagnostic = diagnostic.note("If an object property with the same key is defined multiple times (except when combining a getter with a setter), only the last definition makes it into the object and previous definitions are ignored.");

        Some(diagnostic)
    }

    fn action(
        ctx: &RuleContext<Self>,
        PropertyConflict(_, PropertyDefinition(property_type, node)): &Self::State,
    ) -> Option<JsRuleAction> {
        let mut batch = ctx.root().begin();
        batch.remove_js_object_member(node);
        Some(JsRuleAction {
            category: rome_analyze::ActionCategory::QuickFix,
            // The property initialization could contain side effects
            applicability: rome_diagnostics::Applicability::MaybeIncorrect,
            message: markup!("Remove this property " {property_type.to_string()}).to_owned(),
            mutation: batch,
        })
    }
}

fn get_property_name(member: &JsAnyObjectMember) -> Option<String> {
    match member {
        JsAnyObjectMember::JsGetterObjectMember(member) => {
            member.name().ok()?.as_js_literal_member_name()?.name().ok()
        }
        JsAnyObjectMember::JsMethodObjectMember(member) => {
            member.name().ok()?.as_js_literal_member_name()?.name().ok()
        }
        JsAnyObjectMember::JsPropertyObjectMember(member) => {
            member.name().ok()?.as_js_literal_member_name()?.name().ok()
        }
        JsAnyObjectMember::JsSetterObjectMember(member) => {
            member.name().ok()?.as_js_literal_member_name()?.name().ok()
        }
        JsAnyObjectMember::JsShorthandPropertyObjectMember(member) => {
            Some(member.name().ok()?.text())
        }
        JsAnyObjectMember::JsSpread(_) | JsAnyObjectMember::JsUnknownMember(_) => None,
    }
}
