use std::fmt::Write as _;

use rome_js_parser::parse;

use rome_js_syntax::{JsAnyStatement, JsLanguage, SourceType};
use rome_rowan::{AstNode, SyntaxNode};

fn main() -> anyhow::Result<()> {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut source = String::new();
    for line in stdin.lock().lines() {
        match line {
            Err(_) => break, // with ^Z
            Ok(s) => {
                source.push_str(&s);
                source.push('\n');
            }
        }
    }

    let root = parse(&source, 0, SourceType::js_module());
    let root = root.tree();
    let mut state = StateOfValidity::Unknown;
    let mut module_item_with_validity = vec![];
    let module = root.as_js_module().unwrap();
    for item in module.items().into_iter() {
        if let Some(any_stmt) = item.as_js_any_statement() {
            let syntax = get_syntax_node_from_any_stmt(any_stmt.clone());
            if let Some(leading_trivia) = syntax.first_leading_trivia() {
                let trimmed_text = leading_trivia
                    .text()
                    .trim()
                    .trim_start_matches("//")
                    .trim()
                    .to_lowercase();
                if trimmed_text.starts_with("valid") {
                    state = StateOfValidity::Valid;
                } else if trimmed_text.starts_with("invalid") {
                    state = StateOfValidity::Invalid;
                }
            }
        }
        if state == StateOfValidity::Unknown {
            println!(
                "Ignore generate doc for \n{}\n, because it's validity is unknown",
                item.text()
            );
        } else {
            module_item_with_validity.push((item, state));
        }
    }

    let (valid_items, invalid_items): (Vec<_>, Vec<_>) = module_item_with_validity
        .into_iter()
        .partition(|(_, state)| *state == StateOfValidity::Valid);

    let mut string = String::new();
    writeln!(string, "### Valid\n").unwrap();

    for item in valid_items {
        writeln!(string, "```js").unwrap();
        let syntax_text = item.0.syntax().text().to_string();
        let dedent_text = textwrap::dedent(&syntax_text);
        writeln!(string, "{}", dedent_text.trim()).unwrap();
        writeln!(string, "```").unwrap();
    }
    writeln!(string, "### Invalid\n").unwrap();
    for item in invalid_items {
        writeln!(string, "```js,expect_diagnostic").unwrap();
        let syntax_text = item.0.syntax().text().to_string();
        let dedent_text = textwrap::dedent(&syntax_text);
        writeln!(string, "{}", dedent_text.trim()).unwrap();
        writeln!(string, "```").unwrap();
    }
    println!("{}", string);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StateOfValidity {
    Valid,
    Invalid,
    Unknown,
}

fn get_syntax_node_from_any_stmt(stmt: JsAnyStatement) -> SyntaxNode<JsLanguage> {
    match stmt {
        JsAnyStatement::JsBlockStatement(stmt) => stmt.into_syntax(),
        JsAnyStatement::JsBreakStatement(stmt) => stmt.into_syntax(),
        JsAnyStatement::JsClassDeclaration(stmt) => stmt.into_syntax(),
        JsAnyStatement::JsContinueStatement(stmt) => stmt.into_syntax(),
        JsAnyStatement::JsDebuggerStatement(stmt) => stmt.into_syntax(),
        JsAnyStatement::JsDoWhileStatement(stmt) => stmt.into_syntax(),
        JsAnyStatement::JsEmptyStatement(stmt) => stmt.into_syntax(),
        JsAnyStatement::JsExpressionStatement(stmt) => stmt.into_syntax(),
        JsAnyStatement::JsForInStatement(stmt) => stmt.into_syntax(),
        JsAnyStatement::JsForOfStatement(stmt) => stmt.into_syntax(),
        JsAnyStatement::JsForStatement(stmt) => stmt.into_syntax(),
        JsAnyStatement::JsFunctionDeclaration(stmt) => stmt.into_syntax(),
        JsAnyStatement::JsIfStatement(stmt) => stmt.into_syntax(),
        JsAnyStatement::JsLabeledStatement(stmt) => stmt.into_syntax(),
        JsAnyStatement::JsReturnStatement(stmt) => stmt.into_syntax(),
        JsAnyStatement::JsSwitchStatement(stmt) => stmt.into_syntax(),
        JsAnyStatement::JsThrowStatement(stmt) => stmt.into_syntax(),
        JsAnyStatement::JsTryFinallyStatement(stmt) => stmt.into_syntax(),
        JsAnyStatement::JsTryStatement(stmt) => stmt.into_syntax(),
        JsAnyStatement::JsUnknownStatement(stmt) => stmt.into_syntax(),
        JsAnyStatement::JsVariableStatement(stmt) => stmt.into_syntax(),
        JsAnyStatement::JsWhileStatement(stmt) => stmt.into_syntax(),
        JsAnyStatement::JsWithStatement(stmt) => stmt.into_syntax(),
        JsAnyStatement::TsDeclareFunctionDeclaration(stmt) => stmt.into_syntax(),
        JsAnyStatement::TsDeclareStatement(stmt) => stmt.into_syntax(),
        JsAnyStatement::TsEnumDeclaration(stmt) => stmt.into_syntax(),
        JsAnyStatement::TsExternalModuleDeclaration(stmt) => stmt.into_syntax(),
        JsAnyStatement::TsGlobalDeclaration(stmt) => stmt.into_syntax(),
        JsAnyStatement::TsImportEqualsDeclaration(stmt) => stmt.into_syntax(),
        JsAnyStatement::TsInterfaceDeclaration(stmt) => stmt.into_syntax(),
        JsAnyStatement::TsModuleDeclaration(stmt) => stmt.into_syntax(),
        JsAnyStatement::TsTypeAliasDeclaration(stmt) => stmt.into_syntax(),
    }
}
