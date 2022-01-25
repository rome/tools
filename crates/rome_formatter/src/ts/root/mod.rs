use crate::{join_elements_hard_line, FormatElement, Formatter};
use rslint_parser::ast::JsModuleItemList;
use rslint_parser::AstNode;
use rslint_parser::AstNodeList;

mod any_module_item;
mod module;
mod script;

pub fn format_module_item_list(list: JsModuleItemList, formatter: &Formatter) -> FormatElement {
    join_elements_hard_line(list.iter().map(|module_item| {
        let snapshot = formatter.snapshot();
        let elem = match formatter.format_node(module_item.clone()) {
            Ok(result) => result,
            Err(_) => {
                formatter.restore(snapshot);
                formatter
                    .format_verbatim(module_item.syntax())
                    .trim_start()
                    .trim_end()
            }
        };

        (module_item, elem)
    }))
}
