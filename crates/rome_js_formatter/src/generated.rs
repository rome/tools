//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{AsFormat, FormatNodeRule, FormatOwnedWithRule, FormatRefWithRule, IntoFormat};
impl<'a> AsFormat<'a> for rome_js_syntax::JsScript {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsScript, FormatNodeRule<rome_js_syntax::JsScript>>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsScript {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsScript, FormatNodeRule<rome_js_syntax::JsScript>>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsModule {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsModule, FormatNodeRule<rome_js_syntax::JsModule>>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsModule {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsModule, FormatNodeRule<rome_js_syntax::JsModule>>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExpressionSnipped {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExpressionSnipped,
        FormatNodeRule<rome_js_syntax::JsExpressionSnipped>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsExpressionSnipped {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExpressionSnipped,
        FormatNodeRule<rome_js_syntax::JsExpressionSnipped>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsDirective {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsDirective,
        FormatNodeRule<rome_js_syntax::JsDirective>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsDirective {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsDirective,
        FormatNodeRule<rome_js_syntax::JsDirective>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsBlockStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsBlockStatement,
        FormatNodeRule<rome_js_syntax::JsBlockStatement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsBlockStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsBlockStatement,
        FormatNodeRule<rome_js_syntax::JsBlockStatement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsBreakStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsBreakStatement,
        FormatNodeRule<rome_js_syntax::JsBreakStatement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsBreakStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsBreakStatement,
        FormatNodeRule<rome_js_syntax::JsBreakStatement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsClassDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsClassDeclaration,
        FormatNodeRule<rome_js_syntax::JsClassDeclaration>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsClassDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsClassDeclaration,
        FormatNodeRule<rome_js_syntax::JsClassDeclaration>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsContinueStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsContinueStatement,
        FormatNodeRule<rome_js_syntax::JsContinueStatement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsContinueStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsContinueStatement,
        FormatNodeRule<rome_js_syntax::JsContinueStatement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsDebuggerStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsDebuggerStatement,
        FormatNodeRule<rome_js_syntax::JsDebuggerStatement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsDebuggerStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsDebuggerStatement,
        FormatNodeRule<rome_js_syntax::JsDebuggerStatement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsDoWhileStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsDoWhileStatement,
        FormatNodeRule<rome_js_syntax::JsDoWhileStatement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsDoWhileStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsDoWhileStatement,
        FormatNodeRule<rome_js_syntax::JsDoWhileStatement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsEmptyStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsEmptyStatement,
        FormatNodeRule<rome_js_syntax::JsEmptyStatement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsEmptyStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsEmptyStatement,
        FormatNodeRule<rome_js_syntax::JsEmptyStatement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExpressionStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExpressionStatement,
        FormatNodeRule<rome_js_syntax::JsExpressionStatement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsExpressionStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExpressionStatement,
        FormatNodeRule<rome_js_syntax::JsExpressionStatement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsForInStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsForInStatement,
        FormatNodeRule<rome_js_syntax::JsForInStatement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsForInStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsForInStatement,
        FormatNodeRule<rome_js_syntax::JsForInStatement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsForOfStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsForOfStatement,
        FormatNodeRule<rome_js_syntax::JsForOfStatement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsForOfStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsForOfStatement,
        FormatNodeRule<rome_js_syntax::JsForOfStatement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsForStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsForStatement,
        FormatNodeRule<rome_js_syntax::JsForStatement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsForStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsForStatement,
        FormatNodeRule<rome_js_syntax::JsForStatement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsIfStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsIfStatement,
        FormatNodeRule<rome_js_syntax::JsIfStatement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsIfStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsIfStatement,
        FormatNodeRule<rome_js_syntax::JsIfStatement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsLabeledStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsLabeledStatement,
        FormatNodeRule<rome_js_syntax::JsLabeledStatement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsLabeledStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsLabeledStatement,
        FormatNodeRule<rome_js_syntax::JsLabeledStatement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsReturnStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsReturnStatement,
        FormatNodeRule<rome_js_syntax::JsReturnStatement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsReturnStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsReturnStatement,
        FormatNodeRule<rome_js_syntax::JsReturnStatement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsSwitchStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsSwitchStatement,
        FormatNodeRule<rome_js_syntax::JsSwitchStatement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsSwitchStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsSwitchStatement,
        FormatNodeRule<rome_js_syntax::JsSwitchStatement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsThrowStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsThrowStatement,
        FormatNodeRule<rome_js_syntax::JsThrowStatement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsThrowStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsThrowStatement,
        FormatNodeRule<rome_js_syntax::JsThrowStatement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsTryFinallyStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsTryFinallyStatement,
        FormatNodeRule<rome_js_syntax::JsTryFinallyStatement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsTryFinallyStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsTryFinallyStatement,
        FormatNodeRule<rome_js_syntax::JsTryFinallyStatement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsTryStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsTryStatement,
        FormatNodeRule<rome_js_syntax::JsTryStatement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsTryStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsTryStatement,
        FormatNodeRule<rome_js_syntax::JsTryStatement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsVariableStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsVariableStatement,
        FormatNodeRule<rome_js_syntax::JsVariableStatement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsVariableStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsVariableStatement,
        FormatNodeRule<rome_js_syntax::JsVariableStatement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsWhileStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsWhileStatement,
        FormatNodeRule<rome_js_syntax::JsWhileStatement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsWhileStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsWhileStatement,
        FormatNodeRule<rome_js_syntax::JsWhileStatement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsWithStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsWithStatement,
        FormatNodeRule<rome_js_syntax::JsWithStatement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsWithStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsWithStatement,
        FormatNodeRule<rome_js_syntax::JsWithStatement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsFunctionDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsFunctionDeclaration,
        FormatNodeRule<rome_js_syntax::JsFunctionDeclaration>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsFunctionDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsFunctionDeclaration,
        FormatNodeRule<rome_js_syntax::JsFunctionDeclaration>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsEnumDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsEnumDeclaration,
        FormatNodeRule<rome_js_syntax::TsEnumDeclaration>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsEnumDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsEnumDeclaration,
        FormatNodeRule<rome_js_syntax::TsEnumDeclaration>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeAliasDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeAliasDeclaration,
        FormatNodeRule<rome_js_syntax::TsTypeAliasDeclaration>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsTypeAliasDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeAliasDeclaration,
        FormatNodeRule<rome_js_syntax::TsTypeAliasDeclaration>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsInterfaceDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsInterfaceDeclaration,
        FormatNodeRule<rome_js_syntax::TsInterfaceDeclaration>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsInterfaceDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsInterfaceDeclaration,
        FormatNodeRule<rome_js_syntax::TsInterfaceDeclaration>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsDeclareFunctionDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsDeclareFunctionDeclaration,
        FormatNodeRule<rome_js_syntax::TsDeclareFunctionDeclaration>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsDeclareFunctionDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsDeclareFunctionDeclaration,
        FormatNodeRule<rome_js_syntax::TsDeclareFunctionDeclaration>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsDeclareStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsDeclareStatement,
        FormatNodeRule<rome_js_syntax::TsDeclareStatement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsDeclareStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsDeclareStatement,
        FormatNodeRule<rome_js_syntax::TsDeclareStatement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsModuleDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsModuleDeclaration,
        FormatNodeRule<rome_js_syntax::TsModuleDeclaration>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsModuleDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsModuleDeclaration,
        FormatNodeRule<rome_js_syntax::TsModuleDeclaration>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsExternalModuleDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsExternalModuleDeclaration,
        FormatNodeRule<rome_js_syntax::TsExternalModuleDeclaration>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsExternalModuleDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsExternalModuleDeclaration,
        FormatNodeRule<rome_js_syntax::TsExternalModuleDeclaration>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsGlobalDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsGlobalDeclaration,
        FormatNodeRule<rome_js_syntax::TsGlobalDeclaration>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsGlobalDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsGlobalDeclaration,
        FormatNodeRule<rome_js_syntax::TsGlobalDeclaration>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsImportEqualsDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsImportEqualsDeclaration,
        FormatNodeRule<rome_js_syntax::TsImportEqualsDeclaration>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsImportEqualsDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsImportEqualsDeclaration,
        FormatNodeRule<rome_js_syntax::TsImportEqualsDeclaration>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsElseClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsElseClause,
        FormatNodeRule<rome_js_syntax::JsElseClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsElseClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsElseClause,
        FormatNodeRule<rome_js_syntax::JsElseClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsVariableDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsVariableDeclaration,
        FormatNodeRule<rome_js_syntax::JsVariableDeclaration>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsVariableDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsVariableDeclaration,
        FormatNodeRule<rome_js_syntax::JsVariableDeclaration>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsForVariableDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsForVariableDeclaration,
        FormatNodeRule<rome_js_syntax::JsForVariableDeclaration>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsForVariableDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsForVariableDeclaration,
        FormatNodeRule<rome_js_syntax::JsForVariableDeclaration>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsVariableDeclarator {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsVariableDeclarator,
        FormatNodeRule<rome_js_syntax::JsVariableDeclarator>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsVariableDeclarator {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsVariableDeclarator,
        FormatNodeRule<rome_js_syntax::JsVariableDeclarator>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsCaseClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsCaseClause,
        FormatNodeRule<rome_js_syntax::JsCaseClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsCaseClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsCaseClause,
        FormatNodeRule<rome_js_syntax::JsCaseClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsDefaultClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsDefaultClause,
        FormatNodeRule<rome_js_syntax::JsDefaultClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsDefaultClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsDefaultClause,
        FormatNodeRule<rome_js_syntax::JsDefaultClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsCatchClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsCatchClause,
        FormatNodeRule<rome_js_syntax::JsCatchClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsCatchClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsCatchClause,
        FormatNodeRule<rome_js_syntax::JsCatchClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsFinallyClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsFinallyClause,
        FormatNodeRule<rome_js_syntax::JsFinallyClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsFinallyClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsFinallyClause,
        FormatNodeRule<rome_js_syntax::JsFinallyClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsCatchDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsCatchDeclaration,
        FormatNodeRule<rome_js_syntax::JsCatchDeclaration>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsCatchDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsCatchDeclaration,
        FormatNodeRule<rome_js_syntax::JsCatchDeclaration>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeAnnotation {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeAnnotation,
        FormatNodeRule<rome_js_syntax::TsTypeAnnotation>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsTypeAnnotation {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeAnnotation,
        FormatNodeRule<rome_js_syntax::TsTypeAnnotation>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::ImportMeta {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::ImportMeta,
        FormatNodeRule<rome_js_syntax::ImportMeta>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::ImportMeta {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::ImportMeta, FormatNodeRule<rome_js_syntax::ImportMeta>>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsArrayExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsArrayExpression,
        FormatNodeRule<rome_js_syntax::JsArrayExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsArrayExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsArrayExpression,
        FormatNodeRule<rome_js_syntax::JsArrayExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsArrowFunctionExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsArrowFunctionExpression,
        FormatNodeRule<rome_js_syntax::JsArrowFunctionExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsArrowFunctionExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsArrowFunctionExpression,
        FormatNodeRule<rome_js_syntax::JsArrowFunctionExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAssignmentExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAssignmentExpression,
        FormatNodeRule<rome_js_syntax::JsAssignmentExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAssignmentExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAssignmentExpression,
        FormatNodeRule<rome_js_syntax::JsAssignmentExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAwaitExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAwaitExpression,
        FormatNodeRule<rome_js_syntax::JsAwaitExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAwaitExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAwaitExpression,
        FormatNodeRule<rome_js_syntax::JsAwaitExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsBinaryExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsBinaryExpression,
        FormatNodeRule<rome_js_syntax::JsBinaryExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsBinaryExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsBinaryExpression,
        FormatNodeRule<rome_js_syntax::JsBinaryExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsCallExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsCallExpression,
        FormatNodeRule<rome_js_syntax::JsCallExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsCallExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsCallExpression,
        FormatNodeRule<rome_js_syntax::JsCallExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsClassExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsClassExpression,
        FormatNodeRule<rome_js_syntax::JsClassExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsClassExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsClassExpression,
        FormatNodeRule<rome_js_syntax::JsClassExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsComputedMemberExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsComputedMemberExpression,
        FormatNodeRule<rome_js_syntax::JsComputedMemberExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsComputedMemberExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsComputedMemberExpression,
        FormatNodeRule<rome_js_syntax::JsComputedMemberExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsConditionalExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsConditionalExpression,
        FormatNodeRule<rome_js_syntax::JsConditionalExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsConditionalExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsConditionalExpression,
        FormatNodeRule<rome_js_syntax::JsConditionalExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsFunctionExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsFunctionExpression,
        FormatNodeRule<rome_js_syntax::JsFunctionExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsFunctionExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsFunctionExpression,
        FormatNodeRule<rome_js_syntax::JsFunctionExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsIdentifierExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsIdentifierExpression,
        FormatNodeRule<rome_js_syntax::JsIdentifierExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsIdentifierExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsIdentifierExpression,
        FormatNodeRule<rome_js_syntax::JsIdentifierExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsImportCallExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsImportCallExpression,
        FormatNodeRule<rome_js_syntax::JsImportCallExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsImportCallExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsImportCallExpression,
        FormatNodeRule<rome_js_syntax::JsImportCallExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsInExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsInExpression,
        FormatNodeRule<rome_js_syntax::JsInExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsInExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsInExpression,
        FormatNodeRule<rome_js_syntax::JsInExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsInstanceofExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsInstanceofExpression,
        FormatNodeRule<rome_js_syntax::JsInstanceofExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsInstanceofExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsInstanceofExpression,
        FormatNodeRule<rome_js_syntax::JsInstanceofExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsLogicalExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsLogicalExpression,
        FormatNodeRule<rome_js_syntax::JsLogicalExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsLogicalExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsLogicalExpression,
        FormatNodeRule<rome_js_syntax::JsLogicalExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsNewExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsNewExpression,
        FormatNodeRule<rome_js_syntax::JsNewExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsNewExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsNewExpression,
        FormatNodeRule<rome_js_syntax::JsNewExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsObjectExpression,
        FormatNodeRule<rome_js_syntax::JsObjectExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsObjectExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsObjectExpression,
        FormatNodeRule<rome_js_syntax::JsObjectExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsParenthesizedExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsParenthesizedExpression,
        FormatNodeRule<rome_js_syntax::JsParenthesizedExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsParenthesizedExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsParenthesizedExpression,
        FormatNodeRule<rome_js_syntax::JsParenthesizedExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsPostUpdateExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsPostUpdateExpression,
        FormatNodeRule<rome_js_syntax::JsPostUpdateExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsPostUpdateExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsPostUpdateExpression,
        FormatNodeRule<rome_js_syntax::JsPostUpdateExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsPreUpdateExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsPreUpdateExpression,
        FormatNodeRule<rome_js_syntax::JsPreUpdateExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsPreUpdateExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsPreUpdateExpression,
        FormatNodeRule<rome_js_syntax::JsPreUpdateExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsSequenceExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsSequenceExpression,
        FormatNodeRule<rome_js_syntax::JsSequenceExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsSequenceExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsSequenceExpression,
        FormatNodeRule<rome_js_syntax::JsSequenceExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsStaticMemberExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsStaticMemberExpression,
        FormatNodeRule<rome_js_syntax::JsStaticMemberExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsStaticMemberExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsStaticMemberExpression,
        FormatNodeRule<rome_js_syntax::JsStaticMemberExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsSuperExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsSuperExpression,
        FormatNodeRule<rome_js_syntax::JsSuperExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsSuperExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsSuperExpression,
        FormatNodeRule<rome_js_syntax::JsSuperExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsThisExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsThisExpression,
        FormatNodeRule<rome_js_syntax::JsThisExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsThisExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsThisExpression,
        FormatNodeRule<rome_js_syntax::JsThisExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsUnaryExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsUnaryExpression,
        FormatNodeRule<rome_js_syntax::JsUnaryExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsUnaryExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsUnaryExpression,
        FormatNodeRule<rome_js_syntax::JsUnaryExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsYieldExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsYieldExpression,
        FormatNodeRule<rome_js_syntax::JsYieldExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsYieldExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsYieldExpression,
        FormatNodeRule<rome_js_syntax::JsYieldExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::NewTarget {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::NewTarget, FormatNodeRule<rome_js_syntax::NewTarget>>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::NewTarget {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::NewTarget, FormatNodeRule<rome_js_syntax::NewTarget>>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsTemplate {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsTemplate,
        FormatNodeRule<rome_js_syntax::JsTemplate>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsTemplate {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsTemplate, FormatNodeRule<rome_js_syntax::JsTemplate>>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeAssertionExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeAssertionExpression,
        FormatNodeRule<rome_js_syntax::TsTypeAssertionExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsTypeAssertionExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeAssertionExpression,
        FormatNodeRule<rome_js_syntax::TsTypeAssertionExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAsExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAsExpression,
        FormatNodeRule<rome_js_syntax::TsAsExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsAsExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAsExpression,
        FormatNodeRule<rome_js_syntax::TsAsExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsNonNullAssertionExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsNonNullAssertionExpression,
        FormatNodeRule<rome_js_syntax::TsNonNullAssertionExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsNonNullAssertionExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsNonNullAssertionExpression,
        FormatNodeRule<rome_js_syntax::TsNonNullAssertionExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxTagExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxTagExpression,
        FormatNodeRule<rome_js_syntax::JsxTagExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxTagExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxTagExpression,
        FormatNodeRule<rome_js_syntax::JsxTagExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeArguments {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeArguments,
        FormatNodeRule<rome_js_syntax::TsTypeArguments>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsTypeArguments {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeArguments,
        FormatNodeRule<rome_js_syntax::TsTypeArguments>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsTemplateChunkElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsTemplateChunkElement,
        FormatNodeRule<rome_js_syntax::JsTemplateChunkElement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsTemplateChunkElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsTemplateChunkElement,
        FormatNodeRule<rome_js_syntax::JsTemplateChunkElement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsTemplateElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsTemplateElement,
        FormatNodeRule<rome_js_syntax::JsTemplateElement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsTemplateElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsTemplateElement,
        FormatNodeRule<rome_js_syntax::JsTemplateElement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsCallArguments {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsCallArguments,
        FormatNodeRule<rome_js_syntax::JsCallArguments>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsCallArguments {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsCallArguments,
        FormatNodeRule<rome_js_syntax::JsCallArguments>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsYieldArgument {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsYieldArgument,
        FormatNodeRule<rome_js_syntax::JsYieldArgument>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsYieldArgument {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsYieldArgument,
        FormatNodeRule<rome_js_syntax::JsYieldArgument>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeParameters {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeParameters,
        FormatNodeRule<rome_js_syntax::TsTypeParameters>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsTypeParameters {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeParameters,
        FormatNodeRule<rome_js_syntax::TsTypeParameters>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsParameters {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsParameters,
        FormatNodeRule<rome_js_syntax::JsParameters>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsParameters {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsParameters,
        FormatNodeRule<rome_js_syntax::JsParameters>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsReturnTypeAnnotation {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsReturnTypeAnnotation,
        FormatNodeRule<rome_js_syntax::TsReturnTypeAnnotation>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsReturnTypeAnnotation {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsReturnTypeAnnotation,
        FormatNodeRule<rome_js_syntax::TsReturnTypeAnnotation>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsFunctionBody {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsFunctionBody,
        FormatNodeRule<rome_js_syntax::JsFunctionBody>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsFunctionBody {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsFunctionBody,
        FormatNodeRule<rome_js_syntax::JsFunctionBody>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsSpread {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsSpread, FormatNodeRule<rome_js_syntax::JsSpread>>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsSpread {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsSpread, FormatNodeRule<rome_js_syntax::JsSpread>>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsArrayHole {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsArrayHole,
        FormatNodeRule<rome_js_syntax::JsArrayHole>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsArrayHole {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsArrayHole,
        FormatNodeRule<rome_js_syntax::JsArrayHole>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsReferenceIdentifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsReferenceIdentifier,
        FormatNodeRule<rome_js_syntax::JsReferenceIdentifier>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsReferenceIdentifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsReferenceIdentifier,
        FormatNodeRule<rome_js_syntax::JsReferenceIdentifier>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsPrivateName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsPrivateName,
        FormatNodeRule<rome_js_syntax::JsPrivateName>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsPrivateName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsPrivateName,
        FormatNodeRule<rome_js_syntax::JsPrivateName>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsLiteralMemberName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsLiteralMemberName,
        FormatNodeRule<rome_js_syntax::JsLiteralMemberName>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsLiteralMemberName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsLiteralMemberName,
        FormatNodeRule<rome_js_syntax::JsLiteralMemberName>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsComputedMemberName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsComputedMemberName,
        FormatNodeRule<rome_js_syntax::JsComputedMemberName>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsComputedMemberName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsComputedMemberName,
        FormatNodeRule<rome_js_syntax::JsComputedMemberName>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsPropertyObjectMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsPropertyObjectMember,
        FormatNodeRule<rome_js_syntax::JsPropertyObjectMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsPropertyObjectMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsPropertyObjectMember,
        FormatNodeRule<rome_js_syntax::JsPropertyObjectMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsMethodObjectMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsMethodObjectMember,
        FormatNodeRule<rome_js_syntax::JsMethodObjectMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsMethodObjectMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsMethodObjectMember,
        FormatNodeRule<rome_js_syntax::JsMethodObjectMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsGetterObjectMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsGetterObjectMember,
        FormatNodeRule<rome_js_syntax::JsGetterObjectMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsGetterObjectMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsGetterObjectMember,
        FormatNodeRule<rome_js_syntax::JsGetterObjectMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsSetterObjectMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsSetterObjectMember,
        FormatNodeRule<rome_js_syntax::JsSetterObjectMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsSetterObjectMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsSetterObjectMember,
        FormatNodeRule<rome_js_syntax::JsSetterObjectMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsShorthandPropertyObjectMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsShorthandPropertyObjectMember,
        FormatNodeRule<rome_js_syntax::JsShorthandPropertyObjectMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsShorthandPropertyObjectMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsShorthandPropertyObjectMember,
        FormatNodeRule<rome_js_syntax::JsShorthandPropertyObjectMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExtendsClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExtendsClause,
        FormatNodeRule<rome_js_syntax::JsExtendsClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsExtendsClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExtendsClause,
        FormatNodeRule<rome_js_syntax::JsExtendsClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsImplementsClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsImplementsClause,
        FormatNodeRule<rome_js_syntax::TsImplementsClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsImplementsClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsImplementsClause,
        FormatNodeRule<rome_js_syntax::TsImplementsClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsClassExportDefaultDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsClassExportDefaultDeclaration,
        FormatNodeRule<rome_js_syntax::JsClassExportDefaultDeclaration>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsClassExportDefaultDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsClassExportDefaultDeclaration,
        FormatNodeRule<rome_js_syntax::JsClassExportDefaultDeclaration>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsPrivateClassMemberName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsPrivateClassMemberName,
        FormatNodeRule<rome_js_syntax::JsPrivateClassMemberName>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsPrivateClassMemberName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsPrivateClassMemberName,
        FormatNodeRule<rome_js_syntax::JsPrivateClassMemberName>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsConstructorClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsConstructorClassMember,
        FormatNodeRule<rome_js_syntax::JsConstructorClassMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsConstructorClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsConstructorClassMember,
        FormatNodeRule<rome_js_syntax::JsConstructorClassMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsStaticInitializationBlockClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsStaticInitializationBlockClassMember,
        FormatNodeRule<rome_js_syntax::JsStaticInitializationBlockClassMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsStaticInitializationBlockClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsStaticInitializationBlockClassMember,
        FormatNodeRule<rome_js_syntax::JsStaticInitializationBlockClassMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsPropertyClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsPropertyClassMember,
        FormatNodeRule<rome_js_syntax::JsPropertyClassMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsPropertyClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsPropertyClassMember,
        FormatNodeRule<rome_js_syntax::JsPropertyClassMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsMethodClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsMethodClassMember,
        FormatNodeRule<rome_js_syntax::JsMethodClassMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsMethodClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsMethodClassMember,
        FormatNodeRule<rome_js_syntax::JsMethodClassMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsGetterClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsGetterClassMember,
        FormatNodeRule<rome_js_syntax::JsGetterClassMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsGetterClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsGetterClassMember,
        FormatNodeRule<rome_js_syntax::JsGetterClassMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsSetterClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsSetterClassMember,
        FormatNodeRule<rome_js_syntax::JsSetterClassMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsSetterClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsSetterClassMember,
        FormatNodeRule<rome_js_syntax::JsSetterClassMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsConstructorSignatureClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsConstructorSignatureClassMember,
        FormatNodeRule<rome_js_syntax::TsConstructorSignatureClassMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsConstructorSignatureClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsConstructorSignatureClassMember,
        FormatNodeRule<rome_js_syntax::TsConstructorSignatureClassMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsPropertySignatureClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsPropertySignatureClassMember,
        FormatNodeRule<rome_js_syntax::TsPropertySignatureClassMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsPropertySignatureClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsPropertySignatureClassMember,
        FormatNodeRule<rome_js_syntax::TsPropertySignatureClassMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsMethodSignatureClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsMethodSignatureClassMember,
        FormatNodeRule<rome_js_syntax::TsMethodSignatureClassMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsMethodSignatureClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsMethodSignatureClassMember,
        FormatNodeRule<rome_js_syntax::TsMethodSignatureClassMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsGetterSignatureClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsGetterSignatureClassMember,
        FormatNodeRule<rome_js_syntax::TsGetterSignatureClassMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsGetterSignatureClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsGetterSignatureClassMember,
        FormatNodeRule<rome_js_syntax::TsGetterSignatureClassMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsSetterSignatureClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsSetterSignatureClassMember,
        FormatNodeRule<rome_js_syntax::TsSetterSignatureClassMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsSetterSignatureClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsSetterSignatureClassMember,
        FormatNodeRule<rome_js_syntax::TsSetterSignatureClassMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsIndexSignatureClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsIndexSignatureClassMember,
        FormatNodeRule<rome_js_syntax::TsIndexSignatureClassMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsIndexSignatureClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsIndexSignatureClassMember,
        FormatNodeRule<rome_js_syntax::TsIndexSignatureClassMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsEmptyClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsEmptyClassMember,
        FormatNodeRule<rome_js_syntax::JsEmptyClassMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsEmptyClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsEmptyClassMember,
        FormatNodeRule<rome_js_syntax::JsEmptyClassMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsStaticModifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsStaticModifier,
        FormatNodeRule<rome_js_syntax::JsStaticModifier>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsStaticModifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsStaticModifier,
        FormatNodeRule<rome_js_syntax::JsStaticModifier>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsDeclareModifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsDeclareModifier,
        FormatNodeRule<rome_js_syntax::TsDeclareModifier>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsDeclareModifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsDeclareModifier,
        FormatNodeRule<rome_js_syntax::TsDeclareModifier>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsReadonlyModifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsReadonlyModifier,
        FormatNodeRule<rome_js_syntax::TsReadonlyModifier>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsReadonlyModifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsReadonlyModifier,
        FormatNodeRule<rome_js_syntax::TsReadonlyModifier>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAbstractModifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAbstractModifier,
        FormatNodeRule<rome_js_syntax::TsAbstractModifier>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsAbstractModifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAbstractModifier,
        FormatNodeRule<rome_js_syntax::TsAbstractModifier>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsOverrideModifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsOverrideModifier,
        FormatNodeRule<rome_js_syntax::TsOverrideModifier>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsOverrideModifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsOverrideModifier,
        FormatNodeRule<rome_js_syntax::TsOverrideModifier>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAccessibilityModifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAccessibilityModifier,
        FormatNodeRule<rome_js_syntax::TsAccessibilityModifier>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsAccessibilityModifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAccessibilityModifier,
        FormatNodeRule<rome_js_syntax::TsAccessibilityModifier>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsConstructorParameters {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsConstructorParameters,
        FormatNodeRule<rome_js_syntax::JsConstructorParameters>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsConstructorParameters {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsConstructorParameters,
        FormatNodeRule<rome_js_syntax::JsConstructorParameters>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsRestParameter {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsRestParameter,
        FormatNodeRule<rome_js_syntax::JsRestParameter>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsRestParameter {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsRestParameter,
        FormatNodeRule<rome_js_syntax::JsRestParameter>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsPropertyParameter {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsPropertyParameter,
        FormatNodeRule<rome_js_syntax::TsPropertyParameter>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsPropertyParameter {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsPropertyParameter,
        FormatNodeRule<rome_js_syntax::TsPropertyParameter>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsInitializerClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsInitializerClause,
        FormatNodeRule<rome_js_syntax::JsInitializerClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsInitializerClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsInitializerClause,
        FormatNodeRule<rome_js_syntax::JsInitializerClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsOptionalPropertyAnnotation {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsOptionalPropertyAnnotation,
        FormatNodeRule<rome_js_syntax::TsOptionalPropertyAnnotation>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsOptionalPropertyAnnotation {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsOptionalPropertyAnnotation,
        FormatNodeRule<rome_js_syntax::TsOptionalPropertyAnnotation>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsDefinitePropertyAnnotation {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsDefinitePropertyAnnotation,
        FormatNodeRule<rome_js_syntax::TsDefinitePropertyAnnotation>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsDefinitePropertyAnnotation {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsDefinitePropertyAnnotation,
        FormatNodeRule<rome_js_syntax::TsDefinitePropertyAnnotation>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsIndexSignatureParameter {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsIndexSignatureParameter,
        FormatNodeRule<rome_js_syntax::TsIndexSignatureParameter>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsIndexSignatureParameter {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsIndexSignatureParameter,
        FormatNodeRule<rome_js_syntax::TsIndexSignatureParameter>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsIdentifierAssignment {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsIdentifierAssignment,
        FormatNodeRule<rome_js_syntax::JsIdentifierAssignment>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsIdentifierAssignment {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsIdentifierAssignment,
        FormatNodeRule<rome_js_syntax::JsIdentifierAssignment>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsStaticMemberAssignment {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsStaticMemberAssignment,
        FormatNodeRule<rome_js_syntax::JsStaticMemberAssignment>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsStaticMemberAssignment {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsStaticMemberAssignment,
        FormatNodeRule<rome_js_syntax::JsStaticMemberAssignment>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsComputedMemberAssignment {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsComputedMemberAssignment,
        FormatNodeRule<rome_js_syntax::JsComputedMemberAssignment>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsComputedMemberAssignment {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsComputedMemberAssignment,
        FormatNodeRule<rome_js_syntax::JsComputedMemberAssignment>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsParenthesizedAssignment {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsParenthesizedAssignment,
        FormatNodeRule<rome_js_syntax::JsParenthesizedAssignment>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsParenthesizedAssignment {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsParenthesizedAssignment,
        FormatNodeRule<rome_js_syntax::JsParenthesizedAssignment>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsNonNullAssertionAssignment {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsNonNullAssertionAssignment,
        FormatNodeRule<rome_js_syntax::TsNonNullAssertionAssignment>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsNonNullAssertionAssignment {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsNonNullAssertionAssignment,
        FormatNodeRule<rome_js_syntax::TsNonNullAssertionAssignment>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAsAssignment {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAsAssignment,
        FormatNodeRule<rome_js_syntax::TsAsAssignment>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsAsAssignment {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAsAssignment,
        FormatNodeRule<rome_js_syntax::TsAsAssignment>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeAssertionAssignment {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeAssertionAssignment,
        FormatNodeRule<rome_js_syntax::TsTypeAssertionAssignment>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsTypeAssertionAssignment {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeAssertionAssignment,
        FormatNodeRule<rome_js_syntax::TsTypeAssertionAssignment>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAssignmentWithDefault {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAssignmentWithDefault,
        FormatNodeRule<rome_js_syntax::JsAssignmentWithDefault>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAssignmentWithDefault {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAssignmentWithDefault,
        FormatNodeRule<rome_js_syntax::JsAssignmentWithDefault>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsArrayAssignmentPattern {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsArrayAssignmentPattern,
        FormatNodeRule<rome_js_syntax::JsArrayAssignmentPattern>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsArrayAssignmentPattern {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsArrayAssignmentPattern,
        FormatNodeRule<rome_js_syntax::JsArrayAssignmentPattern>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectAssignmentPattern {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsObjectAssignmentPattern,
        FormatNodeRule<rome_js_syntax::JsObjectAssignmentPattern>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsObjectAssignmentPattern {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsObjectAssignmentPattern,
        FormatNodeRule<rome_js_syntax::JsObjectAssignmentPattern>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsArrayAssignmentPatternRestElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsArrayAssignmentPatternRestElement,
        FormatNodeRule<rome_js_syntax::JsArrayAssignmentPatternRestElement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsArrayAssignmentPatternRestElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsArrayAssignmentPatternRestElement,
        FormatNodeRule<rome_js_syntax::JsArrayAssignmentPatternRestElement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectAssignmentPatternShorthandProperty {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsObjectAssignmentPatternShorthandProperty,
        FormatNodeRule<rome_js_syntax::JsObjectAssignmentPatternShorthandProperty>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsObjectAssignmentPatternShorthandProperty {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsObjectAssignmentPatternShorthandProperty,
        FormatNodeRule<rome_js_syntax::JsObjectAssignmentPatternShorthandProperty>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectAssignmentPatternProperty {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsObjectAssignmentPatternProperty,
        FormatNodeRule<rome_js_syntax::JsObjectAssignmentPatternProperty>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsObjectAssignmentPatternProperty {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsObjectAssignmentPatternProperty,
        FormatNodeRule<rome_js_syntax::JsObjectAssignmentPatternProperty>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectAssignmentPatternRest {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsObjectAssignmentPatternRest,
        FormatNodeRule<rome_js_syntax::JsObjectAssignmentPatternRest>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsObjectAssignmentPatternRest {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsObjectAssignmentPatternRest,
        FormatNodeRule<rome_js_syntax::JsObjectAssignmentPatternRest>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsIdentifierBinding {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsIdentifierBinding,
        FormatNodeRule<rome_js_syntax::JsIdentifierBinding>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsIdentifierBinding {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsIdentifierBinding,
        FormatNodeRule<rome_js_syntax::JsIdentifierBinding>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsBindingPatternWithDefault {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsBindingPatternWithDefault,
        FormatNodeRule<rome_js_syntax::JsBindingPatternWithDefault>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsBindingPatternWithDefault {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsBindingPatternWithDefault,
        FormatNodeRule<rome_js_syntax::JsBindingPatternWithDefault>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsArrayBindingPattern {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsArrayBindingPattern,
        FormatNodeRule<rome_js_syntax::JsArrayBindingPattern>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsArrayBindingPattern {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsArrayBindingPattern,
        FormatNodeRule<rome_js_syntax::JsArrayBindingPattern>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectBindingPattern {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsObjectBindingPattern,
        FormatNodeRule<rome_js_syntax::JsObjectBindingPattern>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsObjectBindingPattern {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsObjectBindingPattern,
        FormatNodeRule<rome_js_syntax::JsObjectBindingPattern>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsArrayBindingPatternRestElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsArrayBindingPatternRestElement,
        FormatNodeRule<rome_js_syntax::JsArrayBindingPatternRestElement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsArrayBindingPatternRestElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsArrayBindingPatternRestElement,
        FormatNodeRule<rome_js_syntax::JsArrayBindingPatternRestElement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectBindingPatternProperty {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsObjectBindingPatternProperty,
        FormatNodeRule<rome_js_syntax::JsObjectBindingPatternProperty>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsObjectBindingPatternProperty {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsObjectBindingPatternProperty,
        FormatNodeRule<rome_js_syntax::JsObjectBindingPatternProperty>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectBindingPatternRest {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsObjectBindingPatternRest,
        FormatNodeRule<rome_js_syntax::JsObjectBindingPatternRest>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsObjectBindingPatternRest {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsObjectBindingPatternRest,
        FormatNodeRule<rome_js_syntax::JsObjectBindingPatternRest>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectBindingPatternShorthandProperty {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsObjectBindingPatternShorthandProperty,
        FormatNodeRule<rome_js_syntax::JsObjectBindingPatternShorthandProperty>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsObjectBindingPatternShorthandProperty {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsObjectBindingPatternShorthandProperty,
        FormatNodeRule<rome_js_syntax::JsObjectBindingPatternShorthandProperty>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsStringLiteralExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsStringLiteralExpression,
        FormatNodeRule<rome_js_syntax::JsStringLiteralExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsStringLiteralExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsStringLiteralExpression,
        FormatNodeRule<rome_js_syntax::JsStringLiteralExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsNumberLiteralExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsNumberLiteralExpression,
        FormatNodeRule<rome_js_syntax::JsNumberLiteralExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsNumberLiteralExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsNumberLiteralExpression,
        FormatNodeRule<rome_js_syntax::JsNumberLiteralExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsBigIntLiteralExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsBigIntLiteralExpression,
        FormatNodeRule<rome_js_syntax::JsBigIntLiteralExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsBigIntLiteralExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsBigIntLiteralExpression,
        FormatNodeRule<rome_js_syntax::JsBigIntLiteralExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsBooleanLiteralExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsBooleanLiteralExpression,
        FormatNodeRule<rome_js_syntax::JsBooleanLiteralExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsBooleanLiteralExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsBooleanLiteralExpression,
        FormatNodeRule<rome_js_syntax::JsBooleanLiteralExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsNullLiteralExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsNullLiteralExpression,
        FormatNodeRule<rome_js_syntax::JsNullLiteralExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsNullLiteralExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsNullLiteralExpression,
        FormatNodeRule<rome_js_syntax::JsNullLiteralExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsRegexLiteralExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsRegexLiteralExpression,
        FormatNodeRule<rome_js_syntax::JsRegexLiteralExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsRegexLiteralExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsRegexLiteralExpression,
        FormatNodeRule<rome_js_syntax::JsRegexLiteralExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsVariableDeclarationClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsVariableDeclarationClause,
        FormatNodeRule<rome_js_syntax::JsVariableDeclarationClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsVariableDeclarationClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsVariableDeclarationClause,
        FormatNodeRule<rome_js_syntax::JsVariableDeclarationClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsDefiniteVariableAnnotation {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsDefiniteVariableAnnotation,
        FormatNodeRule<rome_js_syntax::TsDefiniteVariableAnnotation>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsDefiniteVariableAnnotation {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsDefiniteVariableAnnotation,
        FormatNodeRule<rome_js_syntax::TsDefiniteVariableAnnotation>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExport {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsExport, FormatNodeRule<rome_js_syntax::JsExport>>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsExport {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsExport, FormatNodeRule<rome_js_syntax::JsExport>>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsImport {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsImport, FormatNodeRule<rome_js_syntax::JsImport>>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsImport {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsImport, FormatNodeRule<rome_js_syntax::JsImport>>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsImportBareClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsImportBareClause,
        FormatNodeRule<rome_js_syntax::JsImportBareClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsImportBareClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsImportBareClause,
        FormatNodeRule<rome_js_syntax::JsImportBareClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsImportNamedClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsImportNamedClause,
        FormatNodeRule<rome_js_syntax::JsImportNamedClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsImportNamedClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsImportNamedClause,
        FormatNodeRule<rome_js_syntax::JsImportNamedClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsImportDefaultClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsImportDefaultClause,
        FormatNodeRule<rome_js_syntax::JsImportDefaultClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsImportDefaultClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsImportDefaultClause,
        FormatNodeRule<rome_js_syntax::JsImportDefaultClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsImportNamespaceClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsImportNamespaceClause,
        FormatNodeRule<rome_js_syntax::JsImportNamespaceClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsImportNamespaceClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsImportNamespaceClause,
        FormatNodeRule<rome_js_syntax::JsImportNamespaceClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsModuleSource {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsModuleSource,
        FormatNodeRule<rome_js_syntax::JsModuleSource>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsModuleSource {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsModuleSource,
        FormatNodeRule<rome_js_syntax::JsModuleSource>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsImportAssertion {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsImportAssertion,
        FormatNodeRule<rome_js_syntax::JsImportAssertion>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsImportAssertion {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsImportAssertion,
        FormatNodeRule<rome_js_syntax::JsImportAssertion>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsDefaultImportSpecifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsDefaultImportSpecifier,
        FormatNodeRule<rome_js_syntax::JsDefaultImportSpecifier>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsDefaultImportSpecifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsDefaultImportSpecifier,
        FormatNodeRule<rome_js_syntax::JsDefaultImportSpecifier>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsNamedImportSpecifiers {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsNamedImportSpecifiers,
        FormatNodeRule<rome_js_syntax::JsNamedImportSpecifiers>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsNamedImportSpecifiers {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsNamedImportSpecifiers,
        FormatNodeRule<rome_js_syntax::JsNamedImportSpecifiers>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsNamespaceImportSpecifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsNamespaceImportSpecifier,
        FormatNodeRule<rome_js_syntax::JsNamespaceImportSpecifier>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsNamespaceImportSpecifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsNamespaceImportSpecifier,
        FormatNodeRule<rome_js_syntax::JsNamespaceImportSpecifier>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsShorthandNamedImportSpecifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsShorthandNamedImportSpecifier,
        FormatNodeRule<rome_js_syntax::JsShorthandNamedImportSpecifier>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsShorthandNamedImportSpecifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsShorthandNamedImportSpecifier,
        FormatNodeRule<rome_js_syntax::JsShorthandNamedImportSpecifier>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsNamedImportSpecifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsNamedImportSpecifier,
        FormatNodeRule<rome_js_syntax::JsNamedImportSpecifier>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsNamedImportSpecifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsNamedImportSpecifier,
        FormatNodeRule<rome_js_syntax::JsNamedImportSpecifier>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsLiteralExportName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsLiteralExportName,
        FormatNodeRule<rome_js_syntax::JsLiteralExportName>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsLiteralExportName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsLiteralExportName,
        FormatNodeRule<rome_js_syntax::JsLiteralExportName>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsImportAssertionEntry {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsImportAssertionEntry,
        FormatNodeRule<rome_js_syntax::JsImportAssertionEntry>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsImportAssertionEntry {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsImportAssertionEntry,
        FormatNodeRule<rome_js_syntax::JsImportAssertionEntry>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExportDefaultDeclarationClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExportDefaultDeclarationClause,
        FormatNodeRule<rome_js_syntax::JsExportDefaultDeclarationClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsExportDefaultDeclarationClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExportDefaultDeclarationClause,
        FormatNodeRule<rome_js_syntax::JsExportDefaultDeclarationClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExportDefaultExpressionClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExportDefaultExpressionClause,
        FormatNodeRule<rome_js_syntax::JsExportDefaultExpressionClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsExportDefaultExpressionClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExportDefaultExpressionClause,
        FormatNodeRule<rome_js_syntax::JsExportDefaultExpressionClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExportNamedClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExportNamedClause,
        FormatNodeRule<rome_js_syntax::JsExportNamedClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsExportNamedClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExportNamedClause,
        FormatNodeRule<rome_js_syntax::JsExportNamedClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExportFromClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExportFromClause,
        FormatNodeRule<rome_js_syntax::JsExportFromClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsExportFromClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExportFromClause,
        FormatNodeRule<rome_js_syntax::JsExportFromClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExportNamedFromClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExportNamedFromClause,
        FormatNodeRule<rome_js_syntax::JsExportNamedFromClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsExportNamedFromClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExportNamedFromClause,
        FormatNodeRule<rome_js_syntax::JsExportNamedFromClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsExportAsNamespaceClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsExportAsNamespaceClause,
        FormatNodeRule<rome_js_syntax::TsExportAsNamespaceClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsExportAsNamespaceClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsExportAsNamespaceClause,
        FormatNodeRule<rome_js_syntax::TsExportAsNamespaceClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsExportAssignmentClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsExportAssignmentClause,
        FormatNodeRule<rome_js_syntax::TsExportAssignmentClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsExportAssignmentClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsExportAssignmentClause,
        FormatNodeRule<rome_js_syntax::TsExportAssignmentClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsExportDeclareClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsExportDeclareClause,
        FormatNodeRule<rome_js_syntax::TsExportDeclareClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsExportDeclareClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsExportDeclareClause,
        FormatNodeRule<rome_js_syntax::TsExportDeclareClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsFunctionExportDefaultDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsFunctionExportDefaultDeclaration,
        FormatNodeRule<rome_js_syntax::JsFunctionExportDefaultDeclaration>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsFunctionExportDefaultDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsFunctionExportDefaultDeclaration,
        FormatNodeRule<rome_js_syntax::JsFunctionExportDefaultDeclaration>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExportNamedShorthandSpecifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExportNamedShorthandSpecifier,
        FormatNodeRule<rome_js_syntax::JsExportNamedShorthandSpecifier>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsExportNamedShorthandSpecifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExportNamedShorthandSpecifier,
        FormatNodeRule<rome_js_syntax::JsExportNamedShorthandSpecifier>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExportNamedSpecifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExportNamedSpecifier,
        FormatNodeRule<rome_js_syntax::JsExportNamedSpecifier>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsExportNamedSpecifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExportNamedSpecifier,
        FormatNodeRule<rome_js_syntax::JsExportNamedSpecifier>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExportAsClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExportAsClause,
        FormatNodeRule<rome_js_syntax::JsExportAsClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsExportAsClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExportAsClause,
        FormatNodeRule<rome_js_syntax::JsExportAsClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExportNamedFromSpecifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExportNamedFromSpecifier,
        FormatNodeRule<rome_js_syntax::JsExportNamedFromSpecifier>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsExportNamedFromSpecifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExportNamedFromSpecifier,
        FormatNodeRule<rome_js_syntax::JsExportNamedFromSpecifier>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsName {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsName, FormatNodeRule<rome_js_syntax::JsName>>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsName {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsName, FormatNodeRule<rome_js_syntax::JsName>>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsFormalParameter {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsFormalParameter,
        FormatNodeRule<rome_js_syntax::JsFormalParameter>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsFormalParameter {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsFormalParameter,
        FormatNodeRule<rome_js_syntax::JsFormalParameter>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsThisParameter {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsThisParameter,
        FormatNodeRule<rome_js_syntax::TsThisParameter>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsThisParameter {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsThisParameter,
        FormatNodeRule<rome_js_syntax::TsThisParameter>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyType {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::TsAnyType, FormatNodeRule<rome_js_syntax::TsAnyType>>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsAnyType {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::TsAnyType, FormatNodeRule<rome_js_syntax::TsAnyType>>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsUnknownType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsUnknownType,
        FormatNodeRule<rome_js_syntax::TsUnknownType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsUnknownType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsUnknownType,
        FormatNodeRule<rome_js_syntax::TsUnknownType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsNumberType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsNumberType,
        FormatNodeRule<rome_js_syntax::TsNumberType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsNumberType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsNumberType,
        FormatNodeRule<rome_js_syntax::TsNumberType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsBooleanType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsBooleanType,
        FormatNodeRule<rome_js_syntax::TsBooleanType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsBooleanType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsBooleanType,
        FormatNodeRule<rome_js_syntax::TsBooleanType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsBigintType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsBigintType,
        FormatNodeRule<rome_js_syntax::TsBigintType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsBigintType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsBigintType,
        FormatNodeRule<rome_js_syntax::TsBigintType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsStringType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsStringType,
        FormatNodeRule<rome_js_syntax::TsStringType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsStringType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsStringType,
        FormatNodeRule<rome_js_syntax::TsStringType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsSymbolType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsSymbolType,
        FormatNodeRule<rome_js_syntax::TsSymbolType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsSymbolType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsSymbolType,
        FormatNodeRule<rome_js_syntax::TsSymbolType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsVoidType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsVoidType,
        FormatNodeRule<rome_js_syntax::TsVoidType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsVoidType {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::TsVoidType, FormatNodeRule<rome_js_syntax::TsVoidType>>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsUndefinedType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsUndefinedType,
        FormatNodeRule<rome_js_syntax::TsUndefinedType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsUndefinedType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsUndefinedType,
        FormatNodeRule<rome_js_syntax::TsUndefinedType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsNeverType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsNeverType,
        FormatNodeRule<rome_js_syntax::TsNeverType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsNeverType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsNeverType,
        FormatNodeRule<rome_js_syntax::TsNeverType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsParenthesizedType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsParenthesizedType,
        FormatNodeRule<rome_js_syntax::TsParenthesizedType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsParenthesizedType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsParenthesizedType,
        FormatNodeRule<rome_js_syntax::TsParenthesizedType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsReferenceType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsReferenceType,
        FormatNodeRule<rome_js_syntax::TsReferenceType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsReferenceType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsReferenceType,
        FormatNodeRule<rome_js_syntax::TsReferenceType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsArrayType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsArrayType,
        FormatNodeRule<rome_js_syntax::TsArrayType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsArrayType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsArrayType,
        FormatNodeRule<rome_js_syntax::TsArrayType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTupleType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTupleType,
        FormatNodeRule<rome_js_syntax::TsTupleType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsTupleType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTupleType,
        FormatNodeRule<rome_js_syntax::TsTupleType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeofType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeofType,
        FormatNodeRule<rome_js_syntax::TsTypeofType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsTypeofType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeofType,
        FormatNodeRule<rome_js_syntax::TsTypeofType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsImportType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsImportType,
        FormatNodeRule<rome_js_syntax::TsImportType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsImportType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsImportType,
        FormatNodeRule<rome_js_syntax::TsImportType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeOperatorType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeOperatorType,
        FormatNodeRule<rome_js_syntax::TsTypeOperatorType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsTypeOperatorType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeOperatorType,
        FormatNodeRule<rome_js_syntax::TsTypeOperatorType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsIndexedAccessType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsIndexedAccessType,
        FormatNodeRule<rome_js_syntax::TsIndexedAccessType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsIndexedAccessType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsIndexedAccessType,
        FormatNodeRule<rome_js_syntax::TsIndexedAccessType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsMappedType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsMappedType,
        FormatNodeRule<rome_js_syntax::TsMappedType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsMappedType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsMappedType,
        FormatNodeRule<rome_js_syntax::TsMappedType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsObjectType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsObjectType,
        FormatNodeRule<rome_js_syntax::TsObjectType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsObjectType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsObjectType,
        FormatNodeRule<rome_js_syntax::TsObjectType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsNonPrimitiveType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsNonPrimitiveType,
        FormatNodeRule<rome_js_syntax::TsNonPrimitiveType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsNonPrimitiveType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsNonPrimitiveType,
        FormatNodeRule<rome_js_syntax::TsNonPrimitiveType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsThisType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsThisType,
        FormatNodeRule<rome_js_syntax::TsThisType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsThisType {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::TsThisType, FormatNodeRule<rome_js_syntax::TsThisType>>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsNumberLiteralType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsNumberLiteralType,
        FormatNodeRule<rome_js_syntax::TsNumberLiteralType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsNumberLiteralType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsNumberLiteralType,
        FormatNodeRule<rome_js_syntax::TsNumberLiteralType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsBigIntLiteralType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsBigIntLiteralType,
        FormatNodeRule<rome_js_syntax::TsBigIntLiteralType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsBigIntLiteralType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsBigIntLiteralType,
        FormatNodeRule<rome_js_syntax::TsBigIntLiteralType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsStringLiteralType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsStringLiteralType,
        FormatNodeRule<rome_js_syntax::TsStringLiteralType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsStringLiteralType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsStringLiteralType,
        FormatNodeRule<rome_js_syntax::TsStringLiteralType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsNullLiteralType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsNullLiteralType,
        FormatNodeRule<rome_js_syntax::TsNullLiteralType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsNullLiteralType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsNullLiteralType,
        FormatNodeRule<rome_js_syntax::TsNullLiteralType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsBooleanLiteralType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsBooleanLiteralType,
        FormatNodeRule<rome_js_syntax::TsBooleanLiteralType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsBooleanLiteralType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsBooleanLiteralType,
        FormatNodeRule<rome_js_syntax::TsBooleanLiteralType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTemplateLiteralType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTemplateLiteralType,
        FormatNodeRule<rome_js_syntax::TsTemplateLiteralType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsTemplateLiteralType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTemplateLiteralType,
        FormatNodeRule<rome_js_syntax::TsTemplateLiteralType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsInferType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsInferType,
        FormatNodeRule<rome_js_syntax::TsInferType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsInferType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsInferType,
        FormatNodeRule<rome_js_syntax::TsInferType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsIntersectionType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsIntersectionType,
        FormatNodeRule<rome_js_syntax::TsIntersectionType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsIntersectionType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsIntersectionType,
        FormatNodeRule<rome_js_syntax::TsIntersectionType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsUnionType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsUnionType,
        FormatNodeRule<rome_js_syntax::TsUnionType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsUnionType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsUnionType,
        FormatNodeRule<rome_js_syntax::TsUnionType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsFunctionType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsFunctionType,
        FormatNodeRule<rome_js_syntax::TsFunctionType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsFunctionType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsFunctionType,
        FormatNodeRule<rome_js_syntax::TsFunctionType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsConstructorType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsConstructorType,
        FormatNodeRule<rome_js_syntax::TsConstructorType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsConstructorType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsConstructorType,
        FormatNodeRule<rome_js_syntax::TsConstructorType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsConditionalType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsConditionalType,
        FormatNodeRule<rome_js_syntax::TsConditionalType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsConditionalType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsConditionalType,
        FormatNodeRule<rome_js_syntax::TsConditionalType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsIdentifierBinding {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsIdentifierBinding,
        FormatNodeRule<rome_js_syntax::TsIdentifierBinding>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsIdentifierBinding {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsIdentifierBinding,
        FormatNodeRule<rome_js_syntax::TsIdentifierBinding>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsEnumMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsEnumMember,
        FormatNodeRule<rome_js_syntax::TsEnumMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsEnumMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsEnumMember,
        FormatNodeRule<rome_js_syntax::TsEnumMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsExternalModuleReference {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsExternalModuleReference,
        FormatNodeRule<rome_js_syntax::TsExternalModuleReference>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsExternalModuleReference {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsExternalModuleReference,
        FormatNodeRule<rome_js_syntax::TsExternalModuleReference>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsModuleBlock {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsModuleBlock,
        FormatNodeRule<rome_js_syntax::TsModuleBlock>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsModuleBlock {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsModuleBlock,
        FormatNodeRule<rome_js_syntax::TsModuleBlock>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsQualifiedModuleName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsQualifiedModuleName,
        FormatNodeRule<rome_js_syntax::TsQualifiedModuleName>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsQualifiedModuleName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsQualifiedModuleName,
        FormatNodeRule<rome_js_syntax::TsQualifiedModuleName>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsEmptyExternalModuleDeclarationBody {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsEmptyExternalModuleDeclarationBody,
        FormatNodeRule<rome_js_syntax::TsEmptyExternalModuleDeclarationBody>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsEmptyExternalModuleDeclarationBody {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsEmptyExternalModuleDeclarationBody,
        FormatNodeRule<rome_js_syntax::TsEmptyExternalModuleDeclarationBody>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeParameterName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeParameterName,
        FormatNodeRule<rome_js_syntax::TsTypeParameterName>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsTypeParameterName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeParameterName,
        FormatNodeRule<rome_js_syntax::TsTypeParameterName>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsPredicateReturnType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsPredicateReturnType,
        FormatNodeRule<rome_js_syntax::TsPredicateReturnType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsPredicateReturnType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsPredicateReturnType,
        FormatNodeRule<rome_js_syntax::TsPredicateReturnType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAssertsReturnType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAssertsReturnType,
        FormatNodeRule<rome_js_syntax::TsAssertsReturnType>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsAssertsReturnType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAssertsReturnType,
        FormatNodeRule<rome_js_syntax::TsAssertsReturnType>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAssertsCondition {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAssertsCondition,
        FormatNodeRule<rome_js_syntax::TsAssertsCondition>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsAssertsCondition {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAssertsCondition,
        FormatNodeRule<rome_js_syntax::TsAssertsCondition>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeParameter {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeParameter,
        FormatNodeRule<rome_js_syntax::TsTypeParameter>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsTypeParameter {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeParameter,
        FormatNodeRule<rome_js_syntax::TsTypeParameter>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeConstraintClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeConstraintClause,
        FormatNodeRule<rome_js_syntax::TsTypeConstraintClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsTypeConstraintClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeConstraintClause,
        FormatNodeRule<rome_js_syntax::TsTypeConstraintClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsDefaultTypeClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsDefaultTypeClause,
        FormatNodeRule<rome_js_syntax::TsDefaultTypeClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsDefaultTypeClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsDefaultTypeClause,
        FormatNodeRule<rome_js_syntax::TsDefaultTypeClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsExtendsClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsExtendsClause,
        FormatNodeRule<rome_js_syntax::TsExtendsClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsExtendsClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsExtendsClause,
        FormatNodeRule<rome_js_syntax::TsExtendsClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsNameWithTypeArguments {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsNameWithTypeArguments,
        FormatNodeRule<rome_js_syntax::TsNameWithTypeArguments>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsNameWithTypeArguments {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsNameWithTypeArguments,
        FormatNodeRule<rome_js_syntax::TsNameWithTypeArguments>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsCallSignatureTypeMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsCallSignatureTypeMember,
        FormatNodeRule<rome_js_syntax::TsCallSignatureTypeMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsCallSignatureTypeMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsCallSignatureTypeMember,
        FormatNodeRule<rome_js_syntax::TsCallSignatureTypeMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsPropertySignatureTypeMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsPropertySignatureTypeMember,
        FormatNodeRule<rome_js_syntax::TsPropertySignatureTypeMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsPropertySignatureTypeMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsPropertySignatureTypeMember,
        FormatNodeRule<rome_js_syntax::TsPropertySignatureTypeMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsConstructSignatureTypeMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsConstructSignatureTypeMember,
        FormatNodeRule<rome_js_syntax::TsConstructSignatureTypeMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsConstructSignatureTypeMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsConstructSignatureTypeMember,
        FormatNodeRule<rome_js_syntax::TsConstructSignatureTypeMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsMethodSignatureTypeMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsMethodSignatureTypeMember,
        FormatNodeRule<rome_js_syntax::TsMethodSignatureTypeMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsMethodSignatureTypeMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsMethodSignatureTypeMember,
        FormatNodeRule<rome_js_syntax::TsMethodSignatureTypeMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsGetterSignatureTypeMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsGetterSignatureTypeMember,
        FormatNodeRule<rome_js_syntax::TsGetterSignatureTypeMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsGetterSignatureTypeMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsGetterSignatureTypeMember,
        FormatNodeRule<rome_js_syntax::TsGetterSignatureTypeMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsSetterSignatureTypeMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsSetterSignatureTypeMember,
        FormatNodeRule<rome_js_syntax::TsSetterSignatureTypeMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsSetterSignatureTypeMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsSetterSignatureTypeMember,
        FormatNodeRule<rome_js_syntax::TsSetterSignatureTypeMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsIndexSignatureTypeMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsIndexSignatureTypeMember,
        FormatNodeRule<rome_js_syntax::TsIndexSignatureTypeMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsIndexSignatureTypeMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsIndexSignatureTypeMember,
        FormatNodeRule<rome_js_syntax::TsIndexSignatureTypeMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsMappedTypeReadonlyModifierClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsMappedTypeReadonlyModifierClause,
        FormatNodeRule<rome_js_syntax::TsMappedTypeReadonlyModifierClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsMappedTypeReadonlyModifierClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsMappedTypeReadonlyModifierClause,
        FormatNodeRule<rome_js_syntax::TsMappedTypeReadonlyModifierClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsMappedTypeAsClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsMappedTypeAsClause,
        FormatNodeRule<rome_js_syntax::TsMappedTypeAsClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsMappedTypeAsClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsMappedTypeAsClause,
        FormatNodeRule<rome_js_syntax::TsMappedTypeAsClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsMappedTypeOptionalModifierClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsMappedTypeOptionalModifierClause,
        FormatNodeRule<rome_js_syntax::TsMappedTypeOptionalModifierClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsMappedTypeOptionalModifierClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsMappedTypeOptionalModifierClause,
        FormatNodeRule<rome_js_syntax::TsMappedTypeOptionalModifierClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsImportTypeQualifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsImportTypeQualifier,
        FormatNodeRule<rome_js_syntax::TsImportTypeQualifier>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsImportTypeQualifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsImportTypeQualifier,
        FormatNodeRule<rome_js_syntax::TsImportTypeQualifier>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsNamedTupleTypeElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsNamedTupleTypeElement,
        FormatNodeRule<rome_js_syntax::TsNamedTupleTypeElement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsNamedTupleTypeElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsNamedTupleTypeElement,
        FormatNodeRule<rome_js_syntax::TsNamedTupleTypeElement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsRestTupleTypeElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsRestTupleTypeElement,
        FormatNodeRule<rome_js_syntax::TsRestTupleTypeElement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsRestTupleTypeElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsRestTupleTypeElement,
        FormatNodeRule<rome_js_syntax::TsRestTupleTypeElement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsOptionalTupleTypeElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsOptionalTupleTypeElement,
        FormatNodeRule<rome_js_syntax::TsOptionalTupleTypeElement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsOptionalTupleTypeElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsOptionalTupleTypeElement,
        FormatNodeRule<rome_js_syntax::TsOptionalTupleTypeElement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTemplateChunkElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTemplateChunkElement,
        FormatNodeRule<rome_js_syntax::TsTemplateChunkElement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsTemplateChunkElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTemplateChunkElement,
        FormatNodeRule<rome_js_syntax::TsTemplateChunkElement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTemplateElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTemplateElement,
        FormatNodeRule<rome_js_syntax::TsTemplateElement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsTemplateElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTemplateElement,
        FormatNodeRule<rome_js_syntax::TsTemplateElement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsQualifiedName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsQualifiedName,
        FormatNodeRule<rome_js_syntax::TsQualifiedName>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsQualifiedName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsQualifiedName,
        FormatNodeRule<rome_js_syntax::TsQualifiedName>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxElement,
        FormatNodeRule<rome_js_syntax::JsxElement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxElement {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsxElement, FormatNodeRule<rome_js_syntax::JsxElement>>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxSelfClosingElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxSelfClosingElement,
        FormatNodeRule<rome_js_syntax::JsxSelfClosingElement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxSelfClosingElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxSelfClosingElement,
        FormatNodeRule<rome_js_syntax::JsxSelfClosingElement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxFragment {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxFragment,
        FormatNodeRule<rome_js_syntax::JsxFragment>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxFragment {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxFragment,
        FormatNodeRule<rome_js_syntax::JsxFragment>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxOpeningElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxOpeningElement,
        FormatNodeRule<rome_js_syntax::JsxOpeningElement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxOpeningElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxOpeningElement,
        FormatNodeRule<rome_js_syntax::JsxOpeningElement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxClosingElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxClosingElement,
        FormatNodeRule<rome_js_syntax::JsxClosingElement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxClosingElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxClosingElement,
        FormatNodeRule<rome_js_syntax::JsxClosingElement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxOpeningFragment {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxOpeningFragment,
        FormatNodeRule<rome_js_syntax::JsxOpeningFragment>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxOpeningFragment {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxOpeningFragment,
        FormatNodeRule<rome_js_syntax::JsxOpeningFragment>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxClosingFragment {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxClosingFragment,
        FormatNodeRule<rome_js_syntax::JsxClosingFragment>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxClosingFragment {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxClosingFragment,
        FormatNodeRule<rome_js_syntax::JsxClosingFragment>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxName {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsxName, FormatNodeRule<rome_js_syntax::JsxName>>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxName {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsxName, FormatNodeRule<rome_js_syntax::JsxName>>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxReferenceIdentifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxReferenceIdentifier,
        FormatNodeRule<rome_js_syntax::JsxReferenceIdentifier>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxReferenceIdentifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxReferenceIdentifier,
        FormatNodeRule<rome_js_syntax::JsxReferenceIdentifier>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxNamespaceName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxNamespaceName,
        FormatNodeRule<rome_js_syntax::JsxNamespaceName>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxNamespaceName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxNamespaceName,
        FormatNodeRule<rome_js_syntax::JsxNamespaceName>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxMemberName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxMemberName,
        FormatNodeRule<rome_js_syntax::JsxMemberName>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxMemberName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxMemberName,
        FormatNodeRule<rome_js_syntax::JsxMemberName>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxAttribute {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxAttribute,
        FormatNodeRule<rome_js_syntax::JsxAttribute>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxAttribute {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxAttribute,
        FormatNodeRule<rome_js_syntax::JsxAttribute>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxSpreadAttribute {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxSpreadAttribute,
        FormatNodeRule<rome_js_syntax::JsxSpreadAttribute>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxSpreadAttribute {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxSpreadAttribute,
        FormatNodeRule<rome_js_syntax::JsxSpreadAttribute>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxAttributeInitializerClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxAttributeInitializerClause,
        FormatNodeRule<rome_js_syntax::JsxAttributeInitializerClause>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxAttributeInitializerClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxAttributeInitializerClause,
        FormatNodeRule<rome_js_syntax::JsxAttributeInitializerClause>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxString {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsxString, FormatNodeRule<rome_js_syntax::JsxString>>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxString {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsxString, FormatNodeRule<rome_js_syntax::JsxString>>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxExpressionAttributeValue {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxExpressionAttributeValue,
        FormatNodeRule<rome_js_syntax::JsxExpressionAttributeValue>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxExpressionAttributeValue {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxExpressionAttributeValue,
        FormatNodeRule<rome_js_syntax::JsxExpressionAttributeValue>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxText {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsxText, FormatNodeRule<rome_js_syntax::JsxText>>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxText {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsxText, FormatNodeRule<rome_js_syntax::JsxText>>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxExpressionChild {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxExpressionChild,
        FormatNodeRule<rome_js_syntax::JsxExpressionChild>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxExpressionChild {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxExpressionChild,
        FormatNodeRule<rome_js_syntax::JsxExpressionChild>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxSpreadChild {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxSpreadChild,
        FormatNodeRule<rome_js_syntax::JsxSpreadChild>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxSpreadChild {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxSpreadChild,
        FormatNodeRule<rome_js_syntax::JsxSpreadChild>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsArrayAssignmentPatternElementList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsArrayAssignmentPatternElementList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsArrayAssignmentPatternElementList,
        FormatJsArrayAssignmentPatternElementList,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsArrayAssignmentPatternElementList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsArrayAssignmentPatternElementList,
        FormatJsArrayAssignmentPatternElementList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsArrayBindingPatternElementList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsArrayBindingPatternElementList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsArrayBindingPatternElementList,
        FormatJsArrayBindingPatternElementList,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsArrayBindingPatternElementList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsArrayBindingPatternElementList,
        FormatJsArrayBindingPatternElementList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsArrayElementList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsArrayElementList {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsArrayElementList, FormatJsArrayElementList>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsArrayElementList {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsArrayElementList, FormatJsArrayElementList>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsCallArgumentList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsCallArgumentList {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsCallArgumentList, FormatJsCallArgumentList>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsCallArgumentList {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsCallArgumentList, FormatJsCallArgumentList>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsClassMemberList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsClassMemberList {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsClassMemberList, FormatJsClassMemberList>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsClassMemberList {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsClassMemberList, FormatJsClassMemberList>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsConstructorModifierList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsConstructorModifierList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsConstructorModifierList,
        FormatJsConstructorModifierList,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsConstructorModifierList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsConstructorModifierList,
        FormatJsConstructorModifierList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsConstructorParameterList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsConstructorParameterList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsConstructorParameterList,
        FormatJsConstructorParameterList,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsConstructorParameterList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsConstructorParameterList,
        FormatJsConstructorParameterList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsDirectiveList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsDirectiveList {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsDirectiveList, FormatJsDirectiveList>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsDirectiveList {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsDirectiveList, FormatJsDirectiveList>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsExportNamedFromSpecifierList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsExportNamedFromSpecifierList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExportNamedFromSpecifierList,
        FormatJsExportNamedFromSpecifierList,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsExportNamedFromSpecifierList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExportNamedFromSpecifierList,
        FormatJsExportNamedFromSpecifierList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsExportNamedSpecifierList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsExportNamedSpecifierList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExportNamedSpecifierList,
        FormatJsExportNamedSpecifierList,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsExportNamedSpecifierList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExportNamedSpecifierList,
        FormatJsExportNamedSpecifierList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsImportAssertionEntryList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsImportAssertionEntryList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsImportAssertionEntryList,
        FormatJsImportAssertionEntryList,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsImportAssertionEntryList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsImportAssertionEntryList,
        FormatJsImportAssertionEntryList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsMethodModifierList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsMethodModifierList {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsMethodModifierList, FormatJsMethodModifierList>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsMethodModifierList {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsMethodModifierList, FormatJsMethodModifierList>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsModuleItemList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsModuleItemList {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsModuleItemList, FormatJsModuleItemList>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsModuleItemList {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsModuleItemList, FormatJsModuleItemList>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsNamedImportSpecifierList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsNamedImportSpecifierList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsNamedImportSpecifierList,
        FormatJsNamedImportSpecifierList,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsNamedImportSpecifierList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsNamedImportSpecifierList,
        FormatJsNamedImportSpecifierList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsObjectAssignmentPatternPropertyList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectAssignmentPatternPropertyList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsObjectAssignmentPatternPropertyList,
        FormatJsObjectAssignmentPatternPropertyList,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsObjectAssignmentPatternPropertyList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsObjectAssignmentPatternPropertyList,
        FormatJsObjectAssignmentPatternPropertyList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsObjectBindingPatternPropertyList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectBindingPatternPropertyList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsObjectBindingPatternPropertyList,
        FormatJsObjectBindingPatternPropertyList,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsObjectBindingPatternPropertyList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsObjectBindingPatternPropertyList,
        FormatJsObjectBindingPatternPropertyList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsObjectMemberList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectMemberList {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsObjectMemberList, FormatJsObjectMemberList>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsObjectMemberList {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsObjectMemberList, FormatJsObjectMemberList>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsParameterList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsParameterList {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsParameterList, FormatJsParameterList>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsParameterList {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsParameterList, FormatJsParameterList>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsPropertyModifierList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsPropertyModifierList {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsPropertyModifierList, FormatJsPropertyModifierList>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsPropertyModifierList {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsPropertyModifierList, FormatJsPropertyModifierList>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsStatementList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsStatementList {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsStatementList, FormatJsStatementList>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsStatementList {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsStatementList, FormatJsStatementList>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsSwitchCaseList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsSwitchCaseList {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsSwitchCaseList, FormatJsSwitchCaseList>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsSwitchCaseList {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsSwitchCaseList, FormatJsSwitchCaseList>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsTemplateElementList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsTemplateElementList {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsTemplateElementList, FormatJsTemplateElementList>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsTemplateElementList {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsTemplateElementList, FormatJsTemplateElementList>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsVariableDeclaratorList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsVariableDeclaratorList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsVariableDeclaratorList,
        FormatJsVariableDeclaratorList,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsVariableDeclaratorList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsVariableDeclaratorList,
        FormatJsVariableDeclaratorList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsxAttributeList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsxAttributeList {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsxAttributeList, FormatJsxAttributeList>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxAttributeList {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsxAttributeList, FormatJsxAttributeList>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsxChildList;
impl<'a> AsFormat<'a> for rome_js_syntax::JsxChildList {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsxChildList, FormatJsxChildList>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxChildList {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsxChildList, FormatJsxChildList>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsEnumMemberList;
impl<'a> AsFormat<'a> for rome_js_syntax::TsEnumMemberList {
    type Format = FormatRefWithRule<'a, rome_js_syntax::TsEnumMemberList, FormatTsEnumMemberList>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsEnumMemberList {
    type Format = FormatOwnedWithRule<rome_js_syntax::TsEnumMemberList, FormatTsEnumMemberList>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsIndexSignatureModifierList;
impl<'a> AsFormat<'a> for rome_js_syntax::TsIndexSignatureModifierList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsIndexSignatureModifierList,
        FormatTsIndexSignatureModifierList,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsIndexSignatureModifierList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsIndexSignatureModifierList,
        FormatTsIndexSignatureModifierList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsIntersectionTypeElementList;
impl<'a> AsFormat<'a> for rome_js_syntax::TsIntersectionTypeElementList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsIntersectionTypeElementList,
        FormatTsIntersectionTypeElementList,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsIntersectionTypeElementList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsIntersectionTypeElementList,
        FormatTsIntersectionTypeElementList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsMethodSignatureModifierList;
impl<'a> AsFormat<'a> for rome_js_syntax::TsMethodSignatureModifierList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsMethodSignatureModifierList,
        FormatTsMethodSignatureModifierList,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsMethodSignatureModifierList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsMethodSignatureModifierList,
        FormatTsMethodSignatureModifierList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsPropertyParameterModifierList;
impl<'a> AsFormat<'a> for rome_js_syntax::TsPropertyParameterModifierList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsPropertyParameterModifierList,
        FormatTsPropertyParameterModifierList,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsPropertyParameterModifierList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsPropertyParameterModifierList,
        FormatTsPropertyParameterModifierList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsPropertySignatureModifierList;
impl<'a> AsFormat<'a> for rome_js_syntax::TsPropertySignatureModifierList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsPropertySignatureModifierList,
        FormatTsPropertySignatureModifierList,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsPropertySignatureModifierList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsPropertySignatureModifierList,
        FormatTsPropertySignatureModifierList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsTemplateElementList;
impl<'a> AsFormat<'a> for rome_js_syntax::TsTemplateElementList {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::TsTemplateElementList, FormatTsTemplateElementList>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsTemplateElementList {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::TsTemplateElementList, FormatTsTemplateElementList>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsTupleTypeElementList;
impl<'a> AsFormat<'a> for rome_js_syntax::TsTupleTypeElementList {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::TsTupleTypeElementList, FormatTsTupleTypeElementList>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsTupleTypeElementList {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::TsTupleTypeElementList, FormatTsTupleTypeElementList>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsTypeArgumentList;
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeArgumentList {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::TsTypeArgumentList, FormatTsTypeArgumentList>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsTypeArgumentList {
    type Format = FormatOwnedWithRule<rome_js_syntax::TsTypeArgumentList, FormatTsTypeArgumentList>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsTypeList;
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeList {
    type Format = FormatRefWithRule<'a, rome_js_syntax::TsTypeList, FormatTsTypeList>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsTypeList {
    type Format = FormatOwnedWithRule<rome_js_syntax::TsTypeList, FormatTsTypeList>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsTypeMemberList;
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeMemberList {
    type Format = FormatRefWithRule<'a, rome_js_syntax::TsTypeMemberList, FormatTsTypeMemberList>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsTypeMemberList {
    type Format = FormatOwnedWithRule<rome_js_syntax::TsTypeMemberList, FormatTsTypeMemberList>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsTypeParameterList;
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeParameterList {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::TsTypeParameterList, FormatTsTypeParameterList>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsTypeParameterList {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::TsTypeParameterList, FormatTsTypeParameterList>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsUnionTypeVariantList;
impl<'a> AsFormat<'a> for rome_js_syntax::TsUnionTypeVariantList {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::TsUnionTypeVariantList, FormatTsUnionTypeVariantList>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsUnionTypeVariantList {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::TsUnionTypeVariantList, FormatTsUnionTypeVariantList>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsUnknown {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsUnknown, FormatNodeRule<rome_js_syntax::JsUnknown>>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsUnknown {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsUnknown, FormatNodeRule<rome_js_syntax::JsUnknown>>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsUnknownStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsUnknownStatement,
        FormatNodeRule<rome_js_syntax::JsUnknownStatement>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsUnknownStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsUnknownStatement,
        FormatNodeRule<rome_js_syntax::JsUnknownStatement>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsUnknownExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsUnknownExpression,
        FormatNodeRule<rome_js_syntax::JsUnknownExpression>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsUnknownExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsUnknownExpression,
        FormatNodeRule<rome_js_syntax::JsUnknownExpression>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsUnknownMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsUnknownMember,
        FormatNodeRule<rome_js_syntax::JsUnknownMember>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsUnknownMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsUnknownMember,
        FormatNodeRule<rome_js_syntax::JsUnknownMember>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsUnknownBinding {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsUnknownBinding,
        FormatNodeRule<rome_js_syntax::JsUnknownBinding>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsUnknownBinding {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsUnknownBinding,
        FormatNodeRule<rome_js_syntax::JsUnknownBinding>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsUnknownAssignment {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsUnknownAssignment,
        FormatNodeRule<rome_js_syntax::JsUnknownAssignment>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsUnknownAssignment {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsUnknownAssignment,
        FormatNodeRule<rome_js_syntax::JsUnknownAssignment>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsUnknownParameter {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsUnknownParameter,
        FormatNodeRule<rome_js_syntax::JsUnknownParameter>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsUnknownParameter {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsUnknownParameter,
        FormatNodeRule<rome_js_syntax::JsUnknownParameter>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsUnknownImportAssertionEntry {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsUnknownImportAssertionEntry,
        FormatNodeRule<rome_js_syntax::JsUnknownImportAssertionEntry>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsUnknownImportAssertionEntry {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsUnknownImportAssertionEntry,
        FormatNodeRule<rome_js_syntax::JsUnknownImportAssertionEntry>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsUnknownNamedImportSpecifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsUnknownNamedImportSpecifier,
        FormatNodeRule<rome_js_syntax::JsUnknownNamedImportSpecifier>,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsUnknownNamedImportSpecifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsUnknownNamedImportSpecifier,
        FormatNodeRule<rome_js_syntax::JsUnknownNamedImportSpecifier>,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyRoot;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyRoot {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsAnyRoot, FormatJsAnyRoot>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyRoot {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsAnyRoot, FormatJsAnyRoot>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyExpression;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyExpression {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsAnyExpression, FormatJsAnyExpression>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyExpression {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsAnyExpression, FormatJsAnyExpression>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyStatement;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyStatement {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsAnyStatement, FormatJsAnyStatement>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyStatement {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsAnyStatement, FormatJsAnyStatement>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyForInitializer;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyForInitializer {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsAnyForInitializer, FormatJsAnyForInitializer>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyForInitializer {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsAnyForInitializer, FormatJsAnyForInitializer>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyForInOrOfInitializer;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyForInOrOfInitializer {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyForInOrOfInitializer,
        FormatJsAnyForInOrOfInitializer,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyForInOrOfInitializer {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyForInOrOfInitializer,
        FormatJsAnyForInOrOfInitializer,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyAssignmentPattern;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyAssignmentPattern {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsAnyAssignmentPattern, FormatJsAnyAssignmentPattern>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyAssignmentPattern {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsAnyAssignmentPattern, FormatJsAnyAssignmentPattern>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnySwitchClause;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnySwitchClause {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsAnySwitchClause, FormatJsAnySwitchClause>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnySwitchClause {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsAnySwitchClause, FormatJsAnySwitchClause>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyBindingPattern;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyBindingPattern {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsAnyBindingPattern, FormatJsAnyBindingPattern>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyBindingPattern {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsAnyBindingPattern, FormatJsAnyBindingPattern>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyDeclarationClause;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyDeclarationClause {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsAnyDeclarationClause, FormatJsAnyDeclarationClause>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyDeclarationClause {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsAnyDeclarationClause, FormatJsAnyDeclarationClause>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyLiteralExpression;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyLiteralExpression {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsAnyLiteralExpression, FormatJsAnyLiteralExpression>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyLiteralExpression {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsAnyLiteralExpression, FormatJsAnyLiteralExpression>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyTemplateElement;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyTemplateElement {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsAnyTemplateElement, FormatJsAnyTemplateElement>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyTemplateElement {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsAnyTemplateElement, FormatJsAnyTemplateElement>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyBinding;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyBinding {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsAnyBinding, FormatJsAnyBinding>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyBinding {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsAnyBinding, FormatJsAnyBinding>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyArrowFunctionParameters;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyArrowFunctionParameters {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyArrowFunctionParameters,
        FormatJsAnyArrowFunctionParameters,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyArrowFunctionParameters {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyArrowFunctionParameters,
        FormatJsAnyArrowFunctionParameters,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyFunctionBody;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyFunctionBody {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsAnyFunctionBody, FormatJsAnyFunctionBody>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyFunctionBody {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsAnyFunctionBody, FormatJsAnyFunctionBody>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyArrayElement;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyArrayElement {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsAnyArrayElement, FormatJsAnyArrayElement>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyArrayElement {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsAnyArrayElement, FormatJsAnyArrayElement>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyName;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyName {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsAnyName, FormatJsAnyName>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyName {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsAnyName, FormatJsAnyName>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyInProperty;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyInProperty {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsAnyInProperty, FormatJsAnyInProperty>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyInProperty {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsAnyInProperty, FormatJsAnyInProperty>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyAssignment;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyAssignment {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsAnyAssignment, FormatJsAnyAssignment>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyAssignment {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsAnyAssignment, FormatJsAnyAssignment>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyObjectMemberName;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyObjectMemberName {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsAnyObjectMemberName, FormatJsAnyObjectMemberName>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyObjectMemberName {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsAnyObjectMemberName, FormatJsAnyObjectMemberName>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyObjectMember;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyObjectMember {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsAnyObjectMember, FormatJsAnyObjectMember>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyObjectMember {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsAnyObjectMember, FormatJsAnyObjectMember>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyFormalParameter;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyFormalParameter {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsAnyFormalParameter, FormatJsAnyFormalParameter>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyFormalParameter {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsAnyFormalParameter, FormatJsAnyFormalParameter>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyClassMember;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyClassMember {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsAnyClassMember, FormatJsAnyClassMember>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyClassMember {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsAnyClassMember, FormatJsAnyClassMember>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyClass;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyClass {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsAnyClass, FormatJsAnyClass>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyClass {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsAnyClass, FormatJsAnyClass>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyClassMemberName;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyClassMemberName {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsAnyClassMemberName, FormatJsAnyClassMemberName>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyClassMemberName {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsAnyClassMemberName, FormatJsAnyClassMemberName>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyConstructorParameter;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyConstructorParameter {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyConstructorParameter,
        FormatJsAnyConstructorParameter,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyConstructorParameter {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyConstructorParameter,
        FormatJsAnyConstructorParameter,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsAnyPropertyParameterModifier;
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyPropertyParameterModifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyPropertyParameterModifier,
        FormatTsAnyPropertyParameterModifier,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsAnyPropertyParameterModifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAnyPropertyParameterModifier,
        FormatTsAnyPropertyParameterModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsAnyPropertyAnnotation;
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyPropertyAnnotation {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyPropertyAnnotation,
        FormatTsAnyPropertyAnnotation,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsAnyPropertyAnnotation {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::TsAnyPropertyAnnotation, FormatTsAnyPropertyAnnotation>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyPropertyModifier;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyPropertyModifier {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsAnyPropertyModifier, FormatJsAnyPropertyModifier>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyPropertyModifier {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsAnyPropertyModifier, FormatJsAnyPropertyModifier>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsAnyPropertySignatureAnnotation;
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyPropertySignatureAnnotation {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyPropertySignatureAnnotation,
        FormatTsAnyPropertySignatureAnnotation,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsAnyPropertySignatureAnnotation {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAnyPropertySignatureAnnotation,
        FormatTsAnyPropertySignatureAnnotation,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsAnyPropertySignatureModifier;
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyPropertySignatureModifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyPropertySignatureModifier,
        FormatTsAnyPropertySignatureModifier,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsAnyPropertySignatureModifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAnyPropertySignatureModifier,
        FormatTsAnyPropertySignatureModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyMethodModifier;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyMethodModifier {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsAnyMethodModifier, FormatJsAnyMethodModifier>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyMethodModifier {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsAnyMethodModifier, FormatJsAnyMethodModifier>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsAnyMethodSignatureModifier;
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyMethodSignatureModifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyMethodSignatureModifier,
        FormatTsAnyMethodSignatureModifier,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsAnyMethodSignatureModifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAnyMethodSignatureModifier,
        FormatTsAnyMethodSignatureModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsAnyIndexSignatureModifier;
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyIndexSignatureModifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyIndexSignatureModifier,
        FormatTsAnyIndexSignatureModifier,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsAnyIndexSignatureModifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAnyIndexSignatureModifier,
        FormatTsAnyIndexSignatureModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsType;
impl<'a> AsFormat<'a> for rome_js_syntax::TsType {
    type Format = FormatRefWithRule<'a, rome_js_syntax::TsType, FormatTsType>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsType {
    type Format = FormatOwnedWithRule<rome_js_syntax::TsType, FormatTsType>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyArrayAssignmentPatternElement;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyArrayAssignmentPatternElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyArrayAssignmentPatternElement,
        FormatJsAnyArrayAssignmentPatternElement,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyArrayAssignmentPatternElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyArrayAssignmentPatternElement,
        FormatJsAnyArrayAssignmentPatternElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyObjectAssignmentPatternMember;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyObjectAssignmentPatternMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyObjectAssignmentPatternMember,
        FormatJsAnyObjectAssignmentPatternMember,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyObjectAssignmentPatternMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyObjectAssignmentPatternMember,
        FormatJsAnyObjectAssignmentPatternMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyArrayBindingPatternElement;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyArrayBindingPatternElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyArrayBindingPatternElement,
        FormatJsAnyArrayBindingPatternElement,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyArrayBindingPatternElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyArrayBindingPatternElement,
        FormatJsAnyArrayBindingPatternElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyObjectBindingPatternMember;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyObjectBindingPatternMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyObjectBindingPatternMember,
        FormatJsAnyObjectBindingPatternMember,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyObjectBindingPatternMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyObjectBindingPatternMember,
        FormatJsAnyObjectBindingPatternMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyDeclaration;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyDeclaration {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsAnyDeclaration, FormatJsAnyDeclaration>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyDeclaration {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsAnyDeclaration, FormatJsAnyDeclaration>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsAnyReturnType;
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyReturnType {
    type Format = FormatRefWithRule<'a, rome_js_syntax::TsAnyReturnType, FormatTsAnyReturnType>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsAnyReturnType {
    type Format = FormatOwnedWithRule<rome_js_syntax::TsAnyReturnType, FormatTsAnyReturnType>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsAnyVariableAnnotation;
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyVariableAnnotation {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyVariableAnnotation,
        FormatTsAnyVariableAnnotation,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsAnyVariableAnnotation {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::TsAnyVariableAnnotation, FormatTsAnyVariableAnnotation>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyModuleItem;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyModuleItem {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsAnyModuleItem, FormatJsAnyModuleItem>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyModuleItem {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsAnyModuleItem, FormatJsAnyModuleItem>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyImportClause;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyImportClause {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsAnyImportClause, FormatJsAnyImportClause>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyImportClause {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsAnyImportClause, FormatJsAnyImportClause>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyNamedImport;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyNamedImport {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsAnyNamedImport, FormatJsAnyNamedImport>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyNamedImport {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsAnyNamedImport, FormatJsAnyNamedImport>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyNamedImportSpecifier;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyNamedImportSpecifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyNamedImportSpecifier,
        FormatJsAnyNamedImportSpecifier,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyNamedImportSpecifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyNamedImportSpecifier,
        FormatJsAnyNamedImportSpecifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyImportAssertionEntry;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyImportAssertionEntry {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyImportAssertionEntry,
        FormatJsAnyImportAssertionEntry,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyImportAssertionEntry {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyImportAssertionEntry,
        FormatJsAnyImportAssertionEntry,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyExportClause;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyExportClause {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsAnyExportClause, FormatJsAnyExportClause>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyExportClause {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsAnyExportClause, FormatJsAnyExportClause>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyExportDefaultDeclaration;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyExportDefaultDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyExportDefaultDeclaration,
        FormatJsAnyExportDefaultDeclaration,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyExportDefaultDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyExportDefaultDeclaration,
        FormatJsAnyExportDefaultDeclaration,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyExportNamedSpecifier;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyExportNamedSpecifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyExportNamedSpecifier,
        FormatJsAnyExportNamedSpecifier,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyExportNamedSpecifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyExportNamedSpecifier,
        FormatJsAnyExportNamedSpecifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyFunction;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyFunction {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsAnyFunction, FormatJsAnyFunction>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyFunction {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsAnyFunction, FormatJsAnyFunction>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyParameter;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyParameter {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsAnyParameter, FormatJsAnyParameter>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyParameter {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsAnyParameter, FormatJsAnyParameter>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsAnyCallArgument;
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyCallArgument {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsAnyCallArgument, FormatJsAnyCallArgument>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsAnyCallArgument {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsAnyCallArgument, FormatJsAnyCallArgument>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsAnyName;
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyName {
    type Format = FormatRefWithRule<'a, rome_js_syntax::TsAnyName, FormatTsAnyName>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsAnyName {
    type Format = FormatOwnedWithRule<rome_js_syntax::TsAnyName, FormatTsAnyName>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsAnyModuleReference;
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyModuleReference {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::TsAnyModuleReference, FormatTsAnyModuleReference>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsAnyModuleReference {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::TsAnyModuleReference, FormatTsAnyModuleReference>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsAnyModuleName;
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyModuleName {
    type Format = FormatRefWithRule<'a, rome_js_syntax::TsAnyModuleName, FormatTsAnyModuleName>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsAnyModuleName {
    type Format = FormatOwnedWithRule<rome_js_syntax::TsAnyModuleName, FormatTsAnyModuleName>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsAnyExternalModuleDeclarationBody;
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyExternalModuleDeclarationBody {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyExternalModuleDeclarationBody,
        FormatTsAnyExternalModuleDeclarationBody,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsAnyExternalModuleDeclarationBody {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAnyExternalModuleDeclarationBody,
        FormatTsAnyExternalModuleDeclarationBody,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsAnyTypePredicateParameterName;
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyTypePredicateParameterName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyTypePredicateParameterName,
        FormatTsAnyTypePredicateParameterName,
    >;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsAnyTypePredicateParameterName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAnyTypePredicateParameterName,
        FormatTsAnyTypePredicateParameterName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsAnyTypeMember;
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyTypeMember {
    type Format = FormatRefWithRule<'a, rome_js_syntax::TsAnyTypeMember, FormatTsAnyTypeMember>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsAnyTypeMember {
    type Format = FormatOwnedWithRule<rome_js_syntax::TsAnyTypeMember, FormatTsAnyTypeMember>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsAnyTupleTypeElement;
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyTupleTypeElement {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::TsAnyTupleTypeElement, FormatTsAnyTupleTypeElement>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsAnyTupleTypeElement {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::TsAnyTupleTypeElement, FormatTsAnyTupleTypeElement>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatTsAnyTemplateElement;
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyTemplateElement {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::TsAnyTemplateElement, FormatTsAnyTemplateElement>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::TsAnyTemplateElement {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::TsAnyTemplateElement, FormatTsAnyTemplateElement>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsxAnyTag;
impl<'a> AsFormat<'a> for rome_js_syntax::JsxAnyTag {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsxAnyTag, FormatJsxAnyTag>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxAnyTag {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsxAnyTag, FormatJsxAnyTag>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsxAnyElementName;
impl<'a> AsFormat<'a> for rome_js_syntax::JsxAnyElementName {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsxAnyElementName, FormatJsxAnyElementName>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxAnyElementName {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsxAnyElementName, FormatJsxAnyElementName>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsxAnyObjectName;
impl<'a> AsFormat<'a> for rome_js_syntax::JsxAnyObjectName {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsxAnyObjectName, FormatJsxAnyObjectName>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxAnyObjectName {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsxAnyObjectName, FormatJsxAnyObjectName>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsxAnyName;
impl<'a> AsFormat<'a> for rome_js_syntax::JsxAnyName {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsxAnyName, FormatJsxAnyName>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxAnyName {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsxAnyName, FormatJsxAnyName>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsxAnyAttribute;
impl<'a> AsFormat<'a> for rome_js_syntax::JsxAnyAttribute {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsxAnyAttribute, FormatJsxAnyAttribute>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxAnyAttribute {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsxAnyAttribute, FormatJsxAnyAttribute>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsxAnyAttributeName;
impl<'a> AsFormat<'a> for rome_js_syntax::JsxAnyAttributeName {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsxAnyAttributeName, FormatJsxAnyAttributeName>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxAnyAttributeName {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsxAnyAttributeName, FormatJsxAnyAttributeName>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsxAnyAttributeValue;
impl<'a> AsFormat<'a> for rome_js_syntax::JsxAnyAttributeValue {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsxAnyAttributeValue, FormatJsxAnyAttributeValue>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxAnyAttributeValue {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsxAnyAttributeValue, FormatJsxAnyAttributeValue>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
pub struct FormatJsxAnyChild;
impl<'a> AsFormat<'a> for rome_js_syntax::JsxAnyChild {
    type Format = FormatRefWithRule<'a, rome_js_syntax::JsxAnyChild, FormatJsxAnyChild>;
    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self)
    }
}
impl IntoFormat for rome_js_syntax::JsxAnyChild {
    type Format = FormatOwnedWithRule<rome_js_syntax::JsxAnyChild, FormatJsxAnyChild>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self)
    }
}
