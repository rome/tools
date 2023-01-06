use std::collections::HashMap;

use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_js_syntax::{
    AnyJsClassMemberName, JsClassMemberList, JsGetterClassMember, JsMethodClassMember,
    JsPropertyClassMember, JsSetterClassMember, JsStaticModifier, JsSyntaxList, TextRange,
};
use rome_rowan::AstNodeList;
use rome_rowan::{declare_node_union, AstNode};

declare_rule! {
    /// Disallow duplicate class members.
    ///
    /// If there are declarations of the same name among class members,
    /// the last declaration overwrites other declarations silently.
    /// It can cause unexpected behaviours.
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

impl From<JsSyntaxList> for StaticType {
    fn from(node: JsSyntaxList) -> Self {
        if node.into_iter().any(|m| {
            if let rome_rowan::SyntaxSlot::Node(node) = m {
                JsStaticModifier::can_cast(node.kind())
            } else {
                false
            }
        }) {
            StaticType::Static
        } else {
            StaticType::NonStatic
        }
    }
}

declare_node_union! {
    pub(crate) AnyClassMemberDefinition = JsGetterClassMember | JsMethodClassMember | JsPropertyClassMember | JsSetterClassMember
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum MemberType {
    Normal,
    Getter,
    Setter,
}

impl AnyClassMemberDefinition {
    fn member_name(&self) -> Option<String> {
        match self {
            AnyClassMemberDefinition::JsGetterClassMember(node) => {
                get_member_name_string(node.name().ok()?)
            }
            AnyClassMemberDefinition::JsMethodClassMember(node) => {
                get_member_name_string(node.name().ok()?)
            }
            AnyClassMemberDefinition::JsPropertyClassMember(node) => {
                get_member_name_string(node.name().ok()?)
            }
            AnyClassMemberDefinition::JsSetterClassMember(node) => {
                get_member_name_string(node.name().ok()?)
            }
        }
    }

    fn member_type(&self) -> MemberType {
        match self {
            AnyClassMemberDefinition::JsGetterClassMember(_) => MemberType::Getter,
            AnyClassMemberDefinition::JsSetterClassMember(_) => MemberType::Setter,
            _ => MemberType::Normal,
        }
    }

    fn access_type(&self) -> Option<AccessType> {
        match self {
            AnyClassMemberDefinition::JsGetterClassMember(node) => {
                get_access_type(node.name().ok()?)
            }
            AnyClassMemberDefinition::JsMethodClassMember(node) => {
                get_access_type(node.name().ok()?)
            }
            AnyClassMemberDefinition::JsPropertyClassMember(node) => {
                get_access_type(node.name().ok()?)
            }
            AnyClassMemberDefinition::JsSetterClassMember(node) => {
                get_access_type(node.name().ok()?)
            }
        }
    }

    fn modifiers_list(&self) -> JsSyntaxList {
        match self {
            AnyClassMemberDefinition::JsGetterClassMember(node) => {
                node.modifiers().syntax_list().clone()
            }
            AnyClassMemberDefinition::JsMethodClassMember(node) => {
                node.modifiers().syntax_list().clone()
            }
            AnyClassMemberDefinition::JsPropertyClassMember(node) => {
                node.modifiers().syntax_list().clone()
            }
            AnyClassMemberDefinition::JsSetterClassMember(node) => {
                node.modifiers().syntax_list().clone()
            }
        }
    }

    fn range(&self) -> TextRange {
        match self {
            AnyClassMemberDefinition::JsGetterClassMember(node) => node.range(),
            AnyClassMemberDefinition::JsMethodClassMember(node) => node.range(),
            AnyClassMemberDefinition::JsPropertyClassMember(node) => node.range(),
            AnyClassMemberDefinition::JsSetterClassMember(node) => node.range(),
        }
    }
}

impl Rule for NoDuplicateClassMembers {
    type Query = Ast<JsClassMemberList>;
    type State = AnyClassMemberDefinition;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut defined_members: HashMap<
            (String, StaticType, AccessType),
            HashMap<MemberType, AnyClassMemberDefinition>,
        > = HashMap::new();
        let mut signals = Vec::new();

        let node = ctx.query();
        for member in node {
            if let Some(member_def) = AnyClassMemberDefinition::cast_ref(member.syntax()) {
                if let (Some(member_name), Some(access_type)) =
                    (member_def.member_name(), member_def.access_type())
                {
                    let static_type = StaticType::from(member_def.modifiers_list());
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
            format!("Duplicate class member name {:?}", state.member_name()?),
        );

        Some(diagnostic)
    }
}
