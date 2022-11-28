use crate::utils::batch::JsBatchMutation;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    JsAnyObjectMember, JsGetterObjectMember, JsObjectExpression, JsSetterObjectMember,
};
use rome_js_syntax::{
    JsMethodObjectMember, JsPropertyObjectMember, JsShorthandPropertyObjectMember, TextRange,
};
use rome_rowan::{AstNode, BatchMutationExt};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;

use crate::JsRuleAction;

declare_rule! {
    /// Prevents object literals having more than one property declaration for the same name.
    /// If an object property with the same name is defined multiple times (except when combining a getter with a setter), only the last definition makes it into the object and previous definitions are ignored, which is likely a mistake.
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
        version: "11.0.0",
        name: "noDupeKeys",
        recommended: false, // should be once out of nursery
    }
}

/// An object member defining a single object property.
enum MemberDefinition {
    Getter(JsGetterObjectMember),
    Setter(JsSetterObjectMember),
    Method(JsMethodObjectMember),
    Property(JsPropertyObjectMember),
    ShorthandProperty(JsShorthandPropertyObjectMember),
}
impl MemberDefinition {
    fn name(&self) -> Option<String> {
        match self {
            MemberDefinition::Getter(getter) => {
                getter.name().ok()?.as_js_literal_member_name()?.name().ok()
            }
            MemberDefinition::Setter(setter) => {
                setter.name().ok()?.as_js_literal_member_name()?.name().ok()
            }
            MemberDefinition::Method(method) => {
                method.name().ok()?.as_js_literal_member_name()?.name().ok()
            }
            MemberDefinition::Property(property) => property
                .name()
                .ok()?
                .as_js_literal_member_name()?
                .name()
                .ok(),
            MemberDefinition::ShorthandProperty(shorthand_property) => {
                Some(shorthand_property.name().ok()?.text())
            }
        }
    }

    fn range(&self) -> TextRange {
        match self {
            MemberDefinition::Getter(getter) => getter.range(),
            MemberDefinition::Setter(setter) => setter.range(),
            MemberDefinition::Method(method) => method.range(),
            MemberDefinition::Property(property) => property.range(),
            MemberDefinition::ShorthandProperty(shorthand_property) => shorthand_property.range(),
        }
    }

    fn node(&self) -> JsAnyObjectMember {
        match self {
            MemberDefinition::Getter(getter) => JsAnyObjectMember::from(getter.clone()),
            MemberDefinition::Setter(setter) => JsAnyObjectMember::from(setter.clone()),
            MemberDefinition::Method(method) => JsAnyObjectMember::from(method.clone()),
            MemberDefinition::Property(property) => JsAnyObjectMember::from(property.clone()),
            MemberDefinition::ShorthandProperty(shorthand_property) => {
                JsAnyObjectMember::from(shorthand_property.clone())
            }
        }
    }
}
impl Display for MemberDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Getter(_) => "getter",
            Self::Setter(_) => "setter",
            Self::Method(_) => "method",
            Self::Property(_) => "property value",
            Self::ShorthandProperty(_) => "shorthand property",
        })?;
        if let Some(name) = self.name() {
            f.write_str(" named ")?;
            f.write_str(&name)?;
        }
        Ok(())
    }
}
enum MemberDefinitionError {
    NotASinglePropertyMember,
    BogusMemberType,
}
impl TryFrom<JsAnyObjectMember> for MemberDefinition {
    type Error = MemberDefinitionError;

    fn try_from(member: JsAnyObjectMember) -> Result<Self, Self::Error> {
        match member {
            JsAnyObjectMember::JsGetterObjectMember(member) => Ok(MemberDefinition::Getter(member)),
            JsAnyObjectMember::JsSetterObjectMember(member) => Ok(MemberDefinition::Setter(member)),
            JsAnyObjectMember::JsMethodObjectMember(member) => Ok(MemberDefinition::Method(member)),
            JsAnyObjectMember::JsPropertyObjectMember(member) => {
                Ok(MemberDefinition::Property(member))
            }
            JsAnyObjectMember::JsShorthandPropertyObjectMember(member) => {
                Ok(MemberDefinition::ShorthandProperty(member))
            }
            JsAnyObjectMember::JsSpread(_) => Err(MemberDefinitionError::NotASinglePropertyMember),
            JsAnyObjectMember::JsBogusMember(_) => Err(MemberDefinitionError::BogusMemberType),
        }
    }
}

/// A descriptor for a property that is, as far as we can tell from statically analyzing the object expression,
/// not overwritten by another object member and will make it into the object.
#[derive(Clone)]
enum DefinedProperty {
    Get(TextRange),
    Set(TextRange),
    GetSet(TextRange, TextRange),
    Value(TextRange),
}
impl From<MemberDefinition> for DefinedProperty {
    fn from(definition: MemberDefinition) -> Self {
        match definition {
            MemberDefinition::Getter(getter) => DefinedProperty::Get(getter.range()),
            MemberDefinition::Setter(setter) => DefinedProperty::Set(setter.range()),
            MemberDefinition::Method(method) => DefinedProperty::Value(method.range()),
            MemberDefinition::Property(property) => DefinedProperty::Value(property.range()),
            MemberDefinition::ShorthandProperty(shorthand_property) => {
                DefinedProperty::Value(shorthand_property.range())
            }
        }
    }
}

pub(crate) struct PropertyConflict(DefinedProperty, MemberDefinition);
impl DefinedProperty {
    fn extend_with(
        &self,
        member_definition: MemberDefinition,
    ) -> Result<DefinedProperty, PropertyConflict> {
        match (self, member_definition) {
            // Add missing get/set counterpart
            (DefinedProperty::Set(set_range), MemberDefinition::Getter(getter)) => {
                Ok(DefinedProperty::GetSet(getter.range(), *set_range))
            }

            (DefinedProperty::Get(get_range), MemberDefinition::Setter(setter)) => {
                Ok(DefinedProperty::GetSet(*get_range, setter.range()))
            }
            // Else conflict
            (defined_property, member_definition) => Err(PropertyConflict(
                defined_property.clone(),
                member_definition,
            )),
        }
    }
}

impl Rule for NoDupeKeys {
    type Query = Ast<JsObjectExpression>;
    type State = PropertyConflict;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let mut defined_properties: HashMap<String, DefinedProperty> = HashMap::new();
        let mut signals: Self::Signals = Vec::new();

        for member_definition in node
            .members()
            .into_iter()
            .flatten()
            .filter_map(|member| MemberDefinition::try_from(member).ok())
            // Note that we iterate from last to first property, so that we highlight properties being overwritten as problems and not those that take effect.
            .rev()
        {
            if let Some(member_name) = member_definition.name() {
                match defined_properties.remove(&member_name) {
                    None => {
                        defined_properties
                            .insert(member_name, DefinedProperty::from(member_definition));
                    }
                    Some(defined_property) => {
                        match defined_property.extend_with(member_definition) {
                            Ok(new_defined_property) => {
                                defined_properties.insert(member_name, new_defined_property);
                            }
                            Err(conflict) => {
                                signals.push(conflict);
                                defined_properties.insert(member_name, defined_property);
                            }
                        }
                    }
                }
            }
        }

        signals
    }

    fn diagnostic(
        _ctx: &RuleContext<Self>,
        PropertyConflict(defined_property, member_definition): &Self::State,
    ) -> Option<RuleDiagnostic> {
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            member_definition.range(),
            format!(
                "This {} is later overwritten by an object member with the same name.",
                member_definition
            ),
        );
        diagnostic = match defined_property {
            DefinedProperty::Get(range) => {
                diagnostic.detail(range, "Overwritten with this getter.")
            }
            DefinedProperty::Set(range) => {
                diagnostic.detail(range, "Overwritten with this setter.")
            }
            DefinedProperty::Value(range) => {
                diagnostic.detail(range, "Overwritten with this value.")
            }
            DefinedProperty::GetSet(get_range, set_range) => match member_definition {
                MemberDefinition::Getter(_) => {
                    diagnostic.detail(get_range, "Overwritten with this getter.")
                }
                MemberDefinition::Setter(_) => {
                    diagnostic.detail(set_range, "Overwritten with this setter.")
                }
                MemberDefinition::Method(_)
                | MemberDefinition::Property(_)
                | MemberDefinition::ShorthandProperty(_) => match get_range.ordering(*set_range) {
                    Ordering::Less => diagnostic.detail(set_range, "Overwritten with this setter."),
                    Ordering::Greater => {
                        diagnostic.detail(get_range, "Overwritten with this getter.")
                    }
                    Ordering::Equal => {
                        panic!(
                            "The ranges of the property getter and property setter cannot overlap."
                        )
                    }
                },
            },
        };
        diagnostic = diagnostic.note("If an object property with the same name is defined multiple times (except when combining a getter with a setter), only the last definition makes it into the object and previous definitions are ignored.");

        Some(diagnostic)
    }

    fn action(
        ctx: &RuleContext<Self>,
        PropertyConflict(_, member_definition): &Self::State,
    ) -> Option<JsRuleAction> {
        let mut batch = ctx.root().begin();
        batch.remove_js_object_member(&member_definition.node());
        Some(JsRuleAction {
            category: rome_analyze::ActionCategory::QuickFix,
            // The property initialization could contain side effects
            applicability: rome_diagnostics::Applicability::MaybeIncorrect,
            message: markup!("Remove this " {member_definition.to_string()}).to_owned(),
            mutation: batch,
        })
    }
}
