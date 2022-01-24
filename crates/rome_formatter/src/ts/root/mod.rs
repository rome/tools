use crate::{hard_line_break, join_elements, FormatElement, Formatter};
use rslint_parser::ast::JsModuleItemList;
use rslint_parser::AstNode;
use rslint_parser::AstNodeList;

mod any_module_item;
mod module;
mod script;

pub fn format_module_list(list: JsModuleItemList, formatter: &Formatter) -> FormatElement {
    join_elements(
        hard_line_break(),
        list.iter().map(|module_item| {
            let snapshot = formatter.snapshot();
            match formatter.format_node(module_item.clone()) {
                Ok(result) => result,
                Err(_) => {
                    formatter.restore(snapshot);
                    formatter
                        .format_verbatim(module_item.syntax())
                        .trim_start()
                        .trim_end()
                }
            }
        }),
    )
}
