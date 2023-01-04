use std::collections::HashMap;

use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_js_syntax::{
    AnyJsClassMember, AnyJsClassMemberName, AnyJsMethodModifier, AnyJsPropertyModifier,
    JsClassDeclaration, JsClassExpression, JsClassMemberList, JsGetterClassMember,
    JsMethodClassMember, JsMethodModifierList, JsPropertyClassMember, JsPropertyModifierList,
    JsSetterClassMember, TextRange,
};
use rome_rowan::{declare_node_union, AstNode};

declare_rule! {
    /// Disallow duplicate class members.
    ///
    /// If there are declarations of the same name in class members,
    /// the last declaration overwrites other declarations silently.
    /// It can cause unexpected behaviors.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class Foo {
    ///   bar() { }
    ///   bar() { }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class Foo {
    ///   bar() { }
    ///   get bar() { }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class Foo {
    ///   bar;
    ///   bar() { }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class Foo {
    ///   static bar() { }
    ///   static bar() { }
    /// }
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// class Foo {
    ///   bar() { }
    ///   qux() { }
    /// }
    /// ```
    ///
    /// ```js
    /// class Foo {
    ///   set bar(value) { }
    ///   get bar() { }
    /// }
    /// ```
    ///
    /// ```js
    /// class Foo {
    ///   bar;
    ///   qux;
    /// }
    /// ```
    ///
    /// ```js
    /// class Foo {
    ///   bar;
    ///   qux() { }
    /// }
    /// ```
    ///
    /// ```js
    /// class Foo {
    ///   static bar() { }
    ///   bar() { }
    /// }
    /// ```
    ///
    pub(crate) NoDuplicateClassMembers {
        version: "next",
        name: "noDuplicateClassMembers",
        recommended: true,
    }
}

fn get_member_name_string(member_name_node: AnyJsClassMemberName) -> Option<String> {
    match member_name_node {
        AnyJsClassMemberName::JsLiteralMemberName(node) => node.name().ok(),
        AnyJsClassMemberName::JsPrivateClassMemberName(node) => Some(node.text()),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AccessType {
    Public,
    Private,
}

fn get_access_type(member_name_node: AnyJsClassMemberName) -> Option<AccessType> {
    match member_name_node {
        AnyJsClassMemberName::JsPrivateClassMemberName(_) => Some(AccessType::Private),
        _ => Some(AccessType::Public),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum StaticType {
    Static,
    NonStatic,
}

#[allow(clippy::enum_variant_names)]
enum AnyModifierList {
    JsPropertyModifierList(JsPropertyModifierList),
    JsMethodModifierList(JsMethodModifierList),
}

fn get_static_type(modifier_list: AnyModifierList) -> StaticType {
    match modifier_list {
        AnyModifierList::JsPropertyModifierList(node) => {
            if node
                .into_iter()
                .any(|m| matches!(m, AnyJsPropertyModifier::JsStaticModifier(_)))
            {
                StaticType::Static
            } else {
                StaticType::NonStatic
            }
        }
        AnyModifierList::JsMethodModifierList(node) => {
            if node
                .into_iter()
                .any(|m| matches!(m, AnyJsMethodModifier::JsStaticModifier(_)))
            {
                StaticType::Static
            } else {
                StaticType::NonStatic
            }
        }
    }
}

declare_node_union! {
    pub(crate) ClassMemberDefinition = JsGetterClassMember | JsMethodClassMember | JsPropertyClassMember | JsSetterClassMember
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum MemberType {
    Normal,
    Getter,
    Setter,
}

impl ClassMemberDefinition {
    fn member_name(&self) -> Option<String> {
        match self {
            ClassMemberDefinition::JsGetterClassMember(node) => {
                get_member_name_string(node.name().ok()?)
            }
            ClassMemberDefinition::JsMethodClassMember(node) => {
                get_member_name_string(node.name().ok()?)
            }
            ClassMemberDefinition::JsPropertyClassMember(node) => {
                get_member_name_string(node.name().ok()?)
            }
            ClassMemberDefinition::JsSetterClassMember(node) => {
                get_member_name_string(node.name().ok()?)
            }
        }
    }

    fn member_type(&self) -> MemberType {
        match self {
            ClassMemberDefinition::JsGetterClassMember(_) => MemberType::Getter,
            ClassMemberDefinition::JsSetterClassMember(_) => MemberType::Setter,
            _ => MemberType::Normal,
        }
    }

    fn access_type(&self) -> Option<AccessType> {
        match self {
            ClassMemberDefinition::JsGetterClassMember(node) => get_access_type(node.name().ok()?),
            ClassMemberDefinition::JsMethodClassMember(node) => get_access_type(node.name().ok()?),
            ClassMemberDefinition::JsPropertyClassMember(node) => {
                get_access_type(node.name().ok()?)
            }
            ClassMemberDefinition::JsSetterClassMember(node) => get_access_type(node.name().ok()?),
        }
    }

    fn static_type(&self) -> StaticType {
        match self {
            ClassMemberDefinition::JsGetterClassMember(node) => {
                get_static_type(AnyModifierList::JsMethodModifierList(node.modifiers()))
            }
            ClassMemberDefinition::JsMethodClassMember(node) => {
                get_static_type(AnyModifierList::JsMethodModifierList(node.modifiers()))
            }
            ClassMemberDefinition::JsPropertyClassMember(node) => {
                get_static_type(AnyModifierList::JsPropertyModifierList(node.modifiers()))
            }
            ClassMemberDefinition::JsSetterClassMember(node) => {
                get_static_type(AnyModifierList::JsMethodModifierList(node.modifiers()))
            }
        }
    }

    fn range(&self) -> TextRange {
        match self {
            ClassMemberDefinition::JsGetterClassMember(node) => node.range(),
            ClassMemberDefinition::JsMethodClassMember(node) => node.range(),
            ClassMemberDefinition::JsPropertyClassMember(node) => node.range(),
            ClassMemberDefinition::JsSetterClassMember(node) => node.range(),
        }
    }
}

impl TryFrom<AnyJsClassMember> for ClassMemberDefinition {
    type Error = AnyJsClassMember;

    fn try_from(member: AnyJsClassMember) -> Result<Self, Self::Error> {
        match member {
            AnyJsClassMember::JsGetterClassMember(node) => {
                Ok(ClassMemberDefinition::JsGetterClassMember(node))
            }
            AnyJsClassMember::JsMethodClassMember(node) => {
                Ok(ClassMemberDefinition::JsMethodClassMember(node))
            }
            AnyJsClassMember::JsPropertyClassMember(node) => {
                Ok(ClassMemberDefinition::JsPropertyClassMember(node))
            }
            AnyJsClassMember::JsSetterClassMember(node) => {
                Ok(ClassMemberDefinition::JsSetterClassMember(node))
            }
            _ => Err(member),
        }
    }
}

declare_node_union! {
    pub(crate) ClassDefinition = JsClassExpression | JsClassDeclaration
}

impl ClassDefinition {
    fn members(&self) -> JsClassMemberList {
        match self {
            ClassDefinition::JsClassExpression(node) => node.members(),
            ClassDefinition::JsClassDeclaration(node) => node.members(),
        }
    }
}

impl Rule for NoDuplicateClassMembers {
    type Query = Ast<ClassDefinition>;
    type State = ClassMemberDefinition;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut defined_members: HashMap<
            (String, StaticType, AccessType),
            HashMap<MemberType, ClassMemberDefinition>,
        > = HashMap::new();
        let mut signals: Self::Signals = Vec::new();

        let node = ctx.query();
        for member in node.members() {
            if let Ok(member_def) = ClassMemberDefinition::try_from(member) {
                if let (Some(member_name), Some(access_type)) =
                    (member_def.member_name(), member_def.access_type())
                {
                    let static_type = member_def.static_type();
                    let member_type = member_def.member_type();
                    defined_members
                        .entry((member_name, static_type, access_type))
                        .and_modify(|element| {
                            if element.get(&member_type).is_some() {
                                signals.push(member_def.clone());
                            } else {
                                if member_type != MemberType::Normal
                                    && element.get(&MemberType::Normal).is_some()
                                {
                                    signals.push(member_def.clone());
                                }

                                if member_type == MemberType::Normal
                                    && (element.get(&MemberType::Getter).is_some()
                                        || element.get(&MemberType::Setter).is_some())
                                {
                                    signals.push(member_def.clone());
                                }

                                element.insert(member_type, member_def.clone());
                            }
                        })
                        .or_insert_with(|| HashMap::from([(member_type, member_def)]));
                }
            }
        }

        signals
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.range(),
            format!(
                "Duplicate class member name {:?}",
                state.member_name().unwrap()
            ),
        );

        Some(diagnostic)
    }
}
