use std::collections::{HashMap, HashSet};

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

fn get_member_name(node: &AnyJsClassMemberName) -> Option<String> {
    match node {
        AnyJsClassMemberName::JsLiteralMemberName(node) => node.name().ok(),
        _ => None,
    }
}

fn is_static_member(node: JsSyntaxList) -> bool {
    node.into_iter().any(|m| {
        if let rome_rowan::SyntaxSlot::Node(node) = m {
            JsStaticModifier::can_cast(node.kind())
        } else {
            false
        }
    })
}

declare_node_union! {
    pub(crate) AnyClassMemberDefinition = JsGetterClassMember | JsMethodClassMember | JsPropertyClassMember | JsSetterClassMember
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MemberType {
    Normal,
    Getter,
    Setter,
}

impl AnyClassMemberDefinition {
    fn name(&self) -> Option<AnyJsClassMemberName> {
        match self {
            AnyClassMemberDefinition::JsGetterClassMember(node) => node.name().ok(),
            AnyClassMemberDefinition::JsMethodClassMember(node) => node.name().ok(),
            AnyClassMemberDefinition::JsPropertyClassMember(node) => node.name().ok(),
            AnyClassMemberDefinition::JsSetterClassMember(node) => node.name().ok(),
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

    fn member_type(&self) -> MemberType {
        match self {
            AnyClassMemberDefinition::JsGetterClassMember(_) => MemberType::Getter,
            AnyClassMemberDefinition::JsMethodClassMember(_) => MemberType::Normal,
            AnyClassMemberDefinition::JsPropertyClassMember(_) => MemberType::Normal,
            AnyClassMemberDefinition::JsSetterClassMember(_) => MemberType::Setter,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct MemberState {
    name: String,
    is_static: bool,
}

impl Rule for NoDuplicateClassMembers {
    type Query = Ast<JsClassMemberList>;
    type State = AnyClassMemberDefinition;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut defined_members: HashMap<MemberState, HashSet<MemberType>> = HashMap::new();

        let node = ctx.query();
        node.into_iter()
            .filter_map(|member| {
                let member_definition = AnyClassMemberDefinition::cast_ref(member.syntax())?;
                let member_name_node = member_definition.name()?;
                let member_state = MemberState {
                    name: get_member_name(&member_name_node)?,
                    is_static: is_static_member(member_definition.modifiers_list()),
                };

                let member_type = member_definition.member_type();
                if let Some(stored_members) = defined_members.get_mut(&member_state) {
                    if stored_members.contains(&MemberType::Normal)
                        || stored_members.contains(&member_type)
                        || member_type == MemberType::Normal
                    {
                        return Some(member_definition);
                    } else {
                        stored_members.insert(member_type);
                    }
                } else {
                    defined_members.insert(member_state, HashSet::from([member_type]));
                }

                None
            })
            .collect()
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.range(),
            format!(
                "Duplicate class member name {:?}",
                get_member_name(&state.name()?)?
            ),
        );

        Some(diagnostic)
    }
}
