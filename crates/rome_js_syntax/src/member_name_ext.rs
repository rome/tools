use rome_rowan::{declare_node_union, AstNode};

use crate::{
    AnyJsClassMember, AnyJsObjectBindingPatternMember, AnyJsObjectMember, AnyTsTypeMember,
    JsLiteralMemberName, JsObjectAssignmentPatternProperty, JsPrivateClassMemberName, TsEnumMember,
};

impl JsPrivateClassMemberName {
    pub fn member(&self) -> Option<AnyJsClassMember> {
        self.syntax().ancestors().find_map(AnyJsClassMember::cast)
    }
}

declare_node_union! {
    pub AnyJsMember = AnyJsClassMember | AnyJsObjectMember | AnyTsTypeMember |
        JsObjectAssignmentPatternProperty | AnyJsObjectBindingPatternMember | TsEnumMember
}

impl JsLiteralMemberName {
    pub fn member(&self) -> Option<AnyJsMember> {
        self.syntax().ancestors().find_map(AnyJsMember::cast)
    }
}
