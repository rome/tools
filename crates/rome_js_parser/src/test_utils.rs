use std::{fmt::Debug, path::Path};

use rome_diagnostics::{file::SimpleFile, termcolor::Buffer, Emitter};
use rome_js_syntax::{JsLanguage, JsSyntaxNode};
use rome_rowan::{AstNode, SyntaxKind, SyntaxSlot};

use crate::Parse;

// This check is used in the parser test to ensure it doesn't emit
// unknown nodes without diagnostics, and in the analyzer tests to
// check the syntax trees resulting from code actions are correct
pub fn has_unknown_nodes_or_empty_slots(node: &JsSyntaxNode) -> bool {
    node.descendants().any(|descendant| {
        let kind = descendant.kind();
        if kind.is_unknown() {
            return true;
        }

        if kind.is_list() {
            return descendant
                .slots()
                .any(|slot| matches!(slot, SyntaxSlot::Empty));
        }

        false
    })
}

pub fn assert_errors_are_absent<T>(program: &Parse<T>, path: &Path)
where
    T: AstNode<Language = JsLanguage> + Debug,
{
    let syntax = program.syntax();
    let debug_tree = format!("{:?}", program.tree());
    let has_missing_children = debug_tree.contains("missing (required)");

    if !program.has_errors() && !has_unknown_nodes_or_empty_slots(&syntax) && !has_missing_children
    {
        return;
    }

    let file = SimpleFile::new(path.to_str().unwrap().to_string(), syntax.to_string());
    let mut emitter = Emitter::new(&file);
    let mut buffer = Buffer::no_color();

    for diagnostic in program.diagnostics() {
        emitter.emit_with_writer(diagnostic, &mut buffer).unwrap();
    }

    panic!("There should be no errors in the file {:?} but the following errors where present:\n{}\n\nParsed tree:\n{:#?}",
        path.display(),
        std::str::from_utf8(buffer.as_slice()).unwrap(),
        &syntax
	);
}
