use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{AnyJsClassMember, JsClassMemberList};
use rome_rowan::{AstNode, BatchMutationExt};
use rustc_hash::FxHashMap;
use std::iter::Iterator;

use crate::JsRuleAction;

declare_rule! {
    /// Disallow duplicate class members.
    ///
    /// If there are declarations of the same name in class members,
    /// then the last declaration overwrites other declarations silently.
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
    ///   bar;
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
    ///   get bar() { }
    ///   set bar(value) { }
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
    pub(crate) NoDuplicateClassMembers {
        version: "next",
        name: "noDuplicateClassMembers",
        recommended: true,
    }
}

pub(crate) struct DuplicateClassMember {
    overwritten: AnyJsClassMember,
    effective: AnyJsClassMember,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct MemberInfo {
    is_static: bool,
    name: String,
}

impl MemberInfo {
    fn from(member: &AnyJsClassMember) -> Option<MemberInfo> {
        let name = member
            .name()
            .ok()??
            .as_js_literal_member_name()?
            .name()
            .ok()?;
        let is_static = match member {
            AnyJsClassMember::JsConstructorClassMember(_) => false,
            AnyJsClassMember::JsGetterClassMember(x) => x
                .modifiers()
                .into_iter()
                .any(|y| y.as_js_static_modifier().is_some()),
            AnyJsClassMember::JsMethodClassMember(x) => x
                .modifiers()
                .into_iter()
                .any(|y| y.as_js_static_modifier().is_some()),
            AnyJsClassMember::JsPropertyClassMember(x) => x
                .modifiers()
                .into_iter()
                .any(|y| y.as_js_static_modifier().is_some()),
            AnyJsClassMember::JsSetterClassMember(x) => x
                .modifiers()
                .into_iter()
                .any(|y| y.as_js_static_modifier().is_some()),
            AnyJsClassMember::JsBogusMember(_)
            | AnyJsClassMember::JsEmptyClassMember(_)
            | AnyJsClassMember::JsStaticInitializationBlockClassMember(_)
            | AnyJsClassMember::TsConstructorSignatureClassMember(_)
            | AnyJsClassMember::TsGetterSignatureClassMember(_)
            | AnyJsClassMember::TsIndexSignatureClassMember(_)
            | AnyJsClassMember::TsMethodSignatureClassMember(_)
            | AnyJsClassMember::TsPropertySignatureClassMember(_)
            | AnyJsClassMember::TsSetterSignatureClassMember(_) => return None,
        };
        Some(MemberInfo { is_static, name })
    }
}

impl Rule for NoDuplicateClassMembers {
    type Query = Ast<JsClassMemberList>;
    type State = DuplicateClassMember;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let members = ctx.query();
        let mut effective_members = FxHashMap::<MemberInfo, AnyJsClassMember>::default();
        members
            .into_iter()
            .rev()
            .filter_map(|member| {
                let member_info = MemberInfo::from(&member)?;
                if let Some(effective_member) = effective_members.get(&member_info) {
                    let is_paired = (effective_member.as_js_getter_class_member().is_some()
                        && member.as_js_setter_class_member().is_some())
                        || (member.as_js_getter_class_member().is_some()
                            && effective_member.as_js_setter_class_member().is_some());
                    if !is_paired {
                        return Some(DuplicateClassMember {
                            overwritten: member,
                            effective: effective_member.clone(),
                        });
                    }
                } else {
                    effective_members.insert(member_info, member);
                }
                None
            })
            .collect()
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.overwritten.syntax().text_trimmed_range(),
                markup! {
                    "This class member is later overwritten by another class member."
                },
            )
            .detail(
                state.effective.syntax().text_trimmed_range(),
                markup! {
                    "Overwritten with this class member:"
                }
            ).note(
                markup! {
                    "If a class member with the same name is defined multiple times (except when combining a getter with a setter), only the last definition makes it into the class and previous definitions are ignored."
                }
            )
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        mutation.remove_node(state.overwritten.clone());
        Some(JsRuleAction {
            category: rome_analyze::ActionCategory::QuickFix,
            // The property initialization could contain side effects
            applicability: rome_diagnostics::Applicability::MaybeIncorrect,
            message: markup!("Remove the overwritten class member.").to_owned(),
            mutation,
        })
    }
}
