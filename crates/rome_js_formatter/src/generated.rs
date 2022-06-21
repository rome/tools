//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{AsFormat, FormatNodeRule, IntoFormat, JsFormatContext, JsFormatter};
use rome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult, FormatRule};
impl FormatRule<rome_js_syntax::JsScript> for crate::js::auxiliary::script::FormatJsScript {
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsScript, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: auxiliary :: script :: FormatJsScript as FormatNodeRule < rome_js_syntax :: JsScript >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsScript {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsScript,
        crate::js::auxiliary::script::FormatJsScript,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsScript {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsScript, crate::js::auxiliary::script::FormatJsScript>;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsModule> for crate::js::auxiliary::module::FormatJsModule {
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsModule, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: auxiliary :: module :: FormatJsModule as FormatNodeRule < rome_js_syntax :: JsModule >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsModule {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsModule,
        crate::js::auxiliary::module::FormatJsModule,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsModule {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsModule, crate::js::auxiliary::module::FormatJsModule>;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsExpressionSnipped>
    for crate::js::auxiliary::expression_snipped::FormatJsExpressionSnipped
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsExpressionSnipped, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::auxiliary::expression_snipped::FormatJsExpressionSnipped as FormatNodeRule<
            rome_js_syntax::JsExpressionSnipped,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExpressionSnipped {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExpressionSnipped,
        crate::js::auxiliary::expression_snipped::FormatJsExpressionSnipped,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsExpressionSnipped {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExpressionSnipped,
        crate::js::auxiliary::expression_snipped::FormatJsExpressionSnipped,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsDirective>
    for crate::js::auxiliary::directive::FormatJsDirective
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsDirective, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::auxiliary::directive::FormatJsDirective as FormatNodeRule<
            rome_js_syntax::JsDirective,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsDirective {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsDirective,
        crate::js::auxiliary::directive::FormatJsDirective,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsDirective {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsDirective,
        crate::js::auxiliary::directive::FormatJsDirective,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsBlockStatement>
    for crate::js::statements::block_statement::FormatJsBlockStatement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsBlockStatement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::statements::block_statement::FormatJsBlockStatement as FormatNodeRule<
            rome_js_syntax::JsBlockStatement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsBlockStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsBlockStatement,
        crate::js::statements::block_statement::FormatJsBlockStatement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsBlockStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsBlockStatement,
        crate::js::statements::block_statement::FormatJsBlockStatement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsBreakStatement>
    for crate::js::statements::break_statement::FormatJsBreakStatement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsBreakStatement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::statements::break_statement::FormatJsBreakStatement as FormatNodeRule<
            rome_js_syntax::JsBreakStatement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsBreakStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsBreakStatement,
        crate::js::statements::break_statement::FormatJsBreakStatement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsBreakStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsBreakStatement,
        crate::js::statements::break_statement::FormatJsBreakStatement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsClassDeclaration>
    for crate::js::declarations::class_declaration::FormatJsClassDeclaration
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsClassDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::declarations::class_declaration::FormatJsClassDeclaration as FormatNodeRule<
            rome_js_syntax::JsClassDeclaration,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsClassDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsClassDeclaration,
        crate::js::declarations::class_declaration::FormatJsClassDeclaration,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsClassDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsClassDeclaration,
        crate::js::declarations::class_declaration::FormatJsClassDeclaration,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsContinueStatement>
    for crate::js::statements::continue_statement::FormatJsContinueStatement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsContinueStatement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::statements::continue_statement::FormatJsContinueStatement as FormatNodeRule<
            rome_js_syntax::JsContinueStatement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsContinueStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsContinueStatement,
        crate::js::statements::continue_statement::FormatJsContinueStatement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsContinueStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsContinueStatement,
        crate::js::statements::continue_statement::FormatJsContinueStatement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsDebuggerStatement>
    for crate::js::statements::debugger_statement::FormatJsDebuggerStatement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsDebuggerStatement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::statements::debugger_statement::FormatJsDebuggerStatement as FormatNodeRule<
            rome_js_syntax::JsDebuggerStatement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsDebuggerStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsDebuggerStatement,
        crate::js::statements::debugger_statement::FormatJsDebuggerStatement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsDebuggerStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsDebuggerStatement,
        crate::js::statements::debugger_statement::FormatJsDebuggerStatement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsDoWhileStatement>
    for crate::js::statements::do_while_statement::FormatJsDoWhileStatement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsDoWhileStatement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::statements::do_while_statement::FormatJsDoWhileStatement as FormatNodeRule<
            rome_js_syntax::JsDoWhileStatement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsDoWhileStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsDoWhileStatement,
        crate::js::statements::do_while_statement::FormatJsDoWhileStatement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsDoWhileStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsDoWhileStatement,
        crate::js::statements::do_while_statement::FormatJsDoWhileStatement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsEmptyStatement>
    for crate::js::statements::empty_statement::FormatJsEmptyStatement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsEmptyStatement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::statements::empty_statement::FormatJsEmptyStatement as FormatNodeRule<
            rome_js_syntax::JsEmptyStatement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsEmptyStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsEmptyStatement,
        crate::js::statements::empty_statement::FormatJsEmptyStatement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsEmptyStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsEmptyStatement,
        crate::js::statements::empty_statement::FormatJsEmptyStatement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsExpressionStatement>
    for crate::js::statements::expression_statement::FormatJsExpressionStatement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsExpressionStatement, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: statements :: expression_statement :: FormatJsExpressionStatement as FormatNodeRule < rome_js_syntax :: JsExpressionStatement >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExpressionStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExpressionStatement,
        crate::js::statements::expression_statement::FormatJsExpressionStatement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsExpressionStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExpressionStatement,
        crate::js::statements::expression_statement::FormatJsExpressionStatement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsForInStatement>
    for crate::js::statements::for_in_statement::FormatJsForInStatement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsForInStatement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::statements::for_in_statement::FormatJsForInStatement as FormatNodeRule<
            rome_js_syntax::JsForInStatement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsForInStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsForInStatement,
        crate::js::statements::for_in_statement::FormatJsForInStatement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsForInStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsForInStatement,
        crate::js::statements::for_in_statement::FormatJsForInStatement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsForOfStatement>
    for crate::js::statements::for_of_statement::FormatJsForOfStatement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsForOfStatement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::statements::for_of_statement::FormatJsForOfStatement as FormatNodeRule<
            rome_js_syntax::JsForOfStatement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsForOfStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsForOfStatement,
        crate::js::statements::for_of_statement::FormatJsForOfStatement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsForOfStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsForOfStatement,
        crate::js::statements::for_of_statement::FormatJsForOfStatement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsForStatement>
    for crate::js::statements::for_statement::FormatJsForStatement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsForStatement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::statements::for_statement::FormatJsForStatement as FormatNodeRule<
            rome_js_syntax::JsForStatement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsForStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsForStatement,
        crate::js::statements::for_statement::FormatJsForStatement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsForStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsForStatement,
        crate::js::statements::for_statement::FormatJsForStatement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsIfStatement>
    for crate::js::statements::if_statement::FormatJsIfStatement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsIfStatement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::statements::if_statement::FormatJsIfStatement as FormatNodeRule<
            rome_js_syntax::JsIfStatement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsIfStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsIfStatement,
        crate::js::statements::if_statement::FormatJsIfStatement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsIfStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsIfStatement,
        crate::js::statements::if_statement::FormatJsIfStatement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsLabeledStatement>
    for crate::js::statements::labeled_statement::FormatJsLabeledStatement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsLabeledStatement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::statements::labeled_statement::FormatJsLabeledStatement as FormatNodeRule<
            rome_js_syntax::JsLabeledStatement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsLabeledStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsLabeledStatement,
        crate::js::statements::labeled_statement::FormatJsLabeledStatement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsLabeledStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsLabeledStatement,
        crate::js::statements::labeled_statement::FormatJsLabeledStatement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsReturnStatement>
    for crate::js::statements::return_statement::FormatJsReturnStatement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsReturnStatement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::statements::return_statement::FormatJsReturnStatement as FormatNodeRule<
            rome_js_syntax::JsReturnStatement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsReturnStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsReturnStatement,
        crate::js::statements::return_statement::FormatJsReturnStatement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsReturnStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsReturnStatement,
        crate::js::statements::return_statement::FormatJsReturnStatement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsSwitchStatement>
    for crate::js::statements::switch_statement::FormatJsSwitchStatement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsSwitchStatement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::statements::switch_statement::FormatJsSwitchStatement as FormatNodeRule<
            rome_js_syntax::JsSwitchStatement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsSwitchStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsSwitchStatement,
        crate::js::statements::switch_statement::FormatJsSwitchStatement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsSwitchStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsSwitchStatement,
        crate::js::statements::switch_statement::FormatJsSwitchStatement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsThrowStatement>
    for crate::js::statements::throw_statement::FormatJsThrowStatement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsThrowStatement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::statements::throw_statement::FormatJsThrowStatement as FormatNodeRule<
            rome_js_syntax::JsThrowStatement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsThrowStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsThrowStatement,
        crate::js::statements::throw_statement::FormatJsThrowStatement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsThrowStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsThrowStatement,
        crate::js::statements::throw_statement::FormatJsThrowStatement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsTryFinallyStatement>
    for crate::js::statements::try_finally_statement::FormatJsTryFinallyStatement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsTryFinallyStatement, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: statements :: try_finally_statement :: FormatJsTryFinallyStatement as FormatNodeRule < rome_js_syntax :: JsTryFinallyStatement >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsTryFinallyStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsTryFinallyStatement,
        crate::js::statements::try_finally_statement::FormatJsTryFinallyStatement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsTryFinallyStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsTryFinallyStatement,
        crate::js::statements::try_finally_statement::FormatJsTryFinallyStatement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsTryStatement>
    for crate::js::statements::try_statement::FormatJsTryStatement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsTryStatement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::statements::try_statement::FormatJsTryStatement as FormatNodeRule<
            rome_js_syntax::JsTryStatement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsTryStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsTryStatement,
        crate::js::statements::try_statement::FormatJsTryStatement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsTryStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsTryStatement,
        crate::js::statements::try_statement::FormatJsTryStatement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsVariableStatement>
    for crate::js::statements::variable_statement::FormatJsVariableStatement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsVariableStatement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::statements::variable_statement::FormatJsVariableStatement as FormatNodeRule<
            rome_js_syntax::JsVariableStatement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsVariableStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsVariableStatement,
        crate::js::statements::variable_statement::FormatJsVariableStatement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsVariableStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsVariableStatement,
        crate::js::statements::variable_statement::FormatJsVariableStatement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsWhileStatement>
    for crate::js::statements::while_statement::FormatJsWhileStatement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsWhileStatement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::statements::while_statement::FormatJsWhileStatement as FormatNodeRule<
            rome_js_syntax::JsWhileStatement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsWhileStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsWhileStatement,
        crate::js::statements::while_statement::FormatJsWhileStatement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsWhileStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsWhileStatement,
        crate::js::statements::while_statement::FormatJsWhileStatement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsWithStatement>
    for crate::js::statements::with_statement::FormatJsWithStatement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsWithStatement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::statements::with_statement::FormatJsWithStatement as FormatNodeRule<
            rome_js_syntax::JsWithStatement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsWithStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsWithStatement,
        crate::js::statements::with_statement::FormatJsWithStatement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsWithStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsWithStatement,
        crate::js::statements::with_statement::FormatJsWithStatement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsFunctionDeclaration>
    for crate::js::declarations::function_declaration::FormatJsFunctionDeclaration
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsFunctionDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: declarations :: function_declaration :: FormatJsFunctionDeclaration as FormatNodeRule < rome_js_syntax :: JsFunctionDeclaration >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsFunctionDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsFunctionDeclaration,
        crate::js::declarations::function_declaration::FormatJsFunctionDeclaration,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsFunctionDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsFunctionDeclaration,
        crate::js::declarations::function_declaration::FormatJsFunctionDeclaration,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsEnumDeclaration>
    for crate::ts::declarations::enum_declaration::FormatTsEnumDeclaration
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsEnumDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::declarations::enum_declaration::FormatTsEnumDeclaration as FormatNodeRule<
            rome_js_syntax::TsEnumDeclaration,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsEnumDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsEnumDeclaration,
        crate::ts::declarations::enum_declaration::FormatTsEnumDeclaration,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsEnumDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsEnumDeclaration,
        crate::ts::declarations::enum_declaration::FormatTsEnumDeclaration,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsTypeAliasDeclaration>
    for crate::ts::declarations::type_alias_declaration::FormatTsTypeAliasDeclaration
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsTypeAliasDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: ts :: declarations :: type_alias_declaration :: FormatTsTypeAliasDeclaration as FormatNodeRule < rome_js_syntax :: TsTypeAliasDeclaration >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeAliasDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeAliasDeclaration,
        crate::ts::declarations::type_alias_declaration::FormatTsTypeAliasDeclaration,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsTypeAliasDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeAliasDeclaration,
        crate::ts::declarations::type_alias_declaration::FormatTsTypeAliasDeclaration,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsInterfaceDeclaration>
    for crate::ts::declarations::interface_declaration::FormatTsInterfaceDeclaration
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsInterfaceDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: ts :: declarations :: interface_declaration :: FormatTsInterfaceDeclaration as FormatNodeRule < rome_js_syntax :: TsInterfaceDeclaration >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsInterfaceDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsInterfaceDeclaration,
        crate::ts::declarations::interface_declaration::FormatTsInterfaceDeclaration,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsInterfaceDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsInterfaceDeclaration,
        crate::ts::declarations::interface_declaration::FormatTsInterfaceDeclaration,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsDeclareFunctionDeclaration>
    for crate::ts::declarations::declare_function_declaration::FormatTsDeclareFunctionDeclaration
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsDeclareFunctionDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: declarations :: declare_function_declaration :: FormatTsDeclareFunctionDeclaration as FormatNodeRule < rome_js_syntax :: TsDeclareFunctionDeclaration >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsDeclareFunctionDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsDeclareFunctionDeclaration,
        crate::ts::declarations::declare_function_declaration::FormatTsDeclareFunctionDeclaration,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsDeclareFunctionDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsDeclareFunctionDeclaration,
        crate::ts::declarations::declare_function_declaration::FormatTsDeclareFunctionDeclaration,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsDeclareStatement>
    for crate::ts::statements::declare_statement::FormatTsDeclareStatement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsDeclareStatement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::statements::declare_statement::FormatTsDeclareStatement as FormatNodeRule<
            rome_js_syntax::TsDeclareStatement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsDeclareStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsDeclareStatement,
        crate::ts::statements::declare_statement::FormatTsDeclareStatement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsDeclareStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsDeclareStatement,
        crate::ts::statements::declare_statement::FormatTsDeclareStatement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsModuleDeclaration>
    for crate::ts::declarations::module_declaration::FormatTsModuleDeclaration
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsModuleDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::declarations::module_declaration::FormatTsModuleDeclaration as FormatNodeRule<
            rome_js_syntax::TsModuleDeclaration,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsModuleDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsModuleDeclaration,
        crate::ts::declarations::module_declaration::FormatTsModuleDeclaration,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsModuleDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsModuleDeclaration,
        crate::ts::declarations::module_declaration::FormatTsModuleDeclaration,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsExternalModuleDeclaration>
    for crate::ts::declarations::external_module_declaration::FormatTsExternalModuleDeclaration
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsExternalModuleDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: declarations :: external_module_declaration :: FormatTsExternalModuleDeclaration as FormatNodeRule < rome_js_syntax :: TsExternalModuleDeclaration >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsExternalModuleDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsExternalModuleDeclaration,
        crate::ts::declarations::external_module_declaration::FormatTsExternalModuleDeclaration,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsExternalModuleDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsExternalModuleDeclaration,
        crate::ts::declarations::external_module_declaration::FormatTsExternalModuleDeclaration,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsGlobalDeclaration>
    for crate::ts::declarations::global_declaration::FormatTsGlobalDeclaration
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsGlobalDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::declarations::global_declaration::FormatTsGlobalDeclaration as FormatNodeRule<
            rome_js_syntax::TsGlobalDeclaration,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsGlobalDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsGlobalDeclaration,
        crate::ts::declarations::global_declaration::FormatTsGlobalDeclaration,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsGlobalDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsGlobalDeclaration,
        crate::ts::declarations::global_declaration::FormatTsGlobalDeclaration,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsImportEqualsDeclaration>
    for crate::ts::declarations::import_equals_declaration::FormatTsImportEqualsDeclaration
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsImportEqualsDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: declarations :: import_equals_declaration :: FormatTsImportEqualsDeclaration as FormatNodeRule < rome_js_syntax :: TsImportEqualsDeclaration >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsImportEqualsDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsImportEqualsDeclaration,
        crate::ts::declarations::import_equals_declaration::FormatTsImportEqualsDeclaration,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsImportEqualsDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsImportEqualsDeclaration,
        crate::ts::declarations::import_equals_declaration::FormatTsImportEqualsDeclaration,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsElseClause>
    for crate::js::auxiliary::else_clause::FormatJsElseClause
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsElseClause, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::auxiliary::else_clause::FormatJsElseClause as FormatNodeRule<
            rome_js_syntax::JsElseClause,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsElseClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsElseClause,
        crate::js::auxiliary::else_clause::FormatJsElseClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsElseClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsElseClause,
        crate::js::auxiliary::else_clause::FormatJsElseClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsVariableDeclaration>
    for crate::js::declarations::variable_declaration::FormatJsVariableDeclaration
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsVariableDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: declarations :: variable_declaration :: FormatJsVariableDeclaration as FormatNodeRule < rome_js_syntax :: JsVariableDeclaration >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsVariableDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsVariableDeclaration,
        crate::js::declarations::variable_declaration::FormatJsVariableDeclaration,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsVariableDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsVariableDeclaration,
        crate::js::declarations::variable_declaration::FormatJsVariableDeclaration,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsForVariableDeclaration>
    for crate::js::declarations::for_variable_declaration::FormatJsForVariableDeclaration
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsForVariableDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: declarations :: for_variable_declaration :: FormatJsForVariableDeclaration as FormatNodeRule < rome_js_syntax :: JsForVariableDeclaration >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsForVariableDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsForVariableDeclaration,
        crate::js::declarations::for_variable_declaration::FormatJsForVariableDeclaration,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsForVariableDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsForVariableDeclaration,
        crate::js::declarations::for_variable_declaration::FormatJsForVariableDeclaration,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsVariableDeclarator>
    for crate::js::auxiliary::variable_declarator::FormatJsVariableDeclarator
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsVariableDeclarator, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::auxiliary::variable_declarator::FormatJsVariableDeclarator as FormatNodeRule<
            rome_js_syntax::JsVariableDeclarator,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsVariableDeclarator {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsVariableDeclarator,
        crate::js::auxiliary::variable_declarator::FormatJsVariableDeclarator,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsVariableDeclarator {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsVariableDeclarator,
        crate::js::auxiliary::variable_declarator::FormatJsVariableDeclarator,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsCaseClause>
    for crate::js::auxiliary::case_clause::FormatJsCaseClause
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsCaseClause, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::auxiliary::case_clause::FormatJsCaseClause as FormatNodeRule<
            rome_js_syntax::JsCaseClause,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsCaseClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsCaseClause,
        crate::js::auxiliary::case_clause::FormatJsCaseClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsCaseClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsCaseClause,
        crate::js::auxiliary::case_clause::FormatJsCaseClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsDefaultClause>
    for crate::js::auxiliary::default_clause::FormatJsDefaultClause
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsDefaultClause, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::auxiliary::default_clause::FormatJsDefaultClause as FormatNodeRule<
            rome_js_syntax::JsDefaultClause,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsDefaultClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsDefaultClause,
        crate::js::auxiliary::default_clause::FormatJsDefaultClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsDefaultClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsDefaultClause,
        crate::js::auxiliary::default_clause::FormatJsDefaultClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsCatchClause>
    for crate::js::auxiliary::catch_clause::FormatJsCatchClause
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsCatchClause, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::auxiliary::catch_clause::FormatJsCatchClause as FormatNodeRule<
            rome_js_syntax::JsCatchClause,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsCatchClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsCatchClause,
        crate::js::auxiliary::catch_clause::FormatJsCatchClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsCatchClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsCatchClause,
        crate::js::auxiliary::catch_clause::FormatJsCatchClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsFinallyClause>
    for crate::js::auxiliary::finally_clause::FormatJsFinallyClause
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsFinallyClause, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::auxiliary::finally_clause::FormatJsFinallyClause as FormatNodeRule<
            rome_js_syntax::JsFinallyClause,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsFinallyClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsFinallyClause,
        crate::js::auxiliary::finally_clause::FormatJsFinallyClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsFinallyClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsFinallyClause,
        crate::js::auxiliary::finally_clause::FormatJsFinallyClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsCatchDeclaration>
    for crate::js::declarations::catch_declaration::FormatJsCatchDeclaration
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsCatchDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::declarations::catch_declaration::FormatJsCatchDeclaration as FormatNodeRule<
            rome_js_syntax::JsCatchDeclaration,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsCatchDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsCatchDeclaration,
        crate::js::declarations::catch_declaration::FormatJsCatchDeclaration,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsCatchDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsCatchDeclaration,
        crate::js::declarations::catch_declaration::FormatJsCatchDeclaration,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsTypeAnnotation>
    for crate::ts::auxiliary::type_annotation::FormatTsTypeAnnotation
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsTypeAnnotation, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::auxiliary::type_annotation::FormatTsTypeAnnotation as FormatNodeRule<
            rome_js_syntax::TsTypeAnnotation,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeAnnotation {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeAnnotation,
        crate::ts::auxiliary::type_annotation::FormatTsTypeAnnotation,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsTypeAnnotation {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeAnnotation,
        crate::ts::auxiliary::type_annotation::FormatTsTypeAnnotation,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::ImportMeta> for crate::js::module::import_meta::FormatImportMeta {
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::ImportMeta, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::module::import_meta::FormatImportMeta as FormatNodeRule<
            rome_js_syntax::ImportMeta,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::ImportMeta {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::ImportMeta,
        crate::js::module::import_meta::FormatImportMeta,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::ImportMeta {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::ImportMeta,
        crate::js::module::import_meta::FormatImportMeta,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsArrayExpression>
    for crate::js::expressions::array_expression::FormatJsArrayExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsArrayExpression, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::expressions::array_expression::FormatJsArrayExpression as FormatNodeRule<
            rome_js_syntax::JsArrayExpression,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsArrayExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsArrayExpression,
        crate::js::expressions::array_expression::FormatJsArrayExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsArrayExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsArrayExpression,
        crate::js::expressions::array_expression::FormatJsArrayExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsArrowFunctionExpression>
    for crate::js::expressions::arrow_function_expression::FormatJsArrowFunctionExpression
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsArrowFunctionExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: expressions :: arrow_function_expression :: FormatJsArrowFunctionExpression as FormatNodeRule < rome_js_syntax :: JsArrowFunctionExpression >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsArrowFunctionExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsArrowFunctionExpression,
        crate::js::expressions::arrow_function_expression::FormatJsArrowFunctionExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsArrowFunctionExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsArrowFunctionExpression,
        crate::js::expressions::arrow_function_expression::FormatJsArrowFunctionExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsAssignmentExpression>
    for crate::js::expressions::assignment_expression::FormatJsAssignmentExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsAssignmentExpression, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: expressions :: assignment_expression :: FormatJsAssignmentExpression as FormatNodeRule < rome_js_syntax :: JsAssignmentExpression >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAssignmentExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAssignmentExpression,
        crate::js::expressions::assignment_expression::FormatJsAssignmentExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAssignmentExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAssignmentExpression,
        crate::js::expressions::assignment_expression::FormatJsAssignmentExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsAwaitExpression>
    for crate::js::expressions::await_expression::FormatJsAwaitExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsAwaitExpression, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::expressions::await_expression::FormatJsAwaitExpression as FormatNodeRule<
            rome_js_syntax::JsAwaitExpression,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAwaitExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAwaitExpression,
        crate::js::expressions::await_expression::FormatJsAwaitExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAwaitExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAwaitExpression,
        crate::js::expressions::await_expression::FormatJsAwaitExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsBinaryExpression>
    for crate::js::expressions::binary_expression::FormatJsBinaryExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsBinaryExpression, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::expressions::binary_expression::FormatJsBinaryExpression as FormatNodeRule<
            rome_js_syntax::JsBinaryExpression,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsBinaryExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsBinaryExpression,
        crate::js::expressions::binary_expression::FormatJsBinaryExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsBinaryExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsBinaryExpression,
        crate::js::expressions::binary_expression::FormatJsBinaryExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsCallExpression>
    for crate::js::expressions::call_expression::FormatJsCallExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsCallExpression, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::expressions::call_expression::FormatJsCallExpression as FormatNodeRule<
            rome_js_syntax::JsCallExpression,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsCallExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsCallExpression,
        crate::js::expressions::call_expression::FormatJsCallExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsCallExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsCallExpression,
        crate::js::expressions::call_expression::FormatJsCallExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsClassExpression>
    for crate::js::expressions::class_expression::FormatJsClassExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsClassExpression, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::expressions::class_expression::FormatJsClassExpression as FormatNodeRule<
            rome_js_syntax::JsClassExpression,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsClassExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsClassExpression,
        crate::js::expressions::class_expression::FormatJsClassExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsClassExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsClassExpression,
        crate::js::expressions::class_expression::FormatJsClassExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsComputedMemberExpression>
    for crate::js::expressions::computed_member_expression::FormatJsComputedMemberExpression
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsComputedMemberExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: expressions :: computed_member_expression :: FormatJsComputedMemberExpression as FormatNodeRule < rome_js_syntax :: JsComputedMemberExpression >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsComputedMemberExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsComputedMemberExpression,
        crate::js::expressions::computed_member_expression::FormatJsComputedMemberExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsComputedMemberExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsComputedMemberExpression,
        crate::js::expressions::computed_member_expression::FormatJsComputedMemberExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsConditionalExpression>
    for crate::js::expressions::conditional_expression::FormatJsConditionalExpression
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsConditionalExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: expressions :: conditional_expression :: FormatJsConditionalExpression as FormatNodeRule < rome_js_syntax :: JsConditionalExpression >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsConditionalExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsConditionalExpression,
        crate::js::expressions::conditional_expression::FormatJsConditionalExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsConditionalExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsConditionalExpression,
        crate::js::expressions::conditional_expression::FormatJsConditionalExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsFunctionExpression>
    for crate::js::expressions::function_expression::FormatJsFunctionExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsFunctionExpression, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: expressions :: function_expression :: FormatJsFunctionExpression as FormatNodeRule < rome_js_syntax :: JsFunctionExpression >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsFunctionExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsFunctionExpression,
        crate::js::expressions::function_expression::FormatJsFunctionExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsFunctionExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsFunctionExpression,
        crate::js::expressions::function_expression::FormatJsFunctionExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsIdentifierExpression>
    for crate::js::expressions::identifier_expression::FormatJsIdentifierExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsIdentifierExpression, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: expressions :: identifier_expression :: FormatJsIdentifierExpression as FormatNodeRule < rome_js_syntax :: JsIdentifierExpression >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsIdentifierExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsIdentifierExpression,
        crate::js::expressions::identifier_expression::FormatJsIdentifierExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsIdentifierExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsIdentifierExpression,
        crate::js::expressions::identifier_expression::FormatJsIdentifierExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsImportCallExpression>
    for crate::js::expressions::import_call_expression::FormatJsImportCallExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsImportCallExpression, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: expressions :: import_call_expression :: FormatJsImportCallExpression as FormatNodeRule < rome_js_syntax :: JsImportCallExpression >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsImportCallExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsImportCallExpression,
        crate::js::expressions::import_call_expression::FormatJsImportCallExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsImportCallExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsImportCallExpression,
        crate::js::expressions::import_call_expression::FormatJsImportCallExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsInExpression>
    for crate::js::expressions::in_expression::FormatJsInExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsInExpression, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::expressions::in_expression::FormatJsInExpression as FormatNodeRule<
            rome_js_syntax::JsInExpression,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsInExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsInExpression,
        crate::js::expressions::in_expression::FormatJsInExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsInExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsInExpression,
        crate::js::expressions::in_expression::FormatJsInExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsInstanceofExpression>
    for crate::js::expressions::instanceof_expression::FormatJsInstanceofExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsInstanceofExpression, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: expressions :: instanceof_expression :: FormatJsInstanceofExpression as FormatNodeRule < rome_js_syntax :: JsInstanceofExpression >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsInstanceofExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsInstanceofExpression,
        crate::js::expressions::instanceof_expression::FormatJsInstanceofExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsInstanceofExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsInstanceofExpression,
        crate::js::expressions::instanceof_expression::FormatJsInstanceofExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsLogicalExpression>
    for crate::js::expressions::logical_expression::FormatJsLogicalExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsLogicalExpression, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::expressions::logical_expression::FormatJsLogicalExpression as FormatNodeRule<
            rome_js_syntax::JsLogicalExpression,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsLogicalExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsLogicalExpression,
        crate::js::expressions::logical_expression::FormatJsLogicalExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsLogicalExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsLogicalExpression,
        crate::js::expressions::logical_expression::FormatJsLogicalExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsNewExpression>
    for crate::js::expressions::new_expression::FormatJsNewExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsNewExpression, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::expressions::new_expression::FormatJsNewExpression as FormatNodeRule<
            rome_js_syntax::JsNewExpression,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsNewExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsNewExpression,
        crate::js::expressions::new_expression::FormatJsNewExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsNewExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsNewExpression,
        crate::js::expressions::new_expression::FormatJsNewExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsObjectExpression>
    for crate::js::expressions::object_expression::FormatJsObjectExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsObjectExpression, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::expressions::object_expression::FormatJsObjectExpression as FormatNodeRule<
            rome_js_syntax::JsObjectExpression,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsObjectExpression,
        crate::js::expressions::object_expression::FormatJsObjectExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsObjectExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsObjectExpression,
        crate::js::expressions::object_expression::FormatJsObjectExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsParenthesizedExpression>
    for crate::js::expressions::parenthesized_expression::FormatJsParenthesizedExpression
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsParenthesizedExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: expressions :: parenthesized_expression :: FormatJsParenthesizedExpression as FormatNodeRule < rome_js_syntax :: JsParenthesizedExpression >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsParenthesizedExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsParenthesizedExpression,
        crate::js::expressions::parenthesized_expression::FormatJsParenthesizedExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsParenthesizedExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsParenthesizedExpression,
        crate::js::expressions::parenthesized_expression::FormatJsParenthesizedExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsPostUpdateExpression>
    for crate::js::expressions::post_update_expression::FormatJsPostUpdateExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsPostUpdateExpression, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: expressions :: post_update_expression :: FormatJsPostUpdateExpression as FormatNodeRule < rome_js_syntax :: JsPostUpdateExpression >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsPostUpdateExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsPostUpdateExpression,
        crate::js::expressions::post_update_expression::FormatJsPostUpdateExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsPostUpdateExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsPostUpdateExpression,
        crate::js::expressions::post_update_expression::FormatJsPostUpdateExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsPreUpdateExpression>
    for crate::js::expressions::pre_update_expression::FormatJsPreUpdateExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsPreUpdateExpression, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: expressions :: pre_update_expression :: FormatJsPreUpdateExpression as FormatNodeRule < rome_js_syntax :: JsPreUpdateExpression >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsPreUpdateExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsPreUpdateExpression,
        crate::js::expressions::pre_update_expression::FormatJsPreUpdateExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsPreUpdateExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsPreUpdateExpression,
        crate::js::expressions::pre_update_expression::FormatJsPreUpdateExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsSequenceExpression>
    for crate::js::expressions::sequence_expression::FormatJsSequenceExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsSequenceExpression, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: expressions :: sequence_expression :: FormatJsSequenceExpression as FormatNodeRule < rome_js_syntax :: JsSequenceExpression >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsSequenceExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsSequenceExpression,
        crate::js::expressions::sequence_expression::FormatJsSequenceExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsSequenceExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsSequenceExpression,
        crate::js::expressions::sequence_expression::FormatJsSequenceExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsStaticMemberExpression>
    for crate::js::expressions::static_member_expression::FormatJsStaticMemberExpression
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsStaticMemberExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: expressions :: static_member_expression :: FormatJsStaticMemberExpression as FormatNodeRule < rome_js_syntax :: JsStaticMemberExpression >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsStaticMemberExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsStaticMemberExpression,
        crate::js::expressions::static_member_expression::FormatJsStaticMemberExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsStaticMemberExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsStaticMemberExpression,
        crate::js::expressions::static_member_expression::FormatJsStaticMemberExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsSuperExpression>
    for crate::js::expressions::super_expression::FormatJsSuperExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsSuperExpression, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::expressions::super_expression::FormatJsSuperExpression as FormatNodeRule<
            rome_js_syntax::JsSuperExpression,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsSuperExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsSuperExpression,
        crate::js::expressions::super_expression::FormatJsSuperExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsSuperExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsSuperExpression,
        crate::js::expressions::super_expression::FormatJsSuperExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsThisExpression>
    for crate::js::expressions::this_expression::FormatJsThisExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsThisExpression, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::expressions::this_expression::FormatJsThisExpression as FormatNodeRule<
            rome_js_syntax::JsThisExpression,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsThisExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsThisExpression,
        crate::js::expressions::this_expression::FormatJsThisExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsThisExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsThisExpression,
        crate::js::expressions::this_expression::FormatJsThisExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsUnaryExpression>
    for crate::js::expressions::unary_expression::FormatJsUnaryExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsUnaryExpression, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::expressions::unary_expression::FormatJsUnaryExpression as FormatNodeRule<
            rome_js_syntax::JsUnaryExpression,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsUnaryExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsUnaryExpression,
        crate::js::expressions::unary_expression::FormatJsUnaryExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsUnaryExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsUnaryExpression,
        crate::js::expressions::unary_expression::FormatJsUnaryExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsYieldExpression>
    for crate::js::expressions::yield_expression::FormatJsYieldExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsYieldExpression, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::expressions::yield_expression::FormatJsYieldExpression as FormatNodeRule<
            rome_js_syntax::JsYieldExpression,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsYieldExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsYieldExpression,
        crate::js::expressions::yield_expression::FormatJsYieldExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsYieldExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsYieldExpression,
        crate::js::expressions::yield_expression::FormatJsYieldExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::NewTarget> for crate::js::auxiliary::new_target::FormatNewTarget {
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::NewTarget, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::auxiliary::new_target::FormatNewTarget as FormatNodeRule<
            rome_js_syntax::NewTarget,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::NewTarget {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::NewTarget,
        crate::js::auxiliary::new_target::FormatNewTarget,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::NewTarget {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::NewTarget,
        crate::js::auxiliary::new_target::FormatNewTarget,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsTemplate> for crate::js::expressions::template::FormatJsTemplate {
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsTemplate, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::expressions::template::FormatJsTemplate as FormatNodeRule<
            rome_js_syntax::JsTemplate,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsTemplate {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsTemplate,
        crate::js::expressions::template::FormatJsTemplate,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsTemplate {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsTemplate,
        crate::js::expressions::template::FormatJsTemplate,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsTypeAssertionExpression>
    for crate::ts::expressions::type_assertion_expression::FormatTsTypeAssertionExpression
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsTypeAssertionExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: expressions :: type_assertion_expression :: FormatTsTypeAssertionExpression as FormatNodeRule < rome_js_syntax :: TsTypeAssertionExpression >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeAssertionExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeAssertionExpression,
        crate::ts::expressions::type_assertion_expression::FormatTsTypeAssertionExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsTypeAssertionExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeAssertionExpression,
        crate::ts::expressions::type_assertion_expression::FormatTsTypeAssertionExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsAsExpression>
    for crate::ts::expressions::as_expression::FormatTsAsExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsAsExpression, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::expressions::as_expression::FormatTsAsExpression as FormatNodeRule<
            rome_js_syntax::TsAsExpression,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAsExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAsExpression,
        crate::ts::expressions::as_expression::FormatTsAsExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsAsExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAsExpression,
        crate::ts::expressions::as_expression::FormatTsAsExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsNonNullAssertionExpression>
    for crate::ts::expressions::non_null_assertion_expression::FormatTsNonNullAssertionExpression
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsNonNullAssertionExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: expressions :: non_null_assertion_expression :: FormatTsNonNullAssertionExpression as FormatNodeRule < rome_js_syntax :: TsNonNullAssertionExpression >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsNonNullAssertionExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsNonNullAssertionExpression,
        crate::ts::expressions::non_null_assertion_expression::FormatTsNonNullAssertionExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsNonNullAssertionExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsNonNullAssertionExpression,
        crate::ts::expressions::non_null_assertion_expression::FormatTsNonNullAssertionExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsxTagExpression>
    for crate::jsx::expressions::tag_expression::FormatJsxTagExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsxTagExpression, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::jsx::expressions::tag_expression::FormatJsxTagExpression as FormatNodeRule<
            rome_js_syntax::JsxTagExpression,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxTagExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxTagExpression,
        crate::jsx::expressions::tag_expression::FormatJsxTagExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxTagExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxTagExpression,
        crate::jsx::expressions::tag_expression::FormatJsxTagExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsTypeArguments>
    for crate::ts::expressions::type_arguments::FormatTsTypeArguments
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsTypeArguments, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::expressions::type_arguments::FormatTsTypeArguments as FormatNodeRule<
            rome_js_syntax::TsTypeArguments,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeArguments {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeArguments,
        crate::ts::expressions::type_arguments::FormatTsTypeArguments,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsTypeArguments {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeArguments,
        crate::ts::expressions::type_arguments::FormatTsTypeArguments,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsTemplateChunkElement>
    for crate::js::expressions::template_chunk_element::FormatJsTemplateChunkElement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsTemplateChunkElement, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: expressions :: template_chunk_element :: FormatJsTemplateChunkElement as FormatNodeRule < rome_js_syntax :: JsTemplateChunkElement >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsTemplateChunkElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsTemplateChunkElement,
        crate::js::expressions::template_chunk_element::FormatJsTemplateChunkElement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsTemplateChunkElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsTemplateChunkElement,
        crate::js::expressions::template_chunk_element::FormatJsTemplateChunkElement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsTemplateElement>
    for crate::js::expressions::template_element::FormatJsTemplateElement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsTemplateElement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::expressions::template_element::FormatJsTemplateElement as FormatNodeRule<
            rome_js_syntax::JsTemplateElement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsTemplateElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsTemplateElement,
        crate::js::expressions::template_element::FormatJsTemplateElement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsTemplateElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsTemplateElement,
        crate::js::expressions::template_element::FormatJsTemplateElement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsCallArguments>
    for crate::js::expressions::call_arguments::FormatJsCallArguments
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsCallArguments, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::expressions::call_arguments::FormatJsCallArguments as FormatNodeRule<
            rome_js_syntax::JsCallArguments,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsCallArguments {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsCallArguments,
        crate::js::expressions::call_arguments::FormatJsCallArguments,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsCallArguments {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsCallArguments,
        crate::js::expressions::call_arguments::FormatJsCallArguments,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsYieldArgument>
    for crate::js::expressions::yield_argument::FormatJsYieldArgument
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsYieldArgument, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::expressions::yield_argument::FormatJsYieldArgument as FormatNodeRule<
            rome_js_syntax::JsYieldArgument,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsYieldArgument {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsYieldArgument,
        crate::js::expressions::yield_argument::FormatJsYieldArgument,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsYieldArgument {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsYieldArgument,
        crate::js::expressions::yield_argument::FormatJsYieldArgument,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsTypeParameters>
    for crate::ts::bindings::type_parameters::FormatTsTypeParameters
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsTypeParameters, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::bindings::type_parameters::FormatTsTypeParameters as FormatNodeRule<
            rome_js_syntax::TsTypeParameters,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeParameters {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeParameters,
        crate::ts::bindings::type_parameters::FormatTsTypeParameters,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsTypeParameters {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeParameters,
        crate::ts::bindings::type_parameters::FormatTsTypeParameters,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsParameters>
    for crate::js::bindings::parameters::FormatJsParameters
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsParameters, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::bindings::parameters::FormatJsParameters as FormatNodeRule<
            rome_js_syntax::JsParameters,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsParameters {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsParameters,
        crate::js::bindings::parameters::FormatJsParameters,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsParameters {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsParameters,
        crate::js::bindings::parameters::FormatJsParameters,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsReturnTypeAnnotation>
    for crate::ts::auxiliary::return_type_annotation::FormatTsReturnTypeAnnotation
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsReturnTypeAnnotation, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: ts :: auxiliary :: return_type_annotation :: FormatTsReturnTypeAnnotation as FormatNodeRule < rome_js_syntax :: TsReturnTypeAnnotation >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsReturnTypeAnnotation {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsReturnTypeAnnotation,
        crate::ts::auxiliary::return_type_annotation::FormatTsReturnTypeAnnotation,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsReturnTypeAnnotation {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsReturnTypeAnnotation,
        crate::ts::auxiliary::return_type_annotation::FormatTsReturnTypeAnnotation,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsFunctionBody>
    for crate::js::auxiliary::function_body::FormatJsFunctionBody
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsFunctionBody, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::auxiliary::function_body::FormatJsFunctionBody as FormatNodeRule<
            rome_js_syntax::JsFunctionBody,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsFunctionBody {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsFunctionBody,
        crate::js::auxiliary::function_body::FormatJsFunctionBody,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsFunctionBody {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsFunctionBody,
        crate::js::auxiliary::function_body::FormatJsFunctionBody,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsSpread> for crate::js::auxiliary::spread::FormatJsSpread {
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsSpread, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: auxiliary :: spread :: FormatJsSpread as FormatNodeRule < rome_js_syntax :: JsSpread >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsSpread {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsSpread,
        crate::js::auxiliary::spread::FormatJsSpread,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsSpread {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsSpread, crate::js::auxiliary::spread::FormatJsSpread>;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsArrayHole>
    for crate::js::auxiliary::array_hole::FormatJsArrayHole
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsArrayHole, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::auxiliary::array_hole::FormatJsArrayHole as FormatNodeRule<
            rome_js_syntax::JsArrayHole,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsArrayHole {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsArrayHole,
        crate::js::auxiliary::array_hole::FormatJsArrayHole,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsArrayHole {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsArrayHole,
        crate::js::auxiliary::array_hole::FormatJsArrayHole,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsReferenceIdentifier>
    for crate::js::auxiliary::reference_identifier::FormatJsReferenceIdentifier
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsReferenceIdentifier, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: auxiliary :: reference_identifier :: FormatJsReferenceIdentifier as FormatNodeRule < rome_js_syntax :: JsReferenceIdentifier >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsReferenceIdentifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsReferenceIdentifier,
        crate::js::auxiliary::reference_identifier::FormatJsReferenceIdentifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsReferenceIdentifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsReferenceIdentifier,
        crate::js::auxiliary::reference_identifier::FormatJsReferenceIdentifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsPrivateName>
    for crate::js::auxiliary::private_name::FormatJsPrivateName
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsPrivateName, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::auxiliary::private_name::FormatJsPrivateName as FormatNodeRule<
            rome_js_syntax::JsPrivateName,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsPrivateName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsPrivateName,
        crate::js::auxiliary::private_name::FormatJsPrivateName,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsPrivateName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsPrivateName,
        crate::js::auxiliary::private_name::FormatJsPrivateName,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsLiteralMemberName>
    for crate::js::objects::literal_member_name::FormatJsLiteralMemberName
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsLiteralMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::objects::literal_member_name::FormatJsLiteralMemberName as FormatNodeRule<
            rome_js_syntax::JsLiteralMemberName,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsLiteralMemberName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsLiteralMemberName,
        crate::js::objects::literal_member_name::FormatJsLiteralMemberName,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsLiteralMemberName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsLiteralMemberName,
        crate::js::objects::literal_member_name::FormatJsLiteralMemberName,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsComputedMemberName>
    for crate::js::objects::computed_member_name::FormatJsComputedMemberName
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsComputedMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::objects::computed_member_name::FormatJsComputedMemberName as FormatNodeRule<
            rome_js_syntax::JsComputedMemberName,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsComputedMemberName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsComputedMemberName,
        crate::js::objects::computed_member_name::FormatJsComputedMemberName,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsComputedMemberName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsComputedMemberName,
        crate::js::objects::computed_member_name::FormatJsComputedMemberName,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsPropertyObjectMember>
    for crate::js::objects::property_object_member::FormatJsPropertyObjectMember
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsPropertyObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: objects :: property_object_member :: FormatJsPropertyObjectMember as FormatNodeRule < rome_js_syntax :: JsPropertyObjectMember >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsPropertyObjectMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsPropertyObjectMember,
        crate::js::objects::property_object_member::FormatJsPropertyObjectMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsPropertyObjectMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsPropertyObjectMember,
        crate::js::objects::property_object_member::FormatJsPropertyObjectMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsMethodObjectMember>
    for crate::js::objects::method_object_member::FormatJsMethodObjectMember
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsMethodObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::objects::method_object_member::FormatJsMethodObjectMember as FormatNodeRule<
            rome_js_syntax::JsMethodObjectMember,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsMethodObjectMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsMethodObjectMember,
        crate::js::objects::method_object_member::FormatJsMethodObjectMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsMethodObjectMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsMethodObjectMember,
        crate::js::objects::method_object_member::FormatJsMethodObjectMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsGetterObjectMember>
    for crate::js::objects::getter_object_member::FormatJsGetterObjectMember
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsGetterObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::objects::getter_object_member::FormatJsGetterObjectMember as FormatNodeRule<
            rome_js_syntax::JsGetterObjectMember,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsGetterObjectMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsGetterObjectMember,
        crate::js::objects::getter_object_member::FormatJsGetterObjectMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsGetterObjectMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsGetterObjectMember,
        crate::js::objects::getter_object_member::FormatJsGetterObjectMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsSetterObjectMember>
    for crate::js::objects::setter_object_member::FormatJsSetterObjectMember
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsSetterObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::objects::setter_object_member::FormatJsSetterObjectMember as FormatNodeRule<
            rome_js_syntax::JsSetterObjectMember,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsSetterObjectMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsSetterObjectMember,
        crate::js::objects::setter_object_member::FormatJsSetterObjectMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsSetterObjectMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsSetterObjectMember,
        crate::js::objects::setter_object_member::FormatJsSetterObjectMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsShorthandPropertyObjectMember>
    for crate::js::objects::shorthand_property_object_member::FormatJsShorthandPropertyObjectMember
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsShorthandPropertyObjectMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: objects :: shorthand_property_object_member :: FormatJsShorthandPropertyObjectMember as FormatNodeRule < rome_js_syntax :: JsShorthandPropertyObjectMember >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsShorthandPropertyObjectMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsShorthandPropertyObjectMember,
        crate::js::objects::shorthand_property_object_member::FormatJsShorthandPropertyObjectMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsShorthandPropertyObjectMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsShorthandPropertyObjectMember,
        crate::js::objects::shorthand_property_object_member::FormatJsShorthandPropertyObjectMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsExtendsClause>
    for crate::js::classes::extends_clause::FormatJsExtendsClause
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsExtendsClause, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::classes::extends_clause::FormatJsExtendsClause as FormatNodeRule<
            rome_js_syntax::JsExtendsClause,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExtendsClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExtendsClause,
        crate::js::classes::extends_clause::FormatJsExtendsClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsExtendsClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExtendsClause,
        crate::js::classes::extends_clause::FormatJsExtendsClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsImplementsClause>
    for crate::ts::auxiliary::implements_clause::FormatTsImplementsClause
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsImplementsClause, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::auxiliary::implements_clause::FormatTsImplementsClause as FormatNodeRule<
            rome_js_syntax::TsImplementsClause,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsImplementsClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsImplementsClause,
        crate::ts::auxiliary::implements_clause::FormatTsImplementsClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsImplementsClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsImplementsClause,
        crate::ts::auxiliary::implements_clause::FormatTsImplementsClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule < rome_js_syntax :: JsClassExportDefaultDeclaration > for crate :: js :: declarations :: class_export_default_declaration :: FormatJsClassExportDefaultDeclaration { type Context = JsFormatContext ; fn fmt (node : & rome_js_syntax :: JsClassExportDefaultDeclaration , f : & mut JsFormatter) -> FormatResult < () > { < crate :: js :: declarations :: class_export_default_declaration :: FormatJsClassExportDefaultDeclaration as FormatNodeRule < rome_js_syntax :: JsClassExportDefaultDeclaration >> :: fmt (node , f) } }
impl<'a> AsFormat<'a> for rome_js_syntax::JsClassExportDefaultDeclaration {
    type Format = FormatRefWithRule < 'a , rome_js_syntax :: JsClassExportDefaultDeclaration , crate :: js :: declarations :: class_export_default_declaration :: FormatJsClassExportDefaultDeclaration > ;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsClassExportDefaultDeclaration {
    type Format = FormatOwnedWithRule < rome_js_syntax :: JsClassExportDefaultDeclaration , crate :: js :: declarations :: class_export_default_declaration :: FormatJsClassExportDefaultDeclaration > ;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsPrivateClassMemberName>
    for crate::js::objects::private_class_member_name::FormatJsPrivateClassMemberName
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsPrivateClassMemberName,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: objects :: private_class_member_name :: FormatJsPrivateClassMemberName as FormatNodeRule < rome_js_syntax :: JsPrivateClassMemberName >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsPrivateClassMemberName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsPrivateClassMemberName,
        crate::js::objects::private_class_member_name::FormatJsPrivateClassMemberName,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsPrivateClassMemberName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsPrivateClassMemberName,
        crate::js::objects::private_class_member_name::FormatJsPrivateClassMemberName,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsConstructorClassMember>
    for crate::js::classes::constructor_class_member::FormatJsConstructorClassMember
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsConstructorClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: classes :: constructor_class_member :: FormatJsConstructorClassMember as FormatNodeRule < rome_js_syntax :: JsConstructorClassMember >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsConstructorClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsConstructorClassMember,
        crate::js::classes::constructor_class_member::FormatJsConstructorClassMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsConstructorClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsConstructorClassMember,
        crate::js::classes::constructor_class_member::FormatJsConstructorClassMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule < rome_js_syntax :: JsStaticInitializationBlockClassMember > for crate :: js :: classes :: static_initialization_block_class_member :: FormatJsStaticInitializationBlockClassMember { type Context = JsFormatContext ; fn fmt (node : & rome_js_syntax :: JsStaticInitializationBlockClassMember , f : & mut JsFormatter) -> FormatResult < () > { < crate :: js :: classes :: static_initialization_block_class_member :: FormatJsStaticInitializationBlockClassMember as FormatNodeRule < rome_js_syntax :: JsStaticInitializationBlockClassMember >> :: fmt (node , f) } }
impl<'a> AsFormat<'a> for rome_js_syntax::JsStaticInitializationBlockClassMember {
    type Format = FormatRefWithRule < 'a , rome_js_syntax :: JsStaticInitializationBlockClassMember , crate :: js :: classes :: static_initialization_block_class_member :: FormatJsStaticInitializationBlockClassMember > ;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsStaticInitializationBlockClassMember {
    type Format = FormatOwnedWithRule < rome_js_syntax :: JsStaticInitializationBlockClassMember , crate :: js :: classes :: static_initialization_block_class_member :: FormatJsStaticInitializationBlockClassMember > ;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsPropertyClassMember>
    for crate::js::classes::property_class_member::FormatJsPropertyClassMember
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsPropertyClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::classes::property_class_member::FormatJsPropertyClassMember as FormatNodeRule<
            rome_js_syntax::JsPropertyClassMember,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsPropertyClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsPropertyClassMember,
        crate::js::classes::property_class_member::FormatJsPropertyClassMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsPropertyClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsPropertyClassMember,
        crate::js::classes::property_class_member::FormatJsPropertyClassMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsMethodClassMember>
    for crate::js::classes::method_class_member::FormatJsMethodClassMember
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsMethodClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::classes::method_class_member::FormatJsMethodClassMember as FormatNodeRule<
            rome_js_syntax::JsMethodClassMember,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsMethodClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsMethodClassMember,
        crate::js::classes::method_class_member::FormatJsMethodClassMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsMethodClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsMethodClassMember,
        crate::js::classes::method_class_member::FormatJsMethodClassMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsGetterClassMember>
    for crate::js::classes::getter_class_member::FormatJsGetterClassMember
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsGetterClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::classes::getter_class_member::FormatJsGetterClassMember as FormatNodeRule<
            rome_js_syntax::JsGetterClassMember,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsGetterClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsGetterClassMember,
        crate::js::classes::getter_class_member::FormatJsGetterClassMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsGetterClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsGetterClassMember,
        crate::js::classes::getter_class_member::FormatJsGetterClassMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsSetterClassMember>
    for crate::js::classes::setter_class_member::FormatJsSetterClassMember
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsSetterClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::classes::setter_class_member::FormatJsSetterClassMember as FormatNodeRule<
            rome_js_syntax::JsSetterClassMember,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsSetterClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsSetterClassMember,
        crate::js::classes::setter_class_member::FormatJsSetterClassMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsSetterClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsSetterClassMember,
        crate::js::classes::setter_class_member::FormatJsSetterClassMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule < rome_js_syntax :: TsConstructorSignatureClassMember > for crate :: ts :: classes :: constructor_signature_class_member :: FormatTsConstructorSignatureClassMember { type Context = JsFormatContext ; fn fmt (node : & rome_js_syntax :: TsConstructorSignatureClassMember , f : & mut JsFormatter) -> FormatResult < () > { < crate :: ts :: classes :: constructor_signature_class_member :: FormatTsConstructorSignatureClassMember as FormatNodeRule < rome_js_syntax :: TsConstructorSignatureClassMember >> :: fmt (node , f) } }
impl<'a> AsFormat<'a> for rome_js_syntax::TsConstructorSignatureClassMember {
    type Format = FormatRefWithRule < 'a , rome_js_syntax :: TsConstructorSignatureClassMember , crate :: ts :: classes :: constructor_signature_class_member :: FormatTsConstructorSignatureClassMember > ;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsConstructorSignatureClassMember {
    type Format = FormatOwnedWithRule < rome_js_syntax :: TsConstructorSignatureClassMember , crate :: ts :: classes :: constructor_signature_class_member :: FormatTsConstructorSignatureClassMember > ;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsPropertySignatureClassMember>
    for crate::ts::classes::property_signature_class_member::FormatTsPropertySignatureClassMember
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsPropertySignatureClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: classes :: property_signature_class_member :: FormatTsPropertySignatureClassMember as FormatNodeRule < rome_js_syntax :: TsPropertySignatureClassMember >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsPropertySignatureClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsPropertySignatureClassMember,
        crate::ts::classes::property_signature_class_member::FormatTsPropertySignatureClassMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsPropertySignatureClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsPropertySignatureClassMember,
        crate::ts::classes::property_signature_class_member::FormatTsPropertySignatureClassMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsMethodSignatureClassMember>
    for crate::ts::classes::method_signature_class_member::FormatTsMethodSignatureClassMember
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsMethodSignatureClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: classes :: method_signature_class_member :: FormatTsMethodSignatureClassMember as FormatNodeRule < rome_js_syntax :: TsMethodSignatureClassMember >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsMethodSignatureClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsMethodSignatureClassMember,
        crate::ts::classes::method_signature_class_member::FormatTsMethodSignatureClassMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsMethodSignatureClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsMethodSignatureClassMember,
        crate::ts::classes::method_signature_class_member::FormatTsMethodSignatureClassMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsGetterSignatureClassMember>
    for crate::ts::classes::getter_signature_class_member::FormatTsGetterSignatureClassMember
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsGetterSignatureClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: classes :: getter_signature_class_member :: FormatTsGetterSignatureClassMember as FormatNodeRule < rome_js_syntax :: TsGetterSignatureClassMember >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsGetterSignatureClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsGetterSignatureClassMember,
        crate::ts::classes::getter_signature_class_member::FormatTsGetterSignatureClassMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsGetterSignatureClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsGetterSignatureClassMember,
        crate::ts::classes::getter_signature_class_member::FormatTsGetterSignatureClassMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsSetterSignatureClassMember>
    for crate::ts::classes::setter_signature_class_member::FormatTsSetterSignatureClassMember
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsSetterSignatureClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: classes :: setter_signature_class_member :: FormatTsSetterSignatureClassMember as FormatNodeRule < rome_js_syntax :: TsSetterSignatureClassMember >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsSetterSignatureClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsSetterSignatureClassMember,
        crate::ts::classes::setter_signature_class_member::FormatTsSetterSignatureClassMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsSetterSignatureClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsSetterSignatureClassMember,
        crate::ts::classes::setter_signature_class_member::FormatTsSetterSignatureClassMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsIndexSignatureClassMember>
    for crate::ts::classes::index_signature_class_member::FormatTsIndexSignatureClassMember
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsIndexSignatureClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: classes :: index_signature_class_member :: FormatTsIndexSignatureClassMember as FormatNodeRule < rome_js_syntax :: TsIndexSignatureClassMember >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsIndexSignatureClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsIndexSignatureClassMember,
        crate::ts::classes::index_signature_class_member::FormatTsIndexSignatureClassMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsIndexSignatureClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsIndexSignatureClassMember,
        crate::ts::classes::index_signature_class_member::FormatTsIndexSignatureClassMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsEmptyClassMember>
    for crate::js::classes::empty_class_member::FormatJsEmptyClassMember
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsEmptyClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::classes::empty_class_member::FormatJsEmptyClassMember as FormatNodeRule<
            rome_js_syntax::JsEmptyClassMember,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsEmptyClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsEmptyClassMember,
        crate::js::classes::empty_class_member::FormatJsEmptyClassMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsEmptyClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsEmptyClassMember,
        crate::js::classes::empty_class_member::FormatJsEmptyClassMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsStaticModifier>
    for crate::js::auxiliary::static_modifier::FormatJsStaticModifier
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsStaticModifier, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::auxiliary::static_modifier::FormatJsStaticModifier as FormatNodeRule<
            rome_js_syntax::JsStaticModifier,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsStaticModifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsStaticModifier,
        crate::js::auxiliary::static_modifier::FormatJsStaticModifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsStaticModifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsStaticModifier,
        crate::js::auxiliary::static_modifier::FormatJsStaticModifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsDeclareModifier>
    for crate::ts::auxiliary::declare_modifier::FormatTsDeclareModifier
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsDeclareModifier, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::auxiliary::declare_modifier::FormatTsDeclareModifier as FormatNodeRule<
            rome_js_syntax::TsDeclareModifier,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsDeclareModifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsDeclareModifier,
        crate::ts::auxiliary::declare_modifier::FormatTsDeclareModifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsDeclareModifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsDeclareModifier,
        crate::ts::auxiliary::declare_modifier::FormatTsDeclareModifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsReadonlyModifier>
    for crate::ts::auxiliary::readonly_modifier::FormatTsReadonlyModifier
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsReadonlyModifier, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::auxiliary::readonly_modifier::FormatTsReadonlyModifier as FormatNodeRule<
            rome_js_syntax::TsReadonlyModifier,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsReadonlyModifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsReadonlyModifier,
        crate::ts::auxiliary::readonly_modifier::FormatTsReadonlyModifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsReadonlyModifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsReadonlyModifier,
        crate::ts::auxiliary::readonly_modifier::FormatTsReadonlyModifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsAbstractModifier>
    for crate::ts::auxiliary::abstract_modifier::FormatTsAbstractModifier
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsAbstractModifier, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::auxiliary::abstract_modifier::FormatTsAbstractModifier as FormatNodeRule<
            rome_js_syntax::TsAbstractModifier,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAbstractModifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAbstractModifier,
        crate::ts::auxiliary::abstract_modifier::FormatTsAbstractModifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsAbstractModifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAbstractModifier,
        crate::ts::auxiliary::abstract_modifier::FormatTsAbstractModifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsOverrideModifier>
    for crate::ts::auxiliary::override_modifier::FormatTsOverrideModifier
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsOverrideModifier, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::auxiliary::override_modifier::FormatTsOverrideModifier as FormatNodeRule<
            rome_js_syntax::TsOverrideModifier,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsOverrideModifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsOverrideModifier,
        crate::ts::auxiliary::override_modifier::FormatTsOverrideModifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsOverrideModifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsOverrideModifier,
        crate::ts::auxiliary::override_modifier::FormatTsOverrideModifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsAccessibilityModifier>
    for crate::ts::auxiliary::accessibility_modifier::FormatTsAccessibilityModifier
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsAccessibilityModifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: auxiliary :: accessibility_modifier :: FormatTsAccessibilityModifier as FormatNodeRule < rome_js_syntax :: TsAccessibilityModifier >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAccessibilityModifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAccessibilityModifier,
        crate::ts::auxiliary::accessibility_modifier::FormatTsAccessibilityModifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsAccessibilityModifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAccessibilityModifier,
        crate::ts::auxiliary::accessibility_modifier::FormatTsAccessibilityModifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsConstructorParameters>
    for crate::js::bindings::constructor_parameters::FormatJsConstructorParameters
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsConstructorParameters,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: bindings :: constructor_parameters :: FormatJsConstructorParameters as FormatNodeRule < rome_js_syntax :: JsConstructorParameters >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsConstructorParameters {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsConstructorParameters,
        crate::js::bindings::constructor_parameters::FormatJsConstructorParameters,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsConstructorParameters {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsConstructorParameters,
        crate::js::bindings::constructor_parameters::FormatJsConstructorParameters,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsRestParameter>
    for crate::js::bindings::rest_parameter::FormatJsRestParameter
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsRestParameter, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::bindings::rest_parameter::FormatJsRestParameter as FormatNodeRule<
            rome_js_syntax::JsRestParameter,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsRestParameter {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsRestParameter,
        crate::js::bindings::rest_parameter::FormatJsRestParameter,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsRestParameter {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsRestParameter,
        crate::js::bindings::rest_parameter::FormatJsRestParameter,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsPropertyParameter>
    for crate::ts::bindings::property_parameter::FormatTsPropertyParameter
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsPropertyParameter, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::bindings::property_parameter::FormatTsPropertyParameter as FormatNodeRule<
            rome_js_syntax::TsPropertyParameter,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsPropertyParameter {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsPropertyParameter,
        crate::ts::bindings::property_parameter::FormatTsPropertyParameter,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsPropertyParameter {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsPropertyParameter,
        crate::ts::bindings::property_parameter::FormatTsPropertyParameter,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsInitializerClause>
    for crate::js::auxiliary::initializer_clause::FormatJsInitializerClause
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsInitializerClause, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::auxiliary::initializer_clause::FormatJsInitializerClause as FormatNodeRule<
            rome_js_syntax::JsInitializerClause,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsInitializerClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsInitializerClause,
        crate::js::auxiliary::initializer_clause::FormatJsInitializerClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsInitializerClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsInitializerClause,
        crate::js::auxiliary::initializer_clause::FormatJsInitializerClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsOptionalPropertyAnnotation>
    for crate::ts::auxiliary::optional_property_annotation::FormatTsOptionalPropertyAnnotation
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsOptionalPropertyAnnotation,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: auxiliary :: optional_property_annotation :: FormatTsOptionalPropertyAnnotation as FormatNodeRule < rome_js_syntax :: TsOptionalPropertyAnnotation >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsOptionalPropertyAnnotation {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsOptionalPropertyAnnotation,
        crate::ts::auxiliary::optional_property_annotation::FormatTsOptionalPropertyAnnotation,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsOptionalPropertyAnnotation {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsOptionalPropertyAnnotation,
        crate::ts::auxiliary::optional_property_annotation::FormatTsOptionalPropertyAnnotation,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsDefinitePropertyAnnotation>
    for crate::ts::auxiliary::definite_property_annotation::FormatTsDefinitePropertyAnnotation
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsDefinitePropertyAnnotation,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: auxiliary :: definite_property_annotation :: FormatTsDefinitePropertyAnnotation as FormatNodeRule < rome_js_syntax :: TsDefinitePropertyAnnotation >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsDefinitePropertyAnnotation {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsDefinitePropertyAnnotation,
        crate::ts::auxiliary::definite_property_annotation::FormatTsDefinitePropertyAnnotation,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsDefinitePropertyAnnotation {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsDefinitePropertyAnnotation,
        crate::ts::auxiliary::definite_property_annotation::FormatTsDefinitePropertyAnnotation,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsIndexSignatureParameter>
    for crate::ts::bindings::index_signature_parameter::FormatTsIndexSignatureParameter
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsIndexSignatureParameter,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: bindings :: index_signature_parameter :: FormatTsIndexSignatureParameter as FormatNodeRule < rome_js_syntax :: TsIndexSignatureParameter >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsIndexSignatureParameter {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsIndexSignatureParameter,
        crate::ts::bindings::index_signature_parameter::FormatTsIndexSignatureParameter,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsIndexSignatureParameter {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsIndexSignatureParameter,
        crate::ts::bindings::index_signature_parameter::FormatTsIndexSignatureParameter,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsIdentifierAssignment>
    for crate::js::assignments::identifier_assignment::FormatJsIdentifierAssignment
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsIdentifierAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: assignments :: identifier_assignment :: FormatJsIdentifierAssignment as FormatNodeRule < rome_js_syntax :: JsIdentifierAssignment >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsIdentifierAssignment {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsIdentifierAssignment,
        crate::js::assignments::identifier_assignment::FormatJsIdentifierAssignment,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsIdentifierAssignment {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsIdentifierAssignment,
        crate::js::assignments::identifier_assignment::FormatJsIdentifierAssignment,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsStaticMemberAssignment>
    for crate::js::assignments::static_member_assignment::FormatJsStaticMemberAssignment
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsStaticMemberAssignment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: assignments :: static_member_assignment :: FormatJsStaticMemberAssignment as FormatNodeRule < rome_js_syntax :: JsStaticMemberAssignment >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsStaticMemberAssignment {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsStaticMemberAssignment,
        crate::js::assignments::static_member_assignment::FormatJsStaticMemberAssignment,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsStaticMemberAssignment {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsStaticMemberAssignment,
        crate::js::assignments::static_member_assignment::FormatJsStaticMemberAssignment,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsComputedMemberAssignment>
    for crate::js::assignments::computed_member_assignment::FormatJsComputedMemberAssignment
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsComputedMemberAssignment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: assignments :: computed_member_assignment :: FormatJsComputedMemberAssignment as FormatNodeRule < rome_js_syntax :: JsComputedMemberAssignment >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsComputedMemberAssignment {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsComputedMemberAssignment,
        crate::js::assignments::computed_member_assignment::FormatJsComputedMemberAssignment,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsComputedMemberAssignment {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsComputedMemberAssignment,
        crate::js::assignments::computed_member_assignment::FormatJsComputedMemberAssignment,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsParenthesizedAssignment>
    for crate::js::assignments::parenthesized_assignment::FormatJsParenthesizedAssignment
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsParenthesizedAssignment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: assignments :: parenthesized_assignment :: FormatJsParenthesizedAssignment as FormatNodeRule < rome_js_syntax :: JsParenthesizedAssignment >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsParenthesizedAssignment {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsParenthesizedAssignment,
        crate::js::assignments::parenthesized_assignment::FormatJsParenthesizedAssignment,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsParenthesizedAssignment {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsParenthesizedAssignment,
        crate::js::assignments::parenthesized_assignment::FormatJsParenthesizedAssignment,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsNonNullAssertionAssignment>
    for crate::ts::assignments::non_null_assertion_assignment::FormatTsNonNullAssertionAssignment
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsNonNullAssertionAssignment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: assignments :: non_null_assertion_assignment :: FormatTsNonNullAssertionAssignment as FormatNodeRule < rome_js_syntax :: TsNonNullAssertionAssignment >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsNonNullAssertionAssignment {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsNonNullAssertionAssignment,
        crate::ts::assignments::non_null_assertion_assignment::FormatTsNonNullAssertionAssignment,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsNonNullAssertionAssignment {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsNonNullAssertionAssignment,
        crate::ts::assignments::non_null_assertion_assignment::FormatTsNonNullAssertionAssignment,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsAsAssignment>
    for crate::ts::assignments::as_assignment::FormatTsAsAssignment
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsAsAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::assignments::as_assignment::FormatTsAsAssignment as FormatNodeRule<
            rome_js_syntax::TsAsAssignment,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAsAssignment {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAsAssignment,
        crate::ts::assignments::as_assignment::FormatTsAsAssignment,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsAsAssignment {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAsAssignment,
        crate::ts::assignments::as_assignment::FormatTsAsAssignment,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsTypeAssertionAssignment>
    for crate::ts::assignments::type_assertion_assignment::FormatTsTypeAssertionAssignment
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsTypeAssertionAssignment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: assignments :: type_assertion_assignment :: FormatTsTypeAssertionAssignment as FormatNodeRule < rome_js_syntax :: TsTypeAssertionAssignment >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeAssertionAssignment {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeAssertionAssignment,
        crate::ts::assignments::type_assertion_assignment::FormatTsTypeAssertionAssignment,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsTypeAssertionAssignment {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeAssertionAssignment,
        crate::ts::assignments::type_assertion_assignment::FormatTsTypeAssertionAssignment,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsAssignmentWithDefault>
    for crate::js::assignments::assignment_with_default::FormatJsAssignmentWithDefault
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsAssignmentWithDefault,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: assignments :: assignment_with_default :: FormatJsAssignmentWithDefault as FormatNodeRule < rome_js_syntax :: JsAssignmentWithDefault >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAssignmentWithDefault {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAssignmentWithDefault,
        crate::js::assignments::assignment_with_default::FormatJsAssignmentWithDefault,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAssignmentWithDefault {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAssignmentWithDefault,
        crate::js::assignments::assignment_with_default::FormatJsAssignmentWithDefault,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsArrayAssignmentPattern>
    for crate::js::assignments::array_assignment_pattern::FormatJsArrayAssignmentPattern
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsArrayAssignmentPattern,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: assignments :: array_assignment_pattern :: FormatJsArrayAssignmentPattern as FormatNodeRule < rome_js_syntax :: JsArrayAssignmentPattern >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsArrayAssignmentPattern {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsArrayAssignmentPattern,
        crate::js::assignments::array_assignment_pattern::FormatJsArrayAssignmentPattern,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsArrayAssignmentPattern {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsArrayAssignmentPattern,
        crate::js::assignments::array_assignment_pattern::FormatJsArrayAssignmentPattern,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsObjectAssignmentPattern>
    for crate::js::assignments::object_assignment_pattern::FormatJsObjectAssignmentPattern
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsObjectAssignmentPattern,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: assignments :: object_assignment_pattern :: FormatJsObjectAssignmentPattern as FormatNodeRule < rome_js_syntax :: JsObjectAssignmentPattern >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectAssignmentPattern {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsObjectAssignmentPattern,
        crate::js::assignments::object_assignment_pattern::FormatJsObjectAssignmentPattern,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsObjectAssignmentPattern {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsObjectAssignmentPattern,
        crate::js::assignments::object_assignment_pattern::FormatJsObjectAssignmentPattern,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule < rome_js_syntax :: JsArrayAssignmentPatternRestElement > for crate :: js :: assignments :: array_assignment_pattern_rest_element :: FormatJsArrayAssignmentPatternRestElement { type Context = JsFormatContext ; fn fmt (node : & rome_js_syntax :: JsArrayAssignmentPatternRestElement , f : & mut JsFormatter) -> FormatResult < () > { < crate :: js :: assignments :: array_assignment_pattern_rest_element :: FormatJsArrayAssignmentPatternRestElement as FormatNodeRule < rome_js_syntax :: JsArrayAssignmentPatternRestElement >> :: fmt (node , f) } }
impl<'a> AsFormat<'a> for rome_js_syntax::JsArrayAssignmentPatternRestElement {
    type Format = FormatRefWithRule < 'a , rome_js_syntax :: JsArrayAssignmentPatternRestElement , crate :: js :: assignments :: array_assignment_pattern_rest_element :: FormatJsArrayAssignmentPatternRestElement > ;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsArrayAssignmentPatternRestElement {
    type Format = FormatOwnedWithRule < rome_js_syntax :: JsArrayAssignmentPatternRestElement , crate :: js :: assignments :: array_assignment_pattern_rest_element :: FormatJsArrayAssignmentPatternRestElement > ;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule < rome_js_syntax :: JsObjectAssignmentPatternShorthandProperty > for crate :: js :: assignments :: object_assignment_pattern_shorthand_property :: FormatJsObjectAssignmentPatternShorthandProperty { type Context = JsFormatContext ; fn fmt (node : & rome_js_syntax :: JsObjectAssignmentPatternShorthandProperty , f : & mut JsFormatter) -> FormatResult < () > { < crate :: js :: assignments :: object_assignment_pattern_shorthand_property :: FormatJsObjectAssignmentPatternShorthandProperty as FormatNodeRule < rome_js_syntax :: JsObjectAssignmentPatternShorthandProperty >> :: fmt (node , f) } }
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectAssignmentPatternShorthandProperty {
    type Format = FormatRefWithRule < 'a , rome_js_syntax :: JsObjectAssignmentPatternShorthandProperty , crate :: js :: assignments :: object_assignment_pattern_shorthand_property :: FormatJsObjectAssignmentPatternShorthandProperty > ;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext>
    for rome_js_syntax::JsObjectAssignmentPatternShorthandProperty
{
    type Format = FormatOwnedWithRule < rome_js_syntax :: JsObjectAssignmentPatternShorthandProperty , crate :: js :: assignments :: object_assignment_pattern_shorthand_property :: FormatJsObjectAssignmentPatternShorthandProperty > ;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule < rome_js_syntax :: JsObjectAssignmentPatternProperty > for crate :: js :: assignments :: object_assignment_pattern_property :: FormatJsObjectAssignmentPatternProperty { type Context = JsFormatContext ; fn fmt (node : & rome_js_syntax :: JsObjectAssignmentPatternProperty , f : & mut JsFormatter) -> FormatResult < () > { < crate :: js :: assignments :: object_assignment_pattern_property :: FormatJsObjectAssignmentPatternProperty as FormatNodeRule < rome_js_syntax :: JsObjectAssignmentPatternProperty >> :: fmt (node , f) } }
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectAssignmentPatternProperty {
    type Format = FormatRefWithRule < 'a , rome_js_syntax :: JsObjectAssignmentPatternProperty , crate :: js :: assignments :: object_assignment_pattern_property :: FormatJsObjectAssignmentPatternProperty > ;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsObjectAssignmentPatternProperty {
    type Format = FormatOwnedWithRule < rome_js_syntax :: JsObjectAssignmentPatternProperty , crate :: js :: assignments :: object_assignment_pattern_property :: FormatJsObjectAssignmentPatternProperty > ;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsObjectAssignmentPatternRest>
    for crate::js::assignments::object_assignment_pattern_rest::FormatJsObjectAssignmentPatternRest
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsObjectAssignmentPatternRest,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: assignments :: object_assignment_pattern_rest :: FormatJsObjectAssignmentPatternRest as FormatNodeRule < rome_js_syntax :: JsObjectAssignmentPatternRest >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectAssignmentPatternRest {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsObjectAssignmentPatternRest,
        crate::js::assignments::object_assignment_pattern_rest::FormatJsObjectAssignmentPatternRest,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsObjectAssignmentPatternRest {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsObjectAssignmentPatternRest,
        crate::js::assignments::object_assignment_pattern_rest::FormatJsObjectAssignmentPatternRest,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsIdentifierBinding>
    for crate::js::bindings::identifier_binding::FormatJsIdentifierBinding
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsIdentifierBinding, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::bindings::identifier_binding::FormatJsIdentifierBinding as FormatNodeRule<
            rome_js_syntax::JsIdentifierBinding,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsIdentifierBinding {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsIdentifierBinding,
        crate::js::bindings::identifier_binding::FormatJsIdentifierBinding,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsIdentifierBinding {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsIdentifierBinding,
        crate::js::bindings::identifier_binding::FormatJsIdentifierBinding,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsBindingPatternWithDefault>
    for crate::js::bindings::binding_pattern_with_default::FormatJsBindingPatternWithDefault
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsBindingPatternWithDefault,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: bindings :: binding_pattern_with_default :: FormatJsBindingPatternWithDefault as FormatNodeRule < rome_js_syntax :: JsBindingPatternWithDefault >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsBindingPatternWithDefault {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsBindingPatternWithDefault,
        crate::js::bindings::binding_pattern_with_default::FormatJsBindingPatternWithDefault,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsBindingPatternWithDefault {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsBindingPatternWithDefault,
        crate::js::bindings::binding_pattern_with_default::FormatJsBindingPatternWithDefault,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsArrayBindingPattern>
    for crate::js::bindings::array_binding_pattern::FormatJsArrayBindingPattern
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsArrayBindingPattern, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: bindings :: array_binding_pattern :: FormatJsArrayBindingPattern as FormatNodeRule < rome_js_syntax :: JsArrayBindingPattern >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsArrayBindingPattern {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsArrayBindingPattern,
        crate::js::bindings::array_binding_pattern::FormatJsArrayBindingPattern,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsArrayBindingPattern {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsArrayBindingPattern,
        crate::js::bindings::array_binding_pattern::FormatJsArrayBindingPattern,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsObjectBindingPattern>
    for crate::js::bindings::object_binding_pattern::FormatJsObjectBindingPattern
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsObjectBindingPattern, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: bindings :: object_binding_pattern :: FormatJsObjectBindingPattern as FormatNodeRule < rome_js_syntax :: JsObjectBindingPattern >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectBindingPattern {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsObjectBindingPattern,
        crate::js::bindings::object_binding_pattern::FormatJsObjectBindingPattern,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsObjectBindingPattern {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsObjectBindingPattern,
        crate::js::bindings::object_binding_pattern::FormatJsObjectBindingPattern,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule < rome_js_syntax :: JsArrayBindingPatternRestElement > for crate :: js :: bindings :: array_binding_pattern_rest_element :: FormatJsArrayBindingPatternRestElement { type Context = JsFormatContext ; fn fmt (node : & rome_js_syntax :: JsArrayBindingPatternRestElement , f : & mut JsFormatter) -> FormatResult < () > { < crate :: js :: bindings :: array_binding_pattern_rest_element :: FormatJsArrayBindingPatternRestElement as FormatNodeRule < rome_js_syntax :: JsArrayBindingPatternRestElement >> :: fmt (node , f) } }
impl<'a> AsFormat<'a> for rome_js_syntax::JsArrayBindingPatternRestElement {
    type Format = FormatRefWithRule < 'a , rome_js_syntax :: JsArrayBindingPatternRestElement , crate :: js :: bindings :: array_binding_pattern_rest_element :: FormatJsArrayBindingPatternRestElement > ;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsArrayBindingPatternRestElement {
    type Format = FormatOwnedWithRule < rome_js_syntax :: JsArrayBindingPatternRestElement , crate :: js :: bindings :: array_binding_pattern_rest_element :: FormatJsArrayBindingPatternRestElement > ;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsObjectBindingPatternProperty>
    for crate::js::bindings::object_binding_pattern_property::FormatJsObjectBindingPatternProperty
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsObjectBindingPatternProperty,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: bindings :: object_binding_pattern_property :: FormatJsObjectBindingPatternProperty as FormatNodeRule < rome_js_syntax :: JsObjectBindingPatternProperty >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectBindingPatternProperty {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsObjectBindingPatternProperty,
        crate::js::bindings::object_binding_pattern_property::FormatJsObjectBindingPatternProperty,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsObjectBindingPatternProperty {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsObjectBindingPatternProperty,
        crate::js::bindings::object_binding_pattern_property::FormatJsObjectBindingPatternProperty,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsObjectBindingPatternRest>
    for crate::js::bindings::object_binding_pattern_rest::FormatJsObjectBindingPatternRest
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsObjectBindingPatternRest,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: bindings :: object_binding_pattern_rest :: FormatJsObjectBindingPatternRest as FormatNodeRule < rome_js_syntax :: JsObjectBindingPatternRest >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectBindingPatternRest {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsObjectBindingPatternRest,
        crate::js::bindings::object_binding_pattern_rest::FormatJsObjectBindingPatternRest,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsObjectBindingPatternRest {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsObjectBindingPatternRest,
        crate::js::bindings::object_binding_pattern_rest::FormatJsObjectBindingPatternRest,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule < rome_js_syntax :: JsObjectBindingPatternShorthandProperty > for crate :: js :: bindings :: object_binding_pattern_shorthand_property :: FormatJsObjectBindingPatternShorthandProperty { type Context = JsFormatContext ; fn fmt (node : & rome_js_syntax :: JsObjectBindingPatternShorthandProperty , f : & mut JsFormatter) -> FormatResult < () > { < crate :: js :: bindings :: object_binding_pattern_shorthand_property :: FormatJsObjectBindingPatternShorthandProperty as FormatNodeRule < rome_js_syntax :: JsObjectBindingPatternShorthandProperty >> :: fmt (node , f) } }
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectBindingPatternShorthandProperty {
    type Format = FormatRefWithRule < 'a , rome_js_syntax :: JsObjectBindingPatternShorthandProperty , crate :: js :: bindings :: object_binding_pattern_shorthand_property :: FormatJsObjectBindingPatternShorthandProperty > ;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext>
    for rome_js_syntax::JsObjectBindingPatternShorthandProperty
{
    type Format = FormatOwnedWithRule < rome_js_syntax :: JsObjectBindingPatternShorthandProperty , crate :: js :: bindings :: object_binding_pattern_shorthand_property :: FormatJsObjectBindingPatternShorthandProperty > ;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsStringLiteralExpression>
    for crate::js::expressions::string_literal_expression::FormatJsStringLiteralExpression
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsStringLiteralExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: expressions :: string_literal_expression :: FormatJsStringLiteralExpression as FormatNodeRule < rome_js_syntax :: JsStringLiteralExpression >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsStringLiteralExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsStringLiteralExpression,
        crate::js::expressions::string_literal_expression::FormatJsStringLiteralExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsStringLiteralExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsStringLiteralExpression,
        crate::js::expressions::string_literal_expression::FormatJsStringLiteralExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsNumberLiteralExpression>
    for crate::js::expressions::number_literal_expression::FormatJsNumberLiteralExpression
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsNumberLiteralExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: expressions :: number_literal_expression :: FormatJsNumberLiteralExpression as FormatNodeRule < rome_js_syntax :: JsNumberLiteralExpression >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsNumberLiteralExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsNumberLiteralExpression,
        crate::js::expressions::number_literal_expression::FormatJsNumberLiteralExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsNumberLiteralExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsNumberLiteralExpression,
        crate::js::expressions::number_literal_expression::FormatJsNumberLiteralExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsBigIntLiteralExpression>
    for crate::js::expressions::big_int_literal_expression::FormatJsBigIntLiteralExpression
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsBigIntLiteralExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: expressions :: big_int_literal_expression :: FormatJsBigIntLiteralExpression as FormatNodeRule < rome_js_syntax :: JsBigIntLiteralExpression >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsBigIntLiteralExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsBigIntLiteralExpression,
        crate::js::expressions::big_int_literal_expression::FormatJsBigIntLiteralExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsBigIntLiteralExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsBigIntLiteralExpression,
        crate::js::expressions::big_int_literal_expression::FormatJsBigIntLiteralExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsBooleanLiteralExpression>
    for crate::js::expressions::boolean_literal_expression::FormatJsBooleanLiteralExpression
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsBooleanLiteralExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: expressions :: boolean_literal_expression :: FormatJsBooleanLiteralExpression as FormatNodeRule < rome_js_syntax :: JsBooleanLiteralExpression >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsBooleanLiteralExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsBooleanLiteralExpression,
        crate::js::expressions::boolean_literal_expression::FormatJsBooleanLiteralExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsBooleanLiteralExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsBooleanLiteralExpression,
        crate::js::expressions::boolean_literal_expression::FormatJsBooleanLiteralExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsNullLiteralExpression>
    for crate::js::expressions::null_literal_expression::FormatJsNullLiteralExpression
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsNullLiteralExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: expressions :: null_literal_expression :: FormatJsNullLiteralExpression as FormatNodeRule < rome_js_syntax :: JsNullLiteralExpression >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsNullLiteralExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsNullLiteralExpression,
        crate::js::expressions::null_literal_expression::FormatJsNullLiteralExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsNullLiteralExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsNullLiteralExpression,
        crate::js::expressions::null_literal_expression::FormatJsNullLiteralExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsRegexLiteralExpression>
    for crate::js::expressions::regex_literal_expression::FormatJsRegexLiteralExpression
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsRegexLiteralExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: expressions :: regex_literal_expression :: FormatJsRegexLiteralExpression as FormatNodeRule < rome_js_syntax :: JsRegexLiteralExpression >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsRegexLiteralExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsRegexLiteralExpression,
        crate::js::expressions::regex_literal_expression::FormatJsRegexLiteralExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsRegexLiteralExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsRegexLiteralExpression,
        crate::js::expressions::regex_literal_expression::FormatJsRegexLiteralExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsVariableDeclarationClause>
    for crate::js::auxiliary::variable_declaration_clause::FormatJsVariableDeclarationClause
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsVariableDeclarationClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: auxiliary :: variable_declaration_clause :: FormatJsVariableDeclarationClause as FormatNodeRule < rome_js_syntax :: JsVariableDeclarationClause >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsVariableDeclarationClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsVariableDeclarationClause,
        crate::js::auxiliary::variable_declaration_clause::FormatJsVariableDeclarationClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsVariableDeclarationClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsVariableDeclarationClause,
        crate::js::auxiliary::variable_declaration_clause::FormatJsVariableDeclarationClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsDefiniteVariableAnnotation>
    for crate::ts::auxiliary::definite_variable_annotation::FormatTsDefiniteVariableAnnotation
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsDefiniteVariableAnnotation,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: auxiliary :: definite_variable_annotation :: FormatTsDefiniteVariableAnnotation as FormatNodeRule < rome_js_syntax :: TsDefiniteVariableAnnotation >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsDefiniteVariableAnnotation {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsDefiniteVariableAnnotation,
        crate::ts::auxiliary::definite_variable_annotation::FormatTsDefiniteVariableAnnotation,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsDefiniteVariableAnnotation {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsDefiniteVariableAnnotation,
        crate::ts::auxiliary::definite_variable_annotation::FormatTsDefiniteVariableAnnotation,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsExport> for crate::js::module::export::FormatJsExport {
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsExport, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::module::export::FormatJsExport as FormatNodeRule<rome_js_syntax::JsExport>>::fmt(
            node, f,
        )
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExport {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsExport, crate::js::module::export::FormatJsExport>;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsExport {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsExport, crate::js::module::export::FormatJsExport>;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsImport> for crate::js::module::import::FormatJsImport {
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsImport, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::module::import::FormatJsImport as FormatNodeRule<rome_js_syntax::JsImport>>::fmt(
            node, f,
        )
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsImport {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsImport, crate::js::module::import::FormatJsImport>;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsImport {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsImport, crate::js::module::import::FormatJsImport>;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsImportBareClause>
    for crate::js::module::import_bare_clause::FormatJsImportBareClause
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsImportBareClause, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::module::import_bare_clause::FormatJsImportBareClause as FormatNodeRule<
            rome_js_syntax::JsImportBareClause,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsImportBareClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsImportBareClause,
        crate::js::module::import_bare_clause::FormatJsImportBareClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsImportBareClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsImportBareClause,
        crate::js::module::import_bare_clause::FormatJsImportBareClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsImportNamedClause>
    for crate::js::module::import_named_clause::FormatJsImportNamedClause
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsImportNamedClause, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::module::import_named_clause::FormatJsImportNamedClause as FormatNodeRule<
            rome_js_syntax::JsImportNamedClause,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsImportNamedClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsImportNamedClause,
        crate::js::module::import_named_clause::FormatJsImportNamedClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsImportNamedClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsImportNamedClause,
        crate::js::module::import_named_clause::FormatJsImportNamedClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsImportDefaultClause>
    for crate::js::module::import_default_clause::FormatJsImportDefaultClause
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsImportDefaultClause, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::module::import_default_clause::FormatJsImportDefaultClause as FormatNodeRule<
            rome_js_syntax::JsImportDefaultClause,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsImportDefaultClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsImportDefaultClause,
        crate::js::module::import_default_clause::FormatJsImportDefaultClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsImportDefaultClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsImportDefaultClause,
        crate::js::module::import_default_clause::FormatJsImportDefaultClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsImportNamespaceClause>
    for crate::js::module::import_namespace_clause::FormatJsImportNamespaceClause
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsImportNamespaceClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: module :: import_namespace_clause :: FormatJsImportNamespaceClause as FormatNodeRule < rome_js_syntax :: JsImportNamespaceClause >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsImportNamespaceClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsImportNamespaceClause,
        crate::js::module::import_namespace_clause::FormatJsImportNamespaceClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsImportNamespaceClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsImportNamespaceClause,
        crate::js::module::import_namespace_clause::FormatJsImportNamespaceClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsModuleSource>
    for crate::js::module::module_source::FormatJsModuleSource
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsModuleSource, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::module::module_source::FormatJsModuleSource as FormatNodeRule<
            rome_js_syntax::JsModuleSource,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsModuleSource {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsModuleSource,
        crate::js::module::module_source::FormatJsModuleSource,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsModuleSource {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsModuleSource,
        crate::js::module::module_source::FormatJsModuleSource,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsImportAssertion>
    for crate::js::module::import_assertion::FormatJsImportAssertion
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsImportAssertion, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::module::import_assertion::FormatJsImportAssertion as FormatNodeRule<
            rome_js_syntax::JsImportAssertion,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsImportAssertion {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsImportAssertion,
        crate::js::module::import_assertion::FormatJsImportAssertion,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsImportAssertion {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsImportAssertion,
        crate::js::module::import_assertion::FormatJsImportAssertion,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsDefaultImportSpecifier>
    for crate::js::module::default_import_specifier::FormatJsDefaultImportSpecifier
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsDefaultImportSpecifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: module :: default_import_specifier :: FormatJsDefaultImportSpecifier as FormatNodeRule < rome_js_syntax :: JsDefaultImportSpecifier >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsDefaultImportSpecifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsDefaultImportSpecifier,
        crate::js::module::default_import_specifier::FormatJsDefaultImportSpecifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsDefaultImportSpecifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsDefaultImportSpecifier,
        crate::js::module::default_import_specifier::FormatJsDefaultImportSpecifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsNamedImportSpecifiers>
    for crate::js::module::named_import_specifiers::FormatJsNamedImportSpecifiers
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsNamedImportSpecifiers,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: module :: named_import_specifiers :: FormatJsNamedImportSpecifiers as FormatNodeRule < rome_js_syntax :: JsNamedImportSpecifiers >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsNamedImportSpecifiers {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsNamedImportSpecifiers,
        crate::js::module::named_import_specifiers::FormatJsNamedImportSpecifiers,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsNamedImportSpecifiers {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsNamedImportSpecifiers,
        crate::js::module::named_import_specifiers::FormatJsNamedImportSpecifiers,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsNamespaceImportSpecifier>
    for crate::js::module::namespace_import_specifier::FormatJsNamespaceImportSpecifier
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsNamespaceImportSpecifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: module :: namespace_import_specifier :: FormatJsNamespaceImportSpecifier as FormatNodeRule < rome_js_syntax :: JsNamespaceImportSpecifier >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsNamespaceImportSpecifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsNamespaceImportSpecifier,
        crate::js::module::namespace_import_specifier::FormatJsNamespaceImportSpecifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsNamespaceImportSpecifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsNamespaceImportSpecifier,
        crate::js::module::namespace_import_specifier::FormatJsNamespaceImportSpecifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsShorthandNamedImportSpecifier>
    for crate::js::module::shorthand_named_import_specifier::FormatJsShorthandNamedImportSpecifier
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsShorthandNamedImportSpecifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: module :: shorthand_named_import_specifier :: FormatJsShorthandNamedImportSpecifier as FormatNodeRule < rome_js_syntax :: JsShorthandNamedImportSpecifier >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsShorthandNamedImportSpecifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsShorthandNamedImportSpecifier,
        crate::js::module::shorthand_named_import_specifier::FormatJsShorthandNamedImportSpecifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsShorthandNamedImportSpecifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsShorthandNamedImportSpecifier,
        crate::js::module::shorthand_named_import_specifier::FormatJsShorthandNamedImportSpecifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsNamedImportSpecifier>
    for crate::js::module::named_import_specifier::FormatJsNamedImportSpecifier
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsNamedImportSpecifier, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: module :: named_import_specifier :: FormatJsNamedImportSpecifier as FormatNodeRule < rome_js_syntax :: JsNamedImportSpecifier >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsNamedImportSpecifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsNamedImportSpecifier,
        crate::js::module::named_import_specifier::FormatJsNamedImportSpecifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsNamedImportSpecifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsNamedImportSpecifier,
        crate::js::module::named_import_specifier::FormatJsNamedImportSpecifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsLiteralExportName>
    for crate::js::module::literal_export_name::FormatJsLiteralExportName
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsLiteralExportName, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::module::literal_export_name::FormatJsLiteralExportName as FormatNodeRule<
            rome_js_syntax::JsLiteralExportName,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsLiteralExportName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsLiteralExportName,
        crate::js::module::literal_export_name::FormatJsLiteralExportName,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsLiteralExportName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsLiteralExportName,
        crate::js::module::literal_export_name::FormatJsLiteralExportName,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsImportAssertionEntry>
    for crate::js::module::import_assertion_entry::FormatJsImportAssertionEntry
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsImportAssertionEntry, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: module :: import_assertion_entry :: FormatJsImportAssertionEntry as FormatNodeRule < rome_js_syntax :: JsImportAssertionEntry >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsImportAssertionEntry {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsImportAssertionEntry,
        crate::js::module::import_assertion_entry::FormatJsImportAssertionEntry,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsImportAssertionEntry {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsImportAssertionEntry,
        crate::js::module::import_assertion_entry::FormatJsImportAssertionEntry,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsExportDefaultDeclarationClause>
    for crate::js::module::export_default_declaration_clause::FormatJsExportDefaultDeclarationClause
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsExportDefaultDeclarationClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: module :: export_default_declaration_clause :: FormatJsExportDefaultDeclarationClause as FormatNodeRule < rome_js_syntax :: JsExportDefaultDeclarationClause >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExportDefaultDeclarationClause {
    type Format = FormatRefWithRule < 'a , rome_js_syntax :: JsExportDefaultDeclarationClause , crate :: js :: module :: export_default_declaration_clause :: FormatJsExportDefaultDeclarationClause > ;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsExportDefaultDeclarationClause {
    type Format = FormatOwnedWithRule < rome_js_syntax :: JsExportDefaultDeclarationClause , crate :: js :: module :: export_default_declaration_clause :: FormatJsExportDefaultDeclarationClause > ;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsExportDefaultExpressionClause>
    for crate::js::module::export_default_expression_clause::FormatJsExportDefaultExpressionClause
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsExportDefaultExpressionClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: module :: export_default_expression_clause :: FormatJsExportDefaultExpressionClause as FormatNodeRule < rome_js_syntax :: JsExportDefaultExpressionClause >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExportDefaultExpressionClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExportDefaultExpressionClause,
        crate::js::module::export_default_expression_clause::FormatJsExportDefaultExpressionClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsExportDefaultExpressionClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExportDefaultExpressionClause,
        crate::js::module::export_default_expression_clause::FormatJsExportDefaultExpressionClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsExportNamedClause>
    for crate::js::module::export_named_clause::FormatJsExportNamedClause
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsExportNamedClause, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::module::export_named_clause::FormatJsExportNamedClause as FormatNodeRule<
            rome_js_syntax::JsExportNamedClause,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExportNamedClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExportNamedClause,
        crate::js::module::export_named_clause::FormatJsExportNamedClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsExportNamedClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExportNamedClause,
        crate::js::module::export_named_clause::FormatJsExportNamedClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsExportFromClause>
    for crate::js::module::export_from_clause::FormatJsExportFromClause
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsExportFromClause, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::module::export_from_clause::FormatJsExportFromClause as FormatNodeRule<
            rome_js_syntax::JsExportFromClause,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExportFromClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExportFromClause,
        crate::js::module::export_from_clause::FormatJsExportFromClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsExportFromClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExportFromClause,
        crate::js::module::export_from_clause::FormatJsExportFromClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsExportNamedFromClause>
    for crate::js::module::export_named_from_clause::FormatJsExportNamedFromClause
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsExportNamedFromClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: module :: export_named_from_clause :: FormatJsExportNamedFromClause as FormatNodeRule < rome_js_syntax :: JsExportNamedFromClause >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExportNamedFromClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExportNamedFromClause,
        crate::js::module::export_named_from_clause::FormatJsExportNamedFromClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsExportNamedFromClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExportNamedFromClause,
        crate::js::module::export_named_from_clause::FormatJsExportNamedFromClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsExportAsNamespaceClause>
    for crate::ts::module::export_as_namespace_clause::FormatTsExportAsNamespaceClause
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsExportAsNamespaceClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: module :: export_as_namespace_clause :: FormatTsExportAsNamespaceClause as FormatNodeRule < rome_js_syntax :: TsExportAsNamespaceClause >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsExportAsNamespaceClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsExportAsNamespaceClause,
        crate::ts::module::export_as_namespace_clause::FormatTsExportAsNamespaceClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsExportAsNamespaceClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsExportAsNamespaceClause,
        crate::ts::module::export_as_namespace_clause::FormatTsExportAsNamespaceClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsExportAssignmentClause>
    for crate::ts::module::export_assignment_clause::FormatTsExportAssignmentClause
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsExportAssignmentClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: module :: export_assignment_clause :: FormatTsExportAssignmentClause as FormatNodeRule < rome_js_syntax :: TsExportAssignmentClause >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsExportAssignmentClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsExportAssignmentClause,
        crate::ts::module::export_assignment_clause::FormatTsExportAssignmentClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsExportAssignmentClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsExportAssignmentClause,
        crate::ts::module::export_assignment_clause::FormatTsExportAssignmentClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsExportDeclareClause>
    for crate::ts::module::export_declare_clause::FormatTsExportDeclareClause
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsExportDeclareClause, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::module::export_declare_clause::FormatTsExportDeclareClause as FormatNodeRule<
            rome_js_syntax::TsExportDeclareClause,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsExportDeclareClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsExportDeclareClause,
        crate::ts::module::export_declare_clause::FormatTsExportDeclareClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsExportDeclareClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsExportDeclareClause,
        crate::ts::module::export_declare_clause::FormatTsExportDeclareClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule < rome_js_syntax :: JsFunctionExportDefaultDeclaration > for crate :: js :: declarations :: function_export_default_declaration :: FormatJsFunctionExportDefaultDeclaration { type Context = JsFormatContext ; fn fmt (node : & rome_js_syntax :: JsFunctionExportDefaultDeclaration , f : & mut JsFormatter) -> FormatResult < () > { < crate :: js :: declarations :: function_export_default_declaration :: FormatJsFunctionExportDefaultDeclaration as FormatNodeRule < rome_js_syntax :: JsFunctionExportDefaultDeclaration >> :: fmt (node , f) } }
impl<'a> AsFormat<'a> for rome_js_syntax::JsFunctionExportDefaultDeclaration {
    type Format = FormatRefWithRule < 'a , rome_js_syntax :: JsFunctionExportDefaultDeclaration , crate :: js :: declarations :: function_export_default_declaration :: FormatJsFunctionExportDefaultDeclaration > ;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsFunctionExportDefaultDeclaration {
    type Format = FormatOwnedWithRule < rome_js_syntax :: JsFunctionExportDefaultDeclaration , crate :: js :: declarations :: function_export_default_declaration :: FormatJsFunctionExportDefaultDeclaration > ;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsExportNamedShorthandSpecifier>
    for crate::js::module::export_named_shorthand_specifier::FormatJsExportNamedShorthandSpecifier
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsExportNamedShorthandSpecifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: module :: export_named_shorthand_specifier :: FormatJsExportNamedShorthandSpecifier as FormatNodeRule < rome_js_syntax :: JsExportNamedShorthandSpecifier >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExportNamedShorthandSpecifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExportNamedShorthandSpecifier,
        crate::js::module::export_named_shorthand_specifier::FormatJsExportNamedShorthandSpecifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsExportNamedShorthandSpecifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExportNamedShorthandSpecifier,
        crate::js::module::export_named_shorthand_specifier::FormatJsExportNamedShorthandSpecifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsExportNamedSpecifier>
    for crate::js::module::export_named_specifier::FormatJsExportNamedSpecifier
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsExportNamedSpecifier, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: js :: module :: export_named_specifier :: FormatJsExportNamedSpecifier as FormatNodeRule < rome_js_syntax :: JsExportNamedSpecifier >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExportNamedSpecifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExportNamedSpecifier,
        crate::js::module::export_named_specifier::FormatJsExportNamedSpecifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsExportNamedSpecifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExportNamedSpecifier,
        crate::js::module::export_named_specifier::FormatJsExportNamedSpecifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsExportAsClause>
    for crate::js::module::export_as_clause::FormatJsExportAsClause
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsExportAsClause, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::module::export_as_clause::FormatJsExportAsClause as FormatNodeRule<
            rome_js_syntax::JsExportAsClause,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExportAsClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExportAsClause,
        crate::js::module::export_as_clause::FormatJsExportAsClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsExportAsClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExportAsClause,
        crate::js::module::export_as_clause::FormatJsExportAsClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsExportNamedFromSpecifier>
    for crate::js::module::export_named_from_specifier::FormatJsExportNamedFromSpecifier
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsExportNamedFromSpecifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: module :: export_named_from_specifier :: FormatJsExportNamedFromSpecifier as FormatNodeRule < rome_js_syntax :: JsExportNamedFromSpecifier >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExportNamedFromSpecifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExportNamedFromSpecifier,
        crate::js::module::export_named_from_specifier::FormatJsExportNamedFromSpecifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsExportNamedFromSpecifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExportNamedFromSpecifier,
        crate::js::module::export_named_from_specifier::FormatJsExportNamedFromSpecifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsName> for crate::js::auxiliary::name::FormatJsName {
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsName, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::auxiliary::name::FormatJsName as FormatNodeRule<rome_js_syntax::JsName>>::fmt(
            node, f,
        )
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsName {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsName, crate::js::auxiliary::name::FormatJsName>;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsName {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsName, crate::js::auxiliary::name::FormatJsName>;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsFormalParameter>
    for crate::js::bindings::formal_parameter::FormatJsFormalParameter
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsFormalParameter, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::bindings::formal_parameter::FormatJsFormalParameter as FormatNodeRule<
            rome_js_syntax::JsFormalParameter,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsFormalParameter {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsFormalParameter,
        crate::js::bindings::formal_parameter::FormatJsFormalParameter,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsFormalParameter {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsFormalParameter,
        crate::js::bindings::formal_parameter::FormatJsFormalParameter,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsThisParameter>
    for crate::ts::bindings::this_parameter::FormatTsThisParameter
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsThisParameter, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::bindings::this_parameter::FormatTsThisParameter as FormatNodeRule<
            rome_js_syntax::TsThisParameter,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsThisParameter {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsThisParameter,
        crate::ts::bindings::this_parameter::FormatTsThisParameter,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsThisParameter {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsThisParameter,
        crate::ts::bindings::this_parameter::FormatTsThisParameter,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsAnyType> for crate::ts::types::any_type::FormatTsAnyType {
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsAnyType, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: ts :: types :: any_type :: FormatTsAnyType as FormatNodeRule < rome_js_syntax :: TsAnyType >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyType,
        crate::ts::types::any_type::FormatTsAnyType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsAnyType {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::TsAnyType, crate::ts::types::any_type::FormatTsAnyType>;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsUnknownType>
    for crate::ts::types::unknown_type::FormatTsUnknownType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsUnknownType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::unknown_type::FormatTsUnknownType as FormatNodeRule<
            rome_js_syntax::TsUnknownType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsUnknownType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsUnknownType,
        crate::ts::types::unknown_type::FormatTsUnknownType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsUnknownType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsUnknownType,
        crate::ts::types::unknown_type::FormatTsUnknownType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsNumberType>
    for crate::ts::types::number_type::FormatTsNumberType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsNumberType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::number_type::FormatTsNumberType as FormatNodeRule<
            rome_js_syntax::TsNumberType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsNumberType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsNumberType,
        crate::ts::types::number_type::FormatTsNumberType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsNumberType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsNumberType,
        crate::ts::types::number_type::FormatTsNumberType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsBooleanType>
    for crate::ts::types::boolean_type::FormatTsBooleanType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsBooleanType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::boolean_type::FormatTsBooleanType as FormatNodeRule<
            rome_js_syntax::TsBooleanType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsBooleanType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsBooleanType,
        crate::ts::types::boolean_type::FormatTsBooleanType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsBooleanType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsBooleanType,
        crate::ts::types::boolean_type::FormatTsBooleanType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsBigintType>
    for crate::ts::types::bigint_type::FormatTsBigintType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsBigintType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::bigint_type::FormatTsBigintType as FormatNodeRule<
            rome_js_syntax::TsBigintType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsBigintType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsBigintType,
        crate::ts::types::bigint_type::FormatTsBigintType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsBigintType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsBigintType,
        crate::ts::types::bigint_type::FormatTsBigintType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsStringType>
    for crate::ts::types::string_type::FormatTsStringType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsStringType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::string_type::FormatTsStringType as FormatNodeRule<
            rome_js_syntax::TsStringType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsStringType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsStringType,
        crate::ts::types::string_type::FormatTsStringType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsStringType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsStringType,
        crate::ts::types::string_type::FormatTsStringType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsSymbolType>
    for crate::ts::types::symbol_type::FormatTsSymbolType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsSymbolType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::symbol_type::FormatTsSymbolType as FormatNodeRule<
            rome_js_syntax::TsSymbolType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsSymbolType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsSymbolType,
        crate::ts::types::symbol_type::FormatTsSymbolType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsSymbolType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsSymbolType,
        crate::ts::types::symbol_type::FormatTsSymbolType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsVoidType> for crate::ts::types::void_type::FormatTsVoidType {
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsVoidType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::void_type::FormatTsVoidType as FormatNodeRule<
            rome_js_syntax::TsVoidType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsVoidType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsVoidType,
        crate::ts::types::void_type::FormatTsVoidType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsVoidType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsVoidType,
        crate::ts::types::void_type::FormatTsVoidType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsUndefinedType>
    for crate::ts::types::undefined_type::FormatTsUndefinedType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsUndefinedType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::undefined_type::FormatTsUndefinedType as FormatNodeRule<
            rome_js_syntax::TsUndefinedType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsUndefinedType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsUndefinedType,
        crate::ts::types::undefined_type::FormatTsUndefinedType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsUndefinedType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsUndefinedType,
        crate::ts::types::undefined_type::FormatTsUndefinedType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsNeverType> for crate::ts::types::never_type::FormatTsNeverType {
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsNeverType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::never_type::FormatTsNeverType as FormatNodeRule<
            rome_js_syntax::TsNeverType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsNeverType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsNeverType,
        crate::ts::types::never_type::FormatTsNeverType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsNeverType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsNeverType,
        crate::ts::types::never_type::FormatTsNeverType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsParenthesizedType>
    for crate::ts::types::parenthesized_type::FormatTsParenthesizedType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsParenthesizedType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::parenthesized_type::FormatTsParenthesizedType as FormatNodeRule<
            rome_js_syntax::TsParenthesizedType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsParenthesizedType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsParenthesizedType,
        crate::ts::types::parenthesized_type::FormatTsParenthesizedType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsParenthesizedType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsParenthesizedType,
        crate::ts::types::parenthesized_type::FormatTsParenthesizedType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsReferenceType>
    for crate::ts::types::reference_type::FormatTsReferenceType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsReferenceType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::reference_type::FormatTsReferenceType as FormatNodeRule<
            rome_js_syntax::TsReferenceType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsReferenceType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsReferenceType,
        crate::ts::types::reference_type::FormatTsReferenceType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsReferenceType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsReferenceType,
        crate::ts::types::reference_type::FormatTsReferenceType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsArrayType> for crate::ts::types::array_type::FormatTsArrayType {
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsArrayType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::array_type::FormatTsArrayType as FormatNodeRule<
            rome_js_syntax::TsArrayType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsArrayType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsArrayType,
        crate::ts::types::array_type::FormatTsArrayType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsArrayType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsArrayType,
        crate::ts::types::array_type::FormatTsArrayType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsTupleType> for crate::ts::types::tuple_type::FormatTsTupleType {
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsTupleType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::tuple_type::FormatTsTupleType as FormatNodeRule<
            rome_js_syntax::TsTupleType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTupleType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTupleType,
        crate::ts::types::tuple_type::FormatTsTupleType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsTupleType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTupleType,
        crate::ts::types::tuple_type::FormatTsTupleType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsTypeofType>
    for crate::ts::types::typeof_type::FormatTsTypeofType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsTypeofType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::typeof_type::FormatTsTypeofType as FormatNodeRule<
            rome_js_syntax::TsTypeofType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeofType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeofType,
        crate::ts::types::typeof_type::FormatTsTypeofType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsTypeofType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeofType,
        crate::ts::types::typeof_type::FormatTsTypeofType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsImportType>
    for crate::ts::module::import_type::FormatTsImportType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsImportType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::module::import_type::FormatTsImportType as FormatNodeRule<
            rome_js_syntax::TsImportType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsImportType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsImportType,
        crate::ts::module::import_type::FormatTsImportType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsImportType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsImportType,
        crate::ts::module::import_type::FormatTsImportType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsTypeOperatorType>
    for crate::ts::types::type_operator_type::FormatTsTypeOperatorType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsTypeOperatorType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::type_operator_type::FormatTsTypeOperatorType as FormatNodeRule<
            rome_js_syntax::TsTypeOperatorType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeOperatorType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeOperatorType,
        crate::ts::types::type_operator_type::FormatTsTypeOperatorType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsTypeOperatorType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeOperatorType,
        crate::ts::types::type_operator_type::FormatTsTypeOperatorType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsIndexedAccessType>
    for crate::ts::types::indexed_access_type::FormatTsIndexedAccessType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsIndexedAccessType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::indexed_access_type::FormatTsIndexedAccessType as FormatNodeRule<
            rome_js_syntax::TsIndexedAccessType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsIndexedAccessType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsIndexedAccessType,
        crate::ts::types::indexed_access_type::FormatTsIndexedAccessType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsIndexedAccessType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsIndexedAccessType,
        crate::ts::types::indexed_access_type::FormatTsIndexedAccessType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsMappedType>
    for crate::ts::types::mapped_type::FormatTsMappedType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsMappedType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::mapped_type::FormatTsMappedType as FormatNodeRule<
            rome_js_syntax::TsMappedType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsMappedType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsMappedType,
        crate::ts::types::mapped_type::FormatTsMappedType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsMappedType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsMappedType,
        crate::ts::types::mapped_type::FormatTsMappedType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsObjectType>
    for crate::ts::types::object_type::FormatTsObjectType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsObjectType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::object_type::FormatTsObjectType as FormatNodeRule<
            rome_js_syntax::TsObjectType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsObjectType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsObjectType,
        crate::ts::types::object_type::FormatTsObjectType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsObjectType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsObjectType,
        crate::ts::types::object_type::FormatTsObjectType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsNonPrimitiveType>
    for crate::ts::types::non_primitive_type::FormatTsNonPrimitiveType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsNonPrimitiveType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::non_primitive_type::FormatTsNonPrimitiveType as FormatNodeRule<
            rome_js_syntax::TsNonPrimitiveType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsNonPrimitiveType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsNonPrimitiveType,
        crate::ts::types::non_primitive_type::FormatTsNonPrimitiveType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsNonPrimitiveType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsNonPrimitiveType,
        crate::ts::types::non_primitive_type::FormatTsNonPrimitiveType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsThisType> for crate::ts::types::this_type::FormatTsThisType {
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsThisType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::this_type::FormatTsThisType as FormatNodeRule<
            rome_js_syntax::TsThisType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsThisType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsThisType,
        crate::ts::types::this_type::FormatTsThisType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsThisType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsThisType,
        crate::ts::types::this_type::FormatTsThisType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsNumberLiteralType>
    for crate::ts::types::number_literal_type::FormatTsNumberLiteralType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsNumberLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::number_literal_type::FormatTsNumberLiteralType as FormatNodeRule<
            rome_js_syntax::TsNumberLiteralType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsNumberLiteralType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsNumberLiteralType,
        crate::ts::types::number_literal_type::FormatTsNumberLiteralType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsNumberLiteralType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsNumberLiteralType,
        crate::ts::types::number_literal_type::FormatTsNumberLiteralType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsBigIntLiteralType>
    for crate::ts::types::big_int_literal_type::FormatTsBigIntLiteralType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsBigIntLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::big_int_literal_type::FormatTsBigIntLiteralType as FormatNodeRule<
            rome_js_syntax::TsBigIntLiteralType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsBigIntLiteralType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsBigIntLiteralType,
        crate::ts::types::big_int_literal_type::FormatTsBigIntLiteralType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsBigIntLiteralType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsBigIntLiteralType,
        crate::ts::types::big_int_literal_type::FormatTsBigIntLiteralType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsStringLiteralType>
    for crate::ts::types::string_literal_type::FormatTsStringLiteralType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsStringLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::string_literal_type::FormatTsStringLiteralType as FormatNodeRule<
            rome_js_syntax::TsStringLiteralType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsStringLiteralType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsStringLiteralType,
        crate::ts::types::string_literal_type::FormatTsStringLiteralType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsStringLiteralType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsStringLiteralType,
        crate::ts::types::string_literal_type::FormatTsStringLiteralType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsNullLiteralType>
    for crate::ts::types::null_literal_type::FormatTsNullLiteralType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsNullLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::null_literal_type::FormatTsNullLiteralType as FormatNodeRule<
            rome_js_syntax::TsNullLiteralType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsNullLiteralType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsNullLiteralType,
        crate::ts::types::null_literal_type::FormatTsNullLiteralType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsNullLiteralType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsNullLiteralType,
        crate::ts::types::null_literal_type::FormatTsNullLiteralType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsBooleanLiteralType>
    for crate::ts::types::boolean_literal_type::FormatTsBooleanLiteralType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsBooleanLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::boolean_literal_type::FormatTsBooleanLiteralType as FormatNodeRule<
            rome_js_syntax::TsBooleanLiteralType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsBooleanLiteralType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsBooleanLiteralType,
        crate::ts::types::boolean_literal_type::FormatTsBooleanLiteralType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsBooleanLiteralType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsBooleanLiteralType,
        crate::ts::types::boolean_literal_type::FormatTsBooleanLiteralType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsTemplateLiteralType>
    for crate::ts::expressions::template_literal_type::FormatTsTemplateLiteralType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsTemplateLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: ts :: expressions :: template_literal_type :: FormatTsTemplateLiteralType as FormatNodeRule < rome_js_syntax :: TsTemplateLiteralType >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTemplateLiteralType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTemplateLiteralType,
        crate::ts::expressions::template_literal_type::FormatTsTemplateLiteralType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsTemplateLiteralType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTemplateLiteralType,
        crate::ts::expressions::template_literal_type::FormatTsTemplateLiteralType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsInferType> for crate::ts::types::infer_type::FormatTsInferType {
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsInferType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::infer_type::FormatTsInferType as FormatNodeRule<
            rome_js_syntax::TsInferType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsInferType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsInferType,
        crate::ts::types::infer_type::FormatTsInferType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsInferType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsInferType,
        crate::ts::types::infer_type::FormatTsInferType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsIntersectionType>
    for crate::ts::types::intersection_type::FormatTsIntersectionType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsIntersectionType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::intersection_type::FormatTsIntersectionType as FormatNodeRule<
            rome_js_syntax::TsIntersectionType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsIntersectionType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsIntersectionType,
        crate::ts::types::intersection_type::FormatTsIntersectionType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsIntersectionType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsIntersectionType,
        crate::ts::types::intersection_type::FormatTsIntersectionType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsUnionType> for crate::ts::types::union_type::FormatTsUnionType {
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsUnionType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::union_type::FormatTsUnionType as FormatNodeRule<
            rome_js_syntax::TsUnionType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsUnionType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsUnionType,
        crate::ts::types::union_type::FormatTsUnionType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsUnionType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsUnionType,
        crate::ts::types::union_type::FormatTsUnionType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsFunctionType>
    for crate::ts::types::function_type::FormatTsFunctionType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsFunctionType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::function_type::FormatTsFunctionType as FormatNodeRule<
            rome_js_syntax::TsFunctionType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsFunctionType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsFunctionType,
        crate::ts::types::function_type::FormatTsFunctionType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsFunctionType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsFunctionType,
        crate::ts::types::function_type::FormatTsFunctionType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsConstructorType>
    for crate::ts::types::constructor_type::FormatTsConstructorType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsConstructorType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::constructor_type::FormatTsConstructorType as FormatNodeRule<
            rome_js_syntax::TsConstructorType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsConstructorType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsConstructorType,
        crate::ts::types::constructor_type::FormatTsConstructorType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsConstructorType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsConstructorType,
        crate::ts::types::constructor_type::FormatTsConstructorType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsConditionalType>
    for crate::ts::types::conditional_type::FormatTsConditionalType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsConditionalType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::conditional_type::FormatTsConditionalType as FormatNodeRule<
            rome_js_syntax::TsConditionalType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsConditionalType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsConditionalType,
        crate::ts::types::conditional_type::FormatTsConditionalType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsConditionalType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsConditionalType,
        crate::ts::types::conditional_type::FormatTsConditionalType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsIdentifierBinding>
    for crate::ts::bindings::identifier_binding::FormatTsIdentifierBinding
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsIdentifierBinding, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::bindings::identifier_binding::FormatTsIdentifierBinding as FormatNodeRule<
            rome_js_syntax::TsIdentifierBinding,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsIdentifierBinding {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsIdentifierBinding,
        crate::ts::bindings::identifier_binding::FormatTsIdentifierBinding,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsIdentifierBinding {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsIdentifierBinding,
        crate::ts::bindings::identifier_binding::FormatTsIdentifierBinding,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsEnumMember>
    for crate::ts::auxiliary::enum_member::FormatTsEnumMember
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsEnumMember, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::auxiliary::enum_member::FormatTsEnumMember as FormatNodeRule<
            rome_js_syntax::TsEnumMember,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsEnumMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsEnumMember,
        crate::ts::auxiliary::enum_member::FormatTsEnumMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsEnumMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsEnumMember,
        crate::ts::auxiliary::enum_member::FormatTsEnumMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsExternalModuleReference>
    for crate::ts::auxiliary::external_module_reference::FormatTsExternalModuleReference
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsExternalModuleReference,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: auxiliary :: external_module_reference :: FormatTsExternalModuleReference as FormatNodeRule < rome_js_syntax :: TsExternalModuleReference >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsExternalModuleReference {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsExternalModuleReference,
        crate::ts::auxiliary::external_module_reference::FormatTsExternalModuleReference,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsExternalModuleReference {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsExternalModuleReference,
        crate::ts::auxiliary::external_module_reference::FormatTsExternalModuleReference,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsModuleBlock>
    for crate::ts::auxiliary::module_block::FormatTsModuleBlock
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsModuleBlock, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::auxiliary::module_block::FormatTsModuleBlock as FormatNodeRule<
            rome_js_syntax::TsModuleBlock,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsModuleBlock {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsModuleBlock,
        crate::ts::auxiliary::module_block::FormatTsModuleBlock,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsModuleBlock {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsModuleBlock,
        crate::ts::auxiliary::module_block::FormatTsModuleBlock,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsQualifiedModuleName>
    for crate::ts::auxiliary::qualified_module_name::FormatTsQualifiedModuleName
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsQualifiedModuleName, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: ts :: auxiliary :: qualified_module_name :: FormatTsQualifiedModuleName as FormatNodeRule < rome_js_syntax :: TsQualifiedModuleName >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsQualifiedModuleName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsQualifiedModuleName,
        crate::ts::auxiliary::qualified_module_name::FormatTsQualifiedModuleName,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsQualifiedModuleName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsQualifiedModuleName,
        crate::ts::auxiliary::qualified_module_name::FormatTsQualifiedModuleName,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule < rome_js_syntax :: TsEmptyExternalModuleDeclarationBody > for crate :: ts :: auxiliary :: empty_external_module_declaration_body :: FormatTsEmptyExternalModuleDeclarationBody { type Context = JsFormatContext ; fn fmt (node : & rome_js_syntax :: TsEmptyExternalModuleDeclarationBody , f : & mut JsFormatter) -> FormatResult < () > { < crate :: ts :: auxiliary :: empty_external_module_declaration_body :: FormatTsEmptyExternalModuleDeclarationBody as FormatNodeRule < rome_js_syntax :: TsEmptyExternalModuleDeclarationBody >> :: fmt (node , f) } }
impl<'a> AsFormat<'a> for rome_js_syntax::TsEmptyExternalModuleDeclarationBody {
    type Format = FormatRefWithRule < 'a , rome_js_syntax :: TsEmptyExternalModuleDeclarationBody , crate :: ts :: auxiliary :: empty_external_module_declaration_body :: FormatTsEmptyExternalModuleDeclarationBody > ;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsEmptyExternalModuleDeclarationBody {
    type Format = FormatOwnedWithRule < rome_js_syntax :: TsEmptyExternalModuleDeclarationBody , crate :: ts :: auxiliary :: empty_external_module_declaration_body :: FormatTsEmptyExternalModuleDeclarationBody > ;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsTypeParameterName>
    for crate::ts::auxiliary::type_parameter_name::FormatTsTypeParameterName
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsTypeParameterName, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::auxiliary::type_parameter_name::FormatTsTypeParameterName as FormatNodeRule<
            rome_js_syntax::TsTypeParameterName,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeParameterName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeParameterName,
        crate::ts::auxiliary::type_parameter_name::FormatTsTypeParameterName,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsTypeParameterName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeParameterName,
        crate::ts::auxiliary::type_parameter_name::FormatTsTypeParameterName,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsPredicateReturnType>
    for crate::ts::types::predicate_return_type::FormatTsPredicateReturnType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsPredicateReturnType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::predicate_return_type::FormatTsPredicateReturnType as FormatNodeRule<
            rome_js_syntax::TsPredicateReturnType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsPredicateReturnType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsPredicateReturnType,
        crate::ts::types::predicate_return_type::FormatTsPredicateReturnType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsPredicateReturnType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsPredicateReturnType,
        crate::ts::types::predicate_return_type::FormatTsPredicateReturnType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsAssertsReturnType>
    for crate::ts::types::asserts_return_type::FormatTsAssertsReturnType
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsAssertsReturnType, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::types::asserts_return_type::FormatTsAssertsReturnType as FormatNodeRule<
            rome_js_syntax::TsAssertsReturnType,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAssertsReturnType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAssertsReturnType,
        crate::ts::types::asserts_return_type::FormatTsAssertsReturnType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsAssertsReturnType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAssertsReturnType,
        crate::ts::types::asserts_return_type::FormatTsAssertsReturnType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsAssertsCondition>
    for crate::ts::auxiliary::asserts_condition::FormatTsAssertsCondition
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsAssertsCondition, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::auxiliary::asserts_condition::FormatTsAssertsCondition as FormatNodeRule<
            rome_js_syntax::TsAssertsCondition,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAssertsCondition {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAssertsCondition,
        crate::ts::auxiliary::asserts_condition::FormatTsAssertsCondition,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsAssertsCondition {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAssertsCondition,
        crate::ts::auxiliary::asserts_condition::FormatTsAssertsCondition,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsTypeParameter>
    for crate::ts::bindings::type_parameter::FormatTsTypeParameter
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsTypeParameter, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::bindings::type_parameter::FormatTsTypeParameter as FormatNodeRule<
            rome_js_syntax::TsTypeParameter,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeParameter {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeParameter,
        crate::ts::bindings::type_parameter::FormatTsTypeParameter,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsTypeParameter {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeParameter,
        crate::ts::bindings::type_parameter::FormatTsTypeParameter,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsTypeConstraintClause>
    for crate::ts::auxiliary::type_constraint_clause::FormatTsTypeConstraintClause
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsTypeConstraintClause, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: ts :: auxiliary :: type_constraint_clause :: FormatTsTypeConstraintClause as FormatNodeRule < rome_js_syntax :: TsTypeConstraintClause >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeConstraintClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeConstraintClause,
        crate::ts::auxiliary::type_constraint_clause::FormatTsTypeConstraintClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsTypeConstraintClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeConstraintClause,
        crate::ts::auxiliary::type_constraint_clause::FormatTsTypeConstraintClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsDefaultTypeClause>
    for crate::ts::auxiliary::default_type_clause::FormatTsDefaultTypeClause
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsDefaultTypeClause, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::auxiliary::default_type_clause::FormatTsDefaultTypeClause as FormatNodeRule<
            rome_js_syntax::TsDefaultTypeClause,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsDefaultTypeClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsDefaultTypeClause,
        crate::ts::auxiliary::default_type_clause::FormatTsDefaultTypeClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsDefaultTypeClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsDefaultTypeClause,
        crate::ts::auxiliary::default_type_clause::FormatTsDefaultTypeClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsExtendsClause>
    for crate::ts::classes::extends_clause::FormatTsExtendsClause
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsExtendsClause, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::classes::extends_clause::FormatTsExtendsClause as FormatNodeRule<
            rome_js_syntax::TsExtendsClause,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsExtendsClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsExtendsClause,
        crate::ts::classes::extends_clause::FormatTsExtendsClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsExtendsClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsExtendsClause,
        crate::ts::classes::extends_clause::FormatTsExtendsClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsNameWithTypeArguments>
    for crate::ts::expressions::name_with_type_arguments::FormatTsNameWithTypeArguments
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsNameWithTypeArguments,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: expressions :: name_with_type_arguments :: FormatTsNameWithTypeArguments as FormatNodeRule < rome_js_syntax :: TsNameWithTypeArguments >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsNameWithTypeArguments {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsNameWithTypeArguments,
        crate::ts::expressions::name_with_type_arguments::FormatTsNameWithTypeArguments,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsNameWithTypeArguments {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsNameWithTypeArguments,
        crate::ts::expressions::name_with_type_arguments::FormatTsNameWithTypeArguments,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsCallSignatureTypeMember>
    for crate::ts::auxiliary::call_signature_type_member::FormatTsCallSignatureTypeMember
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsCallSignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: auxiliary :: call_signature_type_member :: FormatTsCallSignatureTypeMember as FormatNodeRule < rome_js_syntax :: TsCallSignatureTypeMember >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsCallSignatureTypeMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsCallSignatureTypeMember,
        crate::ts::auxiliary::call_signature_type_member::FormatTsCallSignatureTypeMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsCallSignatureTypeMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsCallSignatureTypeMember,
        crate::ts::auxiliary::call_signature_type_member::FormatTsCallSignatureTypeMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsPropertySignatureTypeMember>
    for crate::ts::auxiliary::property_signature_type_member::FormatTsPropertySignatureTypeMember
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsPropertySignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: auxiliary :: property_signature_type_member :: FormatTsPropertySignatureTypeMember as FormatNodeRule < rome_js_syntax :: TsPropertySignatureTypeMember >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsPropertySignatureTypeMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsPropertySignatureTypeMember,
        crate::ts::auxiliary::property_signature_type_member::FormatTsPropertySignatureTypeMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsPropertySignatureTypeMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsPropertySignatureTypeMember,
        crate::ts::auxiliary::property_signature_type_member::FormatTsPropertySignatureTypeMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsConstructSignatureTypeMember>
    for crate::ts::auxiliary::construct_signature_type_member::FormatTsConstructSignatureTypeMember
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsConstructSignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: auxiliary :: construct_signature_type_member :: FormatTsConstructSignatureTypeMember as FormatNodeRule < rome_js_syntax :: TsConstructSignatureTypeMember >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsConstructSignatureTypeMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsConstructSignatureTypeMember,
        crate::ts::auxiliary::construct_signature_type_member::FormatTsConstructSignatureTypeMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsConstructSignatureTypeMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsConstructSignatureTypeMember,
        crate::ts::auxiliary::construct_signature_type_member::FormatTsConstructSignatureTypeMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsMethodSignatureTypeMember>
    for crate::ts::auxiliary::method_signature_type_member::FormatTsMethodSignatureTypeMember
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsMethodSignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: auxiliary :: method_signature_type_member :: FormatTsMethodSignatureTypeMember as FormatNodeRule < rome_js_syntax :: TsMethodSignatureTypeMember >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsMethodSignatureTypeMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsMethodSignatureTypeMember,
        crate::ts::auxiliary::method_signature_type_member::FormatTsMethodSignatureTypeMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsMethodSignatureTypeMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsMethodSignatureTypeMember,
        crate::ts::auxiliary::method_signature_type_member::FormatTsMethodSignatureTypeMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsGetterSignatureTypeMember>
    for crate::ts::auxiliary::getter_signature_type_member::FormatTsGetterSignatureTypeMember
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsGetterSignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: auxiliary :: getter_signature_type_member :: FormatTsGetterSignatureTypeMember as FormatNodeRule < rome_js_syntax :: TsGetterSignatureTypeMember >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsGetterSignatureTypeMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsGetterSignatureTypeMember,
        crate::ts::auxiliary::getter_signature_type_member::FormatTsGetterSignatureTypeMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsGetterSignatureTypeMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsGetterSignatureTypeMember,
        crate::ts::auxiliary::getter_signature_type_member::FormatTsGetterSignatureTypeMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsSetterSignatureTypeMember>
    for crate::ts::auxiliary::setter_signature_type_member::FormatTsSetterSignatureTypeMember
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsSetterSignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: auxiliary :: setter_signature_type_member :: FormatTsSetterSignatureTypeMember as FormatNodeRule < rome_js_syntax :: TsSetterSignatureTypeMember >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsSetterSignatureTypeMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsSetterSignatureTypeMember,
        crate::ts::auxiliary::setter_signature_type_member::FormatTsSetterSignatureTypeMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsSetterSignatureTypeMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsSetterSignatureTypeMember,
        crate::ts::auxiliary::setter_signature_type_member::FormatTsSetterSignatureTypeMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsIndexSignatureTypeMember>
    for crate::ts::auxiliary::index_signature_type_member::FormatTsIndexSignatureTypeMember
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsIndexSignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: auxiliary :: index_signature_type_member :: FormatTsIndexSignatureTypeMember as FormatNodeRule < rome_js_syntax :: TsIndexSignatureTypeMember >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsIndexSignatureTypeMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsIndexSignatureTypeMember,
        crate::ts::auxiliary::index_signature_type_member::FormatTsIndexSignatureTypeMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsIndexSignatureTypeMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsIndexSignatureTypeMember,
        crate::ts::auxiliary::index_signature_type_member::FormatTsIndexSignatureTypeMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule < rome_js_syntax :: TsMappedTypeReadonlyModifierClause > for crate :: ts :: auxiliary :: mapped_type_readonly_modifier_clause :: FormatTsMappedTypeReadonlyModifierClause { type Context = JsFormatContext ; fn fmt (node : & rome_js_syntax :: TsMappedTypeReadonlyModifierClause , f : & mut JsFormatter) -> FormatResult < () > { < crate :: ts :: auxiliary :: mapped_type_readonly_modifier_clause :: FormatTsMappedTypeReadonlyModifierClause as FormatNodeRule < rome_js_syntax :: TsMappedTypeReadonlyModifierClause >> :: fmt (node , f) } }
impl<'a> AsFormat<'a> for rome_js_syntax::TsMappedTypeReadonlyModifierClause {
    type Format = FormatRefWithRule < 'a , rome_js_syntax :: TsMappedTypeReadonlyModifierClause , crate :: ts :: auxiliary :: mapped_type_readonly_modifier_clause :: FormatTsMappedTypeReadonlyModifierClause > ;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsMappedTypeReadonlyModifierClause {
    type Format = FormatOwnedWithRule < rome_js_syntax :: TsMappedTypeReadonlyModifierClause , crate :: ts :: auxiliary :: mapped_type_readonly_modifier_clause :: FormatTsMappedTypeReadonlyModifierClause > ;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsMappedTypeAsClause>
    for crate::ts::auxiliary::mapped_type_as_clause::FormatTsMappedTypeAsClause
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsMappedTypeAsClause, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: ts :: auxiliary :: mapped_type_as_clause :: FormatTsMappedTypeAsClause as FormatNodeRule < rome_js_syntax :: TsMappedTypeAsClause >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsMappedTypeAsClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsMappedTypeAsClause,
        crate::ts::auxiliary::mapped_type_as_clause::FormatTsMappedTypeAsClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsMappedTypeAsClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsMappedTypeAsClause,
        crate::ts::auxiliary::mapped_type_as_clause::FormatTsMappedTypeAsClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule < rome_js_syntax :: TsMappedTypeOptionalModifierClause > for crate :: ts :: auxiliary :: mapped_type_optional_modifier_clause :: FormatTsMappedTypeOptionalModifierClause { type Context = JsFormatContext ; fn fmt (node : & rome_js_syntax :: TsMappedTypeOptionalModifierClause , f : & mut JsFormatter) -> FormatResult < () > { < crate :: ts :: auxiliary :: mapped_type_optional_modifier_clause :: FormatTsMappedTypeOptionalModifierClause as FormatNodeRule < rome_js_syntax :: TsMappedTypeOptionalModifierClause >> :: fmt (node , f) } }
impl<'a> AsFormat<'a> for rome_js_syntax::TsMappedTypeOptionalModifierClause {
    type Format = FormatRefWithRule < 'a , rome_js_syntax :: TsMappedTypeOptionalModifierClause , crate :: ts :: auxiliary :: mapped_type_optional_modifier_clause :: FormatTsMappedTypeOptionalModifierClause > ;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsMappedTypeOptionalModifierClause {
    type Format = FormatOwnedWithRule < rome_js_syntax :: TsMappedTypeOptionalModifierClause , crate :: ts :: auxiliary :: mapped_type_optional_modifier_clause :: FormatTsMappedTypeOptionalModifierClause > ;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsImportTypeQualifier>
    for crate::ts::module::import_type_qualifier::FormatTsImportTypeQualifier
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsImportTypeQualifier, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::module::import_type_qualifier::FormatTsImportTypeQualifier as FormatNodeRule<
            rome_js_syntax::TsImportTypeQualifier,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsImportTypeQualifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsImportTypeQualifier,
        crate::ts::module::import_type_qualifier::FormatTsImportTypeQualifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsImportTypeQualifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsImportTypeQualifier,
        crate::ts::module::import_type_qualifier::FormatTsImportTypeQualifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsNamedTupleTypeElement>
    for crate::ts::auxiliary::named_tuple_type_element::FormatTsNamedTupleTypeElement
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsNamedTupleTypeElement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: auxiliary :: named_tuple_type_element :: FormatTsNamedTupleTypeElement as FormatNodeRule < rome_js_syntax :: TsNamedTupleTypeElement >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsNamedTupleTypeElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsNamedTupleTypeElement,
        crate::ts::auxiliary::named_tuple_type_element::FormatTsNamedTupleTypeElement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsNamedTupleTypeElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsNamedTupleTypeElement,
        crate::ts::auxiliary::named_tuple_type_element::FormatTsNamedTupleTypeElement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsRestTupleTypeElement>
    for crate::ts::auxiliary::rest_tuple_type_element::FormatTsRestTupleTypeElement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsRestTupleTypeElement, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: ts :: auxiliary :: rest_tuple_type_element :: FormatTsRestTupleTypeElement as FormatNodeRule < rome_js_syntax :: TsRestTupleTypeElement >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsRestTupleTypeElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsRestTupleTypeElement,
        crate::ts::auxiliary::rest_tuple_type_element::FormatTsRestTupleTypeElement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsRestTupleTypeElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsRestTupleTypeElement,
        crate::ts::auxiliary::rest_tuple_type_element::FormatTsRestTupleTypeElement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsOptionalTupleTypeElement>
    for crate::ts::auxiliary::optional_tuple_type_element::FormatTsOptionalTupleTypeElement
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::TsOptionalTupleTypeElement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: ts :: auxiliary :: optional_tuple_type_element :: FormatTsOptionalTupleTypeElement as FormatNodeRule < rome_js_syntax :: TsOptionalTupleTypeElement >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsOptionalTupleTypeElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsOptionalTupleTypeElement,
        crate::ts::auxiliary::optional_tuple_type_element::FormatTsOptionalTupleTypeElement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsOptionalTupleTypeElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsOptionalTupleTypeElement,
        crate::ts::auxiliary::optional_tuple_type_element::FormatTsOptionalTupleTypeElement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsTemplateChunkElement>
    for crate::ts::expressions::template_chunk_element::FormatTsTemplateChunkElement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsTemplateChunkElement, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: ts :: expressions :: template_chunk_element :: FormatTsTemplateChunkElement as FormatNodeRule < rome_js_syntax :: TsTemplateChunkElement >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTemplateChunkElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTemplateChunkElement,
        crate::ts::expressions::template_chunk_element::FormatTsTemplateChunkElement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsTemplateChunkElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTemplateChunkElement,
        crate::ts::expressions::template_chunk_element::FormatTsTemplateChunkElement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsTemplateElement>
    for crate::ts::expressions::template_element::FormatTsTemplateElement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsTemplateElement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::expressions::template_element::FormatTsTemplateElement as FormatNodeRule<
            rome_js_syntax::TsTemplateElement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTemplateElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTemplateElement,
        crate::ts::expressions::template_element::FormatTsTemplateElement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsTemplateElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTemplateElement,
        crate::ts::expressions::template_element::FormatTsTemplateElement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::TsQualifiedName>
    for crate::ts::auxiliary::qualified_name::FormatTsQualifiedName
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::TsQualifiedName, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::ts::auxiliary::qualified_name::FormatTsQualifiedName as FormatNodeRule<
            rome_js_syntax::TsQualifiedName,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsQualifiedName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsQualifiedName,
        crate::ts::auxiliary::qualified_name::FormatTsQualifiedName,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsQualifiedName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsQualifiedName,
        crate::ts::auxiliary::qualified_name::FormatTsQualifiedName,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsxElement> for crate::jsx::tag::element::FormatJsxElement {
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsxElement, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: jsx :: tag :: element :: FormatJsxElement as FormatNodeRule < rome_js_syntax :: JsxElement >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxElement,
        crate::jsx::tag::element::FormatJsxElement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxElement {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsxElement, crate::jsx::tag::element::FormatJsxElement>;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsxSelfClosingElement>
    for crate::jsx::tag::self_closing_element::FormatJsxSelfClosingElement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsxSelfClosingElement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::jsx::tag::self_closing_element::FormatJsxSelfClosingElement as FormatNodeRule<
            rome_js_syntax::JsxSelfClosingElement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxSelfClosingElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxSelfClosingElement,
        crate::jsx::tag::self_closing_element::FormatJsxSelfClosingElement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxSelfClosingElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxSelfClosingElement,
        crate::jsx::tag::self_closing_element::FormatJsxSelfClosingElement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsxFragment> for crate::jsx::tag::fragment::FormatJsxFragment {
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsxFragment, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::jsx::tag::fragment::FormatJsxFragment as FormatNodeRule<
            rome_js_syntax::JsxFragment,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxFragment {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxFragment,
        crate::jsx::tag::fragment::FormatJsxFragment,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxFragment {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxFragment,
        crate::jsx::tag::fragment::FormatJsxFragment,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsxOpeningElement>
    for crate::jsx::tag::opening_element::FormatJsxOpeningElement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsxOpeningElement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::jsx::tag::opening_element::FormatJsxOpeningElement as FormatNodeRule<
            rome_js_syntax::JsxOpeningElement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxOpeningElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxOpeningElement,
        crate::jsx::tag::opening_element::FormatJsxOpeningElement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxOpeningElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxOpeningElement,
        crate::jsx::tag::opening_element::FormatJsxOpeningElement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsxClosingElement>
    for crate::jsx::tag::closing_element::FormatJsxClosingElement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsxClosingElement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::jsx::tag::closing_element::FormatJsxClosingElement as FormatNodeRule<
            rome_js_syntax::JsxClosingElement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxClosingElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxClosingElement,
        crate::jsx::tag::closing_element::FormatJsxClosingElement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxClosingElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxClosingElement,
        crate::jsx::tag::closing_element::FormatJsxClosingElement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsxOpeningFragment>
    for crate::jsx::tag::opening_fragment::FormatJsxOpeningFragment
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsxOpeningFragment, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::jsx::tag::opening_fragment::FormatJsxOpeningFragment as FormatNodeRule<
            rome_js_syntax::JsxOpeningFragment,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxOpeningFragment {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxOpeningFragment,
        crate::jsx::tag::opening_fragment::FormatJsxOpeningFragment,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxOpeningFragment {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxOpeningFragment,
        crate::jsx::tag::opening_fragment::FormatJsxOpeningFragment,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsxClosingFragment>
    for crate::jsx::tag::closing_fragment::FormatJsxClosingFragment
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsxClosingFragment, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::jsx::tag::closing_fragment::FormatJsxClosingFragment as FormatNodeRule<
            rome_js_syntax::JsxClosingFragment,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxClosingFragment {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxClosingFragment,
        crate::jsx::tag::closing_fragment::FormatJsxClosingFragment,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxClosingFragment {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxClosingFragment,
        crate::jsx::tag::closing_fragment::FormatJsxClosingFragment,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsxName> for crate::jsx::auxiliary::name::FormatJsxName {
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsxName, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::jsx::auxiliary::name::FormatJsxName as FormatNodeRule<rome_js_syntax::JsxName>>::fmt(
            node, f,
        )
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxName {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsxName, crate::jsx::auxiliary::name::FormatJsxName>;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxName {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsxName, crate::jsx::auxiliary::name::FormatJsxName>;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsxReferenceIdentifier>
    for crate::jsx::auxiliary::reference_identifier::FormatJsxReferenceIdentifier
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsxReferenceIdentifier, f: &mut JsFormatter) -> FormatResult<()> {
        < crate :: jsx :: auxiliary :: reference_identifier :: FormatJsxReferenceIdentifier as FormatNodeRule < rome_js_syntax :: JsxReferenceIdentifier >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxReferenceIdentifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxReferenceIdentifier,
        crate::jsx::auxiliary::reference_identifier::FormatJsxReferenceIdentifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxReferenceIdentifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxReferenceIdentifier,
        crate::jsx::auxiliary::reference_identifier::FormatJsxReferenceIdentifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsxNamespaceName>
    for crate::jsx::auxiliary::namespace_name::FormatJsxNamespaceName
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsxNamespaceName, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::jsx::auxiliary::namespace_name::FormatJsxNamespaceName as FormatNodeRule<
            rome_js_syntax::JsxNamespaceName,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxNamespaceName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxNamespaceName,
        crate::jsx::auxiliary::namespace_name::FormatJsxNamespaceName,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxNamespaceName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxNamespaceName,
        crate::jsx::auxiliary::namespace_name::FormatJsxNamespaceName,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsxMemberName>
    for crate::jsx::objects::member_name::FormatJsxMemberName
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsxMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::jsx::objects::member_name::FormatJsxMemberName as FormatNodeRule<
            rome_js_syntax::JsxMemberName,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxMemberName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxMemberName,
        crate::jsx::objects::member_name::FormatJsxMemberName,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxMemberName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxMemberName,
        crate::jsx::objects::member_name::FormatJsxMemberName,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsxAttribute>
    for crate::jsx::attribute::attribute::FormatJsxAttribute
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsxAttribute, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::jsx::attribute::attribute::FormatJsxAttribute as FormatNodeRule<
            rome_js_syntax::JsxAttribute,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxAttribute {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxAttribute,
        crate::jsx::attribute::attribute::FormatJsxAttribute,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxAttribute {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxAttribute,
        crate::jsx::attribute::attribute::FormatJsxAttribute,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsxSpreadAttribute>
    for crate::jsx::attribute::spread_attribute::FormatJsxSpreadAttribute
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsxSpreadAttribute, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::jsx::attribute::spread_attribute::FormatJsxSpreadAttribute as FormatNodeRule<
            rome_js_syntax::JsxSpreadAttribute,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxSpreadAttribute {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxSpreadAttribute,
        crate::jsx::attribute::spread_attribute::FormatJsxSpreadAttribute,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxSpreadAttribute {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxSpreadAttribute,
        crate::jsx::attribute::spread_attribute::FormatJsxSpreadAttribute,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsxAttributeInitializerClause>
    for crate::jsx::attribute::attribute_initializer_clause::FormatJsxAttributeInitializerClause
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsxAttributeInitializerClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: jsx :: attribute :: attribute_initializer_clause :: FormatJsxAttributeInitializerClause as FormatNodeRule < rome_js_syntax :: JsxAttributeInitializerClause >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxAttributeInitializerClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxAttributeInitializerClause,
        crate::jsx::attribute::attribute_initializer_clause::FormatJsxAttributeInitializerClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxAttributeInitializerClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxAttributeInitializerClause,
        crate::jsx::attribute::attribute_initializer_clause::FormatJsxAttributeInitializerClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsxString> for crate::jsx::auxiliary::string::FormatJsxString {
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsxString, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::jsx::auxiliary::string::FormatJsxString as FormatNodeRule<
            rome_js_syntax::JsxString,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxString {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxString,
        crate::jsx::auxiliary::string::FormatJsxString,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxString {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxString,
        crate::jsx::auxiliary::string::FormatJsxString,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsxExpressionAttributeValue>
    for crate::jsx::attribute::expression_attribute_value::FormatJsxExpressionAttributeValue
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsxExpressionAttributeValue,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: jsx :: attribute :: expression_attribute_value :: FormatJsxExpressionAttributeValue as FormatNodeRule < rome_js_syntax :: JsxExpressionAttributeValue >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxExpressionAttributeValue {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxExpressionAttributeValue,
        crate::jsx::attribute::expression_attribute_value::FormatJsxExpressionAttributeValue,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxExpressionAttributeValue {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxExpressionAttributeValue,
        crate::jsx::attribute::expression_attribute_value::FormatJsxExpressionAttributeValue,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsxText> for crate::jsx::auxiliary::text::FormatJsxText {
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsxText, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::jsx::auxiliary::text::FormatJsxText as FormatNodeRule<rome_js_syntax::JsxText>>::fmt(
            node, f,
        )
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxText {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsxText, crate::jsx::auxiliary::text::FormatJsxText>;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxText {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsxText, crate::jsx::auxiliary::text::FormatJsxText>;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsxExpressionChild>
    for crate::jsx::auxiliary::expression_child::FormatJsxExpressionChild
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsxExpressionChild, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::jsx::auxiliary::expression_child::FormatJsxExpressionChild as FormatNodeRule<
            rome_js_syntax::JsxExpressionChild,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxExpressionChild {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxExpressionChild,
        crate::jsx::auxiliary::expression_child::FormatJsxExpressionChild,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxExpressionChild {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxExpressionChild,
        crate::jsx::auxiliary::expression_child::FormatJsxExpressionChild,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsxSpreadChild>
    for crate::jsx::auxiliary::spread_child::FormatJsxSpreadChild
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsxSpreadChild, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::jsx::auxiliary::spread_child::FormatJsxSpreadChild as FormatNodeRule<
            rome_js_syntax::JsxSpreadChild,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxSpreadChild {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxSpreadChild,
        crate::jsx::auxiliary::spread_child::FormatJsxSpreadChild,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxSpreadChild {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxSpreadChild,
        crate::jsx::auxiliary::spread_child::FormatJsxSpreadChild,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsArrayAssignmentPatternElementList {
    type Format = FormatRefWithRule < 'a , rome_js_syntax :: JsArrayAssignmentPatternElementList , crate :: js :: lists :: array_assignment_pattern_element_list :: FormatJsArrayAssignmentPatternElementList > ;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsArrayAssignmentPatternElementList {
    type Format = FormatOwnedWithRule < rome_js_syntax :: JsArrayAssignmentPatternElementList , crate :: js :: lists :: array_assignment_pattern_element_list :: FormatJsArrayAssignmentPatternElementList > ;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsArrayBindingPatternElementList {
    type Format = FormatRefWithRule < 'a , rome_js_syntax :: JsArrayBindingPatternElementList , crate :: js :: lists :: array_binding_pattern_element_list :: FormatJsArrayBindingPatternElementList > ;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsArrayBindingPatternElementList {
    type Format = FormatOwnedWithRule < rome_js_syntax :: JsArrayBindingPatternElementList , crate :: js :: lists :: array_binding_pattern_element_list :: FormatJsArrayBindingPatternElementList > ;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsArrayElementList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsArrayElementList,
        crate::js::lists::array_element_list::FormatJsArrayElementList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsArrayElementList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsArrayElementList,
        crate::js::lists::array_element_list::FormatJsArrayElementList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsCallArgumentList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsCallArgumentList,
        crate::js::lists::call_argument_list::FormatJsCallArgumentList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsCallArgumentList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsCallArgumentList,
        crate::js::lists::call_argument_list::FormatJsCallArgumentList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsClassMemberList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsClassMemberList,
        crate::js::lists::class_member_list::FormatJsClassMemberList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsClassMemberList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsClassMemberList,
        crate::js::lists::class_member_list::FormatJsClassMemberList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsConstructorModifierList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsConstructorModifierList,
        crate::js::lists::constructor_modifier_list::FormatJsConstructorModifierList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsConstructorModifierList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsConstructorModifierList,
        crate::js::lists::constructor_modifier_list::FormatJsConstructorModifierList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsConstructorParameterList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsConstructorParameterList,
        crate::js::lists::constructor_parameter_list::FormatJsConstructorParameterList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsConstructorParameterList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsConstructorParameterList,
        crate::js::lists::constructor_parameter_list::FormatJsConstructorParameterList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsDirectiveList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsDirectiveList,
        crate::js::lists::directive_list::FormatJsDirectiveList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsDirectiveList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsDirectiveList,
        crate::js::lists::directive_list::FormatJsDirectiveList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExportNamedFromSpecifierList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExportNamedFromSpecifierList,
        crate::js::lists::export_named_from_specifier_list::FormatJsExportNamedFromSpecifierList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsExportNamedFromSpecifierList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExportNamedFromSpecifierList,
        crate::js::lists::export_named_from_specifier_list::FormatJsExportNamedFromSpecifierList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsExportNamedSpecifierList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsExportNamedSpecifierList,
        crate::js::lists::export_named_specifier_list::FormatJsExportNamedSpecifierList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsExportNamedSpecifierList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsExportNamedSpecifierList,
        crate::js::lists::export_named_specifier_list::FormatJsExportNamedSpecifierList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsImportAssertionEntryList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsImportAssertionEntryList,
        crate::js::lists::import_assertion_entry_list::FormatJsImportAssertionEntryList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsImportAssertionEntryList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsImportAssertionEntryList,
        crate::js::lists::import_assertion_entry_list::FormatJsImportAssertionEntryList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsMethodModifierList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsMethodModifierList,
        crate::js::lists::method_modifier_list::FormatJsMethodModifierList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsMethodModifierList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsMethodModifierList,
        crate::js::lists::method_modifier_list::FormatJsMethodModifierList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsModuleItemList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsModuleItemList,
        crate::js::lists::module_item_list::FormatJsModuleItemList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsModuleItemList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsModuleItemList,
        crate::js::lists::module_item_list::FormatJsModuleItemList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsNamedImportSpecifierList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsNamedImportSpecifierList,
        crate::js::lists::named_import_specifier_list::FormatJsNamedImportSpecifierList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsNamedImportSpecifierList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsNamedImportSpecifierList,
        crate::js::lists::named_import_specifier_list::FormatJsNamedImportSpecifierList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectAssignmentPatternPropertyList {
    type Format = FormatRefWithRule < 'a , rome_js_syntax :: JsObjectAssignmentPatternPropertyList , crate :: js :: lists :: object_assignment_pattern_property_list :: FormatJsObjectAssignmentPatternPropertyList > ;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsObjectAssignmentPatternPropertyList {
    type Format = FormatOwnedWithRule < rome_js_syntax :: JsObjectAssignmentPatternPropertyList , crate :: js :: lists :: object_assignment_pattern_property_list :: FormatJsObjectAssignmentPatternPropertyList > ;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectBindingPatternPropertyList {
    type Format = FormatRefWithRule < 'a , rome_js_syntax :: JsObjectBindingPatternPropertyList , crate :: js :: lists :: object_binding_pattern_property_list :: FormatJsObjectBindingPatternPropertyList > ;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsObjectBindingPatternPropertyList {
    type Format = FormatOwnedWithRule < rome_js_syntax :: JsObjectBindingPatternPropertyList , crate :: js :: lists :: object_binding_pattern_property_list :: FormatJsObjectBindingPatternPropertyList > ;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsObjectMemberList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsObjectMemberList,
        crate::js::lists::object_member_list::FormatJsObjectMemberList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsObjectMemberList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsObjectMemberList,
        crate::js::lists::object_member_list::FormatJsObjectMemberList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsParameterList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsParameterList,
        crate::js::lists::parameter_list::FormatJsParameterList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsParameterList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsParameterList,
        crate::js::lists::parameter_list::FormatJsParameterList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsPropertyModifierList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsPropertyModifierList,
        crate::js::lists::property_modifier_list::FormatJsPropertyModifierList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsPropertyModifierList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsPropertyModifierList,
        crate::js::lists::property_modifier_list::FormatJsPropertyModifierList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsStatementList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsStatementList,
        crate::js::lists::statement_list::FormatJsStatementList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsStatementList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsStatementList,
        crate::js::lists::statement_list::FormatJsStatementList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsSwitchCaseList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsSwitchCaseList,
        crate::js::lists::switch_case_list::FormatJsSwitchCaseList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsSwitchCaseList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsSwitchCaseList,
        crate::js::lists::switch_case_list::FormatJsSwitchCaseList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsTemplateElementList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsTemplateElementList,
        crate::js::lists::template_element_list::FormatJsTemplateElementList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsTemplateElementList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsTemplateElementList,
        crate::js::lists::template_element_list::FormatJsTemplateElementList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsVariableDeclaratorList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsVariableDeclaratorList,
        crate::js::lists::variable_declarator_list::FormatJsVariableDeclaratorList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsVariableDeclaratorList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsVariableDeclaratorList,
        crate::js::lists::variable_declarator_list::FormatJsVariableDeclaratorList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxAttributeList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxAttributeList,
        crate::jsx::lists::attribute_list::FormatJsxAttributeList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxAttributeList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxAttributeList,
        crate::jsx::lists::attribute_list::FormatJsxAttributeList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxChildList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxChildList,
        crate::jsx::lists::child_list::FormatJsxChildList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxChildList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxChildList,
        crate::jsx::lists::child_list::FormatJsxChildList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsEnumMemberList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsEnumMemberList,
        crate::ts::lists::enum_member_list::FormatTsEnumMemberList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsEnumMemberList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsEnumMemberList,
        crate::ts::lists::enum_member_list::FormatTsEnumMemberList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsIndexSignatureModifierList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsIndexSignatureModifierList,
        crate::ts::lists::index_signature_modifier_list::FormatTsIndexSignatureModifierList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsIndexSignatureModifierList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsIndexSignatureModifierList,
        crate::ts::lists::index_signature_modifier_list::FormatTsIndexSignatureModifierList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsIntersectionTypeElementList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsIntersectionTypeElementList,
        crate::ts::lists::intersection_type_element_list::FormatTsIntersectionTypeElementList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsIntersectionTypeElementList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsIntersectionTypeElementList,
        crate::ts::lists::intersection_type_element_list::FormatTsIntersectionTypeElementList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsMethodSignatureModifierList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsMethodSignatureModifierList,
        crate::ts::lists::method_signature_modifier_list::FormatTsMethodSignatureModifierList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsMethodSignatureModifierList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsMethodSignatureModifierList,
        crate::ts::lists::method_signature_modifier_list::FormatTsMethodSignatureModifierList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsPropertyParameterModifierList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsPropertyParameterModifierList,
        crate::ts::lists::property_parameter_modifier_list::FormatTsPropertyParameterModifierList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsPropertyParameterModifierList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsPropertyParameterModifierList,
        crate::ts::lists::property_parameter_modifier_list::FormatTsPropertyParameterModifierList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsPropertySignatureModifierList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsPropertySignatureModifierList,
        crate::ts::lists::property_signature_modifier_list::FormatTsPropertySignatureModifierList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsPropertySignatureModifierList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsPropertySignatureModifierList,
        crate::ts::lists::property_signature_modifier_list::FormatTsPropertySignatureModifierList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTemplateElementList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTemplateElementList,
        crate::ts::lists::template_element_list::FormatTsTemplateElementList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsTemplateElementList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTemplateElementList,
        crate::ts::lists::template_element_list::FormatTsTemplateElementList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTupleTypeElementList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTupleTypeElementList,
        crate::ts::lists::tuple_type_element_list::FormatTsTupleTypeElementList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsTupleTypeElementList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTupleTypeElementList,
        crate::ts::lists::tuple_type_element_list::FormatTsTupleTypeElementList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeArgumentList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeArgumentList,
        crate::ts::lists::type_argument_list::FormatTsTypeArgumentList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsTypeArgumentList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeArgumentList,
        crate::ts::lists::type_argument_list::FormatTsTypeArgumentList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeList,
        crate::ts::lists::type_list::FormatTsTypeList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsTypeList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeList,
        crate::ts::lists::type_list::FormatTsTypeList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeMemberList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeMemberList,
        crate::ts::lists::type_member_list::FormatTsTypeMemberList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsTypeMemberList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeMemberList,
        crate::ts::lists::type_member_list::FormatTsTypeMemberList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsTypeParameterList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsTypeParameterList,
        crate::ts::lists::type_parameter_list::FormatTsTypeParameterList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsTypeParameterList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsTypeParameterList,
        crate::ts::lists::type_parameter_list::FormatTsTypeParameterList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsUnionTypeVariantList {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsUnionTypeVariantList,
        crate::ts::lists::union_type_variant_list::FormatTsUnionTypeVariantList,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsUnionTypeVariantList {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsUnionTypeVariantList,
        crate::ts::lists::union_type_variant_list::FormatTsUnionTypeVariantList,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsUnknown> for crate::js::unknown::unknown::FormatJsUnknown {
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsUnknown, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::unknown::unknown::FormatJsUnknown as FormatNodeRule<
            rome_js_syntax::JsUnknown,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsUnknown {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsUnknown,
        crate::js::unknown::unknown::FormatJsUnknown,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsUnknown {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsUnknown,
        crate::js::unknown::unknown::FormatJsUnknown,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsUnknownStatement>
    for crate::js::unknown::unknown_statement::FormatJsUnknownStatement
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsUnknownStatement, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::unknown::unknown_statement::FormatJsUnknownStatement as FormatNodeRule<
            rome_js_syntax::JsUnknownStatement,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsUnknownStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsUnknownStatement,
        crate::js::unknown::unknown_statement::FormatJsUnknownStatement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsUnknownStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsUnknownStatement,
        crate::js::unknown::unknown_statement::FormatJsUnknownStatement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsUnknownExpression>
    for crate::js::unknown::unknown_expression::FormatJsUnknownExpression
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsUnknownExpression, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::unknown::unknown_expression::FormatJsUnknownExpression as FormatNodeRule<
            rome_js_syntax::JsUnknownExpression,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsUnknownExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsUnknownExpression,
        crate::js::unknown::unknown_expression::FormatJsUnknownExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsUnknownExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsUnknownExpression,
        crate::js::unknown::unknown_expression::FormatJsUnknownExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsUnknownMember>
    for crate::js::unknown::unknown_member::FormatJsUnknownMember
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsUnknownMember, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::unknown::unknown_member::FormatJsUnknownMember as FormatNodeRule<
            rome_js_syntax::JsUnknownMember,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsUnknownMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsUnknownMember,
        crate::js::unknown::unknown_member::FormatJsUnknownMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsUnknownMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsUnknownMember,
        crate::js::unknown::unknown_member::FormatJsUnknownMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsUnknownBinding>
    for crate::js::unknown::unknown_binding::FormatJsUnknownBinding
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsUnknownBinding, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::unknown::unknown_binding::FormatJsUnknownBinding as FormatNodeRule<
            rome_js_syntax::JsUnknownBinding,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsUnknownBinding {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsUnknownBinding,
        crate::js::unknown::unknown_binding::FormatJsUnknownBinding,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsUnknownBinding {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsUnknownBinding,
        crate::js::unknown::unknown_binding::FormatJsUnknownBinding,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsUnknownAssignment>
    for crate::js::unknown::unknown_assignment::FormatJsUnknownAssignment
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsUnknownAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::unknown::unknown_assignment::FormatJsUnknownAssignment as FormatNodeRule<
            rome_js_syntax::JsUnknownAssignment,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsUnknownAssignment {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsUnknownAssignment,
        crate::js::unknown::unknown_assignment::FormatJsUnknownAssignment,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsUnknownAssignment {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsUnknownAssignment,
        crate::js::unknown::unknown_assignment::FormatJsUnknownAssignment,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsUnknownParameter>
    for crate::js::unknown::unknown_parameter::FormatJsUnknownParameter
{
    type Context = JsFormatContext;
    fn fmt(node: &rome_js_syntax::JsUnknownParameter, f: &mut JsFormatter) -> FormatResult<()> {
        <crate::js::unknown::unknown_parameter::FormatJsUnknownParameter as FormatNodeRule<
            rome_js_syntax::JsUnknownParameter,
        >>::fmt(node, f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsUnknownParameter {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsUnknownParameter,
        crate::js::unknown::unknown_parameter::FormatJsUnknownParameter,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsUnknownParameter {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsUnknownParameter,
        crate::js::unknown::unknown_parameter::FormatJsUnknownParameter,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsUnknownImportAssertionEntry>
    for crate::js::unknown::unknown_import_assertion_entry::FormatJsUnknownImportAssertionEntry
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsUnknownImportAssertionEntry,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: unknown :: unknown_import_assertion_entry :: FormatJsUnknownImportAssertionEntry as FormatNodeRule < rome_js_syntax :: JsUnknownImportAssertionEntry >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsUnknownImportAssertionEntry {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsUnknownImportAssertionEntry,
        crate::js::unknown::unknown_import_assertion_entry::FormatJsUnknownImportAssertionEntry,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsUnknownImportAssertionEntry {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsUnknownImportAssertionEntry,
        crate::js::unknown::unknown_import_assertion_entry::FormatJsUnknownImportAssertionEntry,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl FormatRule<rome_js_syntax::JsUnknownNamedImportSpecifier>
    for crate::js::unknown::unknown_named_import_specifier::FormatJsUnknownNamedImportSpecifier
{
    type Context = JsFormatContext;
    fn fmt(
        node: &rome_js_syntax::JsUnknownNamedImportSpecifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        < crate :: js :: unknown :: unknown_named_import_specifier :: FormatJsUnknownNamedImportSpecifier as FormatNodeRule < rome_js_syntax :: JsUnknownNamedImportSpecifier >> :: fmt (node , f)
    }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsUnknownNamedImportSpecifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsUnknownNamedImportSpecifier,
        crate::js::unknown::unknown_named_import_specifier::FormatJsUnknownNamedImportSpecifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsUnknownNamedImportSpecifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsUnknownNamedImportSpecifier,
        crate::js::unknown::unknown_named_import_specifier::FormatJsUnknownNamedImportSpecifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyRoot {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsAnyRoot, crate::js::any::root::FormatJsAnyRoot>;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyRoot {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsAnyRoot, crate::js::any::root::FormatJsAnyRoot>;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyExpression,
        crate::js::any::expression::FormatJsAnyExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyExpression,
        crate::js::any::expression::FormatJsAnyExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyStatement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyStatement,
        crate::js::any::statement::FormatJsAnyStatement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyStatement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyStatement,
        crate::js::any::statement::FormatJsAnyStatement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyForInitializer {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyForInitializer,
        crate::js::any::for_initializer::FormatJsAnyForInitializer,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyForInitializer {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyForInitializer,
        crate::js::any::for_initializer::FormatJsAnyForInitializer,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyForInOrOfInitializer {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyForInOrOfInitializer,
        crate::js::any::for_in_or_of_initializer::FormatJsAnyForInOrOfInitializer,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyForInOrOfInitializer {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyForInOrOfInitializer,
        crate::js::any::for_in_or_of_initializer::FormatJsAnyForInOrOfInitializer,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyAssignmentPattern {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyAssignmentPattern,
        crate::js::any::assignment_pattern::FormatJsAnyAssignmentPattern,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyAssignmentPattern {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyAssignmentPattern,
        crate::js::any::assignment_pattern::FormatJsAnyAssignmentPattern,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnySwitchClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnySwitchClause,
        crate::js::any::switch_clause::FormatJsAnySwitchClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnySwitchClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnySwitchClause,
        crate::js::any::switch_clause::FormatJsAnySwitchClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyBindingPattern {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyBindingPattern,
        crate::js::any::binding_pattern::FormatJsAnyBindingPattern,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyBindingPattern {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyBindingPattern,
        crate::js::any::binding_pattern::FormatJsAnyBindingPattern,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyDeclarationClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyDeclarationClause,
        crate::js::any::declaration_clause::FormatJsAnyDeclarationClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyDeclarationClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyDeclarationClause,
        crate::js::any::declaration_clause::FormatJsAnyDeclarationClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyLiteralExpression {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyLiteralExpression,
        crate::js::any::literal_expression::FormatJsAnyLiteralExpression,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyLiteralExpression {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyLiteralExpression,
        crate::js::any::literal_expression::FormatJsAnyLiteralExpression,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyTemplateElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyTemplateElement,
        crate::js::any::template_element::FormatJsAnyTemplateElement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyTemplateElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyTemplateElement,
        crate::js::any::template_element::FormatJsAnyTemplateElement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyBinding {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyBinding,
        crate::js::any::binding::FormatJsAnyBinding,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyBinding {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyBinding,
        crate::js::any::binding::FormatJsAnyBinding,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyArrowFunctionParameters {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyArrowFunctionParameters,
        crate::js::any::arrow_function_parameters::FormatJsAnyArrowFunctionParameters,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyArrowFunctionParameters {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyArrowFunctionParameters,
        crate::js::any::arrow_function_parameters::FormatJsAnyArrowFunctionParameters,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyFunctionBody {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyFunctionBody,
        crate::js::any::function_body::FormatJsAnyFunctionBody,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyFunctionBody {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyFunctionBody,
        crate::js::any::function_body::FormatJsAnyFunctionBody,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyArrayElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyArrayElement,
        crate::js::any::array_element::FormatJsAnyArrayElement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyArrayElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyArrayElement,
        crate::js::any::array_element::FormatJsAnyArrayElement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyName {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsAnyName, crate::js::any::name::FormatJsAnyName>;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyName {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsAnyName, crate::js::any::name::FormatJsAnyName>;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyInProperty {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyInProperty,
        crate::js::any::in_property::FormatJsAnyInProperty,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyInProperty {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyInProperty,
        crate::js::any::in_property::FormatJsAnyInProperty,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyAssignment {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyAssignment,
        crate::js::any::assignment::FormatJsAnyAssignment,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyAssignment {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyAssignment,
        crate::js::any::assignment::FormatJsAnyAssignment,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyObjectMemberName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyObjectMemberName,
        crate::js::any::object_member_name::FormatJsAnyObjectMemberName,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyObjectMemberName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyObjectMemberName,
        crate::js::any::object_member_name::FormatJsAnyObjectMemberName,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyObjectMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyObjectMember,
        crate::js::any::object_member::FormatJsAnyObjectMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyObjectMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyObjectMember,
        crate::js::any::object_member::FormatJsAnyObjectMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyFormalParameter {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyFormalParameter,
        crate::js::any::formal_parameter::FormatJsAnyFormalParameter,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyFormalParameter {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyFormalParameter,
        crate::js::any::formal_parameter::FormatJsAnyFormalParameter,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyClassMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyClassMember,
        crate::js::any::class_member::FormatJsAnyClassMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyClassMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyClassMember,
        crate::js::any::class_member::FormatJsAnyClassMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyClass {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsAnyClass, crate::js::any::class::FormatJsAnyClass>;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyClass {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsAnyClass, crate::js::any::class::FormatJsAnyClass>;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyClassMemberName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyClassMemberName,
        crate::js::any::class_member_name::FormatJsAnyClassMemberName,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyClassMemberName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyClassMemberName,
        crate::js::any::class_member_name::FormatJsAnyClassMemberName,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyConstructorParameter {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyConstructorParameter,
        crate::js::any::constructor_parameter::FormatJsAnyConstructorParameter,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyConstructorParameter {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyConstructorParameter,
        crate::js::any::constructor_parameter::FormatJsAnyConstructorParameter,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyPropertyParameterModifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyPropertyParameterModifier,
        crate::ts::any::property_parameter_modifier::FormatTsAnyPropertyParameterModifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsAnyPropertyParameterModifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAnyPropertyParameterModifier,
        crate::ts::any::property_parameter_modifier::FormatTsAnyPropertyParameterModifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyPropertyAnnotation {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyPropertyAnnotation,
        crate::ts::any::property_annotation::FormatTsAnyPropertyAnnotation,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsAnyPropertyAnnotation {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAnyPropertyAnnotation,
        crate::ts::any::property_annotation::FormatTsAnyPropertyAnnotation,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyPropertyModifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyPropertyModifier,
        crate::js::any::property_modifier::FormatJsAnyPropertyModifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyPropertyModifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyPropertyModifier,
        crate::js::any::property_modifier::FormatJsAnyPropertyModifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyPropertySignatureAnnotation {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyPropertySignatureAnnotation,
        crate::ts::any::property_signature_annotation::FormatTsAnyPropertySignatureAnnotation,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsAnyPropertySignatureAnnotation {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAnyPropertySignatureAnnotation,
        crate::ts::any::property_signature_annotation::FormatTsAnyPropertySignatureAnnotation,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyPropertySignatureModifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyPropertySignatureModifier,
        crate::ts::any::property_signature_modifier::FormatTsAnyPropertySignatureModifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsAnyPropertySignatureModifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAnyPropertySignatureModifier,
        crate::ts::any::property_signature_modifier::FormatTsAnyPropertySignatureModifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyMethodModifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyMethodModifier,
        crate::js::any::method_modifier::FormatJsAnyMethodModifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyMethodModifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyMethodModifier,
        crate::js::any::method_modifier::FormatJsAnyMethodModifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyMethodSignatureModifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyMethodSignatureModifier,
        crate::ts::any::method_signature_modifier::FormatTsAnyMethodSignatureModifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsAnyMethodSignatureModifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAnyMethodSignatureModifier,
        crate::ts::any::method_signature_modifier::FormatTsAnyMethodSignatureModifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyIndexSignatureModifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyIndexSignatureModifier,
        crate::ts::any::index_signature_modifier::FormatTsAnyIndexSignatureModifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsAnyIndexSignatureModifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAnyIndexSignatureModifier,
        crate::ts::any::index_signature_modifier::FormatTsAnyIndexSignatureModifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsType {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::TsType, crate::ts::any::ts_type::FormatTsType>;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsType {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::TsType, crate::ts::any::ts_type::FormatTsType>;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyArrayAssignmentPatternElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyArrayAssignmentPatternElement,
        crate::js::any::array_assignment_pattern_element::FormatJsAnyArrayAssignmentPatternElement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyArrayAssignmentPatternElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyArrayAssignmentPatternElement,
        crate::js::any::array_assignment_pattern_element::FormatJsAnyArrayAssignmentPatternElement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyObjectAssignmentPatternMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyObjectAssignmentPatternMember,
        crate::js::any::object_assignment_pattern_member::FormatJsAnyObjectAssignmentPatternMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyObjectAssignmentPatternMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyObjectAssignmentPatternMember,
        crate::js::any::object_assignment_pattern_member::FormatJsAnyObjectAssignmentPatternMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyArrayBindingPatternElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyArrayBindingPatternElement,
        crate::js::any::array_binding_pattern_element::FormatJsAnyArrayBindingPatternElement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyArrayBindingPatternElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyArrayBindingPatternElement,
        crate::js::any::array_binding_pattern_element::FormatJsAnyArrayBindingPatternElement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyObjectBindingPatternMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyObjectBindingPatternMember,
        crate::js::any::object_binding_pattern_member::FormatJsAnyObjectBindingPatternMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyObjectBindingPatternMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyObjectBindingPatternMember,
        crate::js::any::object_binding_pattern_member::FormatJsAnyObjectBindingPatternMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyDeclaration,
        crate::js::any::declaration::FormatJsAnyDeclaration,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyDeclaration,
        crate::js::any::declaration::FormatJsAnyDeclaration,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyReturnType {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyReturnType,
        crate::ts::any::return_type::FormatTsAnyReturnType,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsAnyReturnType {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAnyReturnType,
        crate::ts::any::return_type::FormatTsAnyReturnType,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyVariableAnnotation {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyVariableAnnotation,
        crate::ts::any::variable_annotation::FormatTsAnyVariableAnnotation,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsAnyVariableAnnotation {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAnyVariableAnnotation,
        crate::ts::any::variable_annotation::FormatTsAnyVariableAnnotation,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyModuleItem {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyModuleItem,
        crate::js::any::module_item::FormatJsAnyModuleItem,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyModuleItem {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyModuleItem,
        crate::js::any::module_item::FormatJsAnyModuleItem,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyImportClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyImportClause,
        crate::js::any::import_clause::FormatJsAnyImportClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyImportClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyImportClause,
        crate::js::any::import_clause::FormatJsAnyImportClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyNamedImport {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyNamedImport,
        crate::js::any::named_import::FormatJsAnyNamedImport,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyNamedImport {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyNamedImport,
        crate::js::any::named_import::FormatJsAnyNamedImport,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyNamedImportSpecifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyNamedImportSpecifier,
        crate::js::any::named_import_specifier::FormatJsAnyNamedImportSpecifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyNamedImportSpecifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyNamedImportSpecifier,
        crate::js::any::named_import_specifier::FormatJsAnyNamedImportSpecifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyImportAssertionEntry {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyImportAssertionEntry,
        crate::js::any::import_assertion_entry::FormatJsAnyImportAssertionEntry,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyImportAssertionEntry {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyImportAssertionEntry,
        crate::js::any::import_assertion_entry::FormatJsAnyImportAssertionEntry,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyExportClause {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyExportClause,
        crate::js::any::export_clause::FormatJsAnyExportClause,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyExportClause {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyExportClause,
        crate::js::any::export_clause::FormatJsAnyExportClause,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyExportDefaultDeclaration {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyExportDefaultDeclaration,
        crate::js::any::export_default_declaration::FormatJsAnyExportDefaultDeclaration,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyExportDefaultDeclaration {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyExportDefaultDeclaration,
        crate::js::any::export_default_declaration::FormatJsAnyExportDefaultDeclaration,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyExportNamedSpecifier {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyExportNamedSpecifier,
        crate::js::any::export_named_specifier::FormatJsAnyExportNamedSpecifier,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyExportNamedSpecifier {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyExportNamedSpecifier,
        crate::js::any::export_named_specifier::FormatJsAnyExportNamedSpecifier,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyFunction {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyFunction,
        crate::js::any::function::FormatJsAnyFunction,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyFunction {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyFunction,
        crate::js::any::function::FormatJsAnyFunction,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyParameter {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyParameter,
        crate::js::any::parameter::FormatJsAnyParameter,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyParameter {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyParameter,
        crate::js::any::parameter::FormatJsAnyParameter,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsAnyCallArgument {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsAnyCallArgument,
        crate::js::any::call_argument::FormatJsAnyCallArgument,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsAnyCallArgument {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsAnyCallArgument,
        crate::js::any::call_argument::FormatJsAnyCallArgument,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyName {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::TsAnyName, crate::ts::any::name::FormatTsAnyName>;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsAnyName {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::TsAnyName, crate::ts::any::name::FormatTsAnyName>;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyModuleReference {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyModuleReference,
        crate::ts::any::module_reference::FormatTsAnyModuleReference,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsAnyModuleReference {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAnyModuleReference,
        crate::ts::any::module_reference::FormatTsAnyModuleReference,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyModuleName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyModuleName,
        crate::ts::any::module_name::FormatTsAnyModuleName,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsAnyModuleName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAnyModuleName,
        crate::ts::any::module_name::FormatTsAnyModuleName,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyExternalModuleDeclarationBody {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyExternalModuleDeclarationBody,
        crate::ts::any::external_module_declaration_body::FormatTsAnyExternalModuleDeclarationBody,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsAnyExternalModuleDeclarationBody {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAnyExternalModuleDeclarationBody,
        crate::ts::any::external_module_declaration_body::FormatTsAnyExternalModuleDeclarationBody,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyTypePredicateParameterName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyTypePredicateParameterName,
        crate::ts::any::type_predicate_parameter_name::FormatTsAnyTypePredicateParameterName,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsAnyTypePredicateParameterName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAnyTypePredicateParameterName,
        crate::ts::any::type_predicate_parameter_name::FormatTsAnyTypePredicateParameterName,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyTypeMember {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyTypeMember,
        crate::ts::any::type_member::FormatTsAnyTypeMember,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsAnyTypeMember {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAnyTypeMember,
        crate::ts::any::type_member::FormatTsAnyTypeMember,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyTupleTypeElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyTupleTypeElement,
        crate::ts::any::tuple_type_element::FormatTsAnyTupleTypeElement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsAnyTupleTypeElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAnyTupleTypeElement,
        crate::ts::any::tuple_type_element::FormatTsAnyTupleTypeElement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::TsAnyTemplateElement {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::TsAnyTemplateElement,
        crate::ts::any::template_element::FormatTsAnyTemplateElement,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::TsAnyTemplateElement {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::TsAnyTemplateElement,
        crate::ts::any::template_element::FormatTsAnyTemplateElement,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxAnyTag {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsxAnyTag, crate::jsx::any::tag::FormatJsxAnyTag>;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxAnyTag {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsxAnyTag, crate::jsx::any::tag::FormatJsxAnyTag>;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxAnyElementName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxAnyElementName,
        crate::jsx::any::element_name::FormatJsxAnyElementName,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxAnyElementName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxAnyElementName,
        crate::jsx::any::element_name::FormatJsxAnyElementName,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxAnyObjectName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxAnyObjectName,
        crate::jsx::any::object_name::FormatJsxAnyObjectName,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxAnyObjectName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxAnyObjectName,
        crate::jsx::any::object_name::FormatJsxAnyObjectName,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxAnyName {
    type Format =
        FormatRefWithRule<'a, rome_js_syntax::JsxAnyName, crate::jsx::any::name::FormatJsxAnyName>;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxAnyName {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsxAnyName, crate::jsx::any::name::FormatJsxAnyName>;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxAnyAttribute {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxAnyAttribute,
        crate::jsx::any::attribute::FormatJsxAnyAttribute,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxAnyAttribute {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxAnyAttribute,
        crate::jsx::any::attribute::FormatJsxAnyAttribute,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxAnyAttributeName {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxAnyAttributeName,
        crate::jsx::any::attribute_name::FormatJsxAnyAttributeName,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxAnyAttributeName {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxAnyAttributeName,
        crate::jsx::any::attribute_name::FormatJsxAnyAttributeName,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxAnyAttributeValue {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxAnyAttributeValue,
        crate::jsx::any::attribute_value::FormatJsxAnyAttributeValue,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxAnyAttributeValue {
    type Format = FormatOwnedWithRule<
        rome_js_syntax::JsxAnyAttributeValue,
        crate::jsx::any::attribute_value::FormatJsxAnyAttributeValue,
    >;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
impl<'a> AsFormat<'a> for rome_js_syntax::JsxAnyChild {
    type Format = FormatRefWithRule<
        'a,
        rome_js_syntax::JsxAnyChild,
        crate::jsx::any::child::FormatJsxAnyChild,
    >;
    fn format(&'a self) -> Self::Format { FormatRefWithRule::new(self) }
}
impl IntoFormat<crate::JsFormatContext> for rome_js_syntax::JsxAnyChild {
    type Format =
        FormatOwnedWithRule<rome_js_syntax::JsxAnyChild, crate::jsx::any::child::FormatJsxAnyChild>;
    fn into_format(self) -> Self::Format { FormatOwnedWithRule::new(self) }
}
