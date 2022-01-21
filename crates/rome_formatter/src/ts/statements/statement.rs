use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::{ast::JsAnyStatement, AstNode};

impl ToFormatElement for JsAnyStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyStatement::JsBlockStatement(block) => block.to_format_element(formatter),
            JsAnyStatement::JsEmptyStatement(empty_statement) => {
                empty_statement.to_format_element(formatter)
            }
            JsAnyStatement::JsExpressionStatement(expr_stmt) => {
                expr_stmt.to_format_element(formatter)
            }
            JsAnyStatement::JsIfStatement(if_stmt) => if_stmt.to_format_element(formatter),
            JsAnyStatement::JsDoWhileStatement(do_while_statement) => {
                do_while_statement.to_format_element(formatter)
            }
            JsAnyStatement::JsWhileStatement(while_statement) => {
                while_statement.to_format_element(formatter)
            }
            JsAnyStatement::JsForStatement(for_stmt) => for_stmt.to_format_element(formatter),
            JsAnyStatement::JsForInStatement(for_in_statement) => {
                for_in_statement.to_format_element(formatter)
            }
            JsAnyStatement::JsContinueStatement(continue_statement) => {
                continue_statement.to_format_element(formatter)
            }
            JsAnyStatement::JsBreakStatement(break_statement) => {
                break_statement.to_format_element(formatter)
            }
            JsAnyStatement::JsReturnStatement(stmt) => stmt.to_format_element(formatter),
            JsAnyStatement::JsWithStatement(with_statement) => {
                with_statement.to_format_element(formatter)
            }
            JsAnyStatement::JsLabeledStatement(label_statement) => {
                label_statement.to_format_element(formatter)
            }
            JsAnyStatement::JsSwitchStatement(switch_statement) => {
                switch_statement.to_format_element(formatter)
            }
            JsAnyStatement::JsThrowStatement(throw_statement) => {
                throw_statement.to_format_element(formatter)
            }
            JsAnyStatement::JsTryStatement(try_statement) => {
                try_statement.to_format_element(formatter)
            }
            JsAnyStatement::JsDebuggerStatement(debugger_statement) => {
                debugger_statement.to_format_element(formatter)
            }

            JsAnyStatement::JsForOfStatement(for_of_statement) => {
                for_of_statement.to_format_element(formatter)
            }
            JsAnyStatement::JsFunctionStatement(statement) => {
                statement.to_format_element(formatter)
            }
            JsAnyStatement::JsClassStatement(statement) => statement.to_format_element(formatter),
            JsAnyStatement::JsVariableStatement(decl) => decl.to_format_element(formatter),
            JsAnyStatement::JsUnknownStatement(unknown_statement) => {
                Ok(formatter.format_verbatim(unknown_statement.syntax()))
            }
            JsAnyStatement::JsTryFinallyStatement(try_finally) => {
                try_finally.to_format_element(formatter)
            }
            JsAnyStatement::TsEnum(_) => todo!(),
        }
    }
}
