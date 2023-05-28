use rome_rowan::{declare_node_union, AstNode};

use crate::{
    AnyJsClassMember, AnyJsObjectBindingPatternMember, AnyJsObjectMember, AnyTsTypeMember,
    JsExportAsClause, JsExportNamedFromSpecifier, JsExportNamedSpecifier, JsLiteralExportName,
    JsLiteralMemberName, JsNamedImportSpecifier, JsObjectAssignmentPatternProperty,
    JsPrivateClassMemberName, TsEnumMember,
};

impl JsPrivateClassMemberName {
    pub fn member(&self) -> Option<AnyJsClassMember> {
        self.syntax().ancestors().find_map(AnyJsClassMember::cast)
    }
}

declare_node_union! {
    pub AnyJsNamedExport = JsNamedImportSpecifier | JsExportNamedSpecifier | JsExportNamedFromSpecifier | JsExportAsClause
}

impl JsLiteralExportName {
    pub fn named_export(&self) -> Option<AnyJsNamedExport> {
        self.syntax().ancestors().find_map(AnyJsNamedExport::cast)
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
