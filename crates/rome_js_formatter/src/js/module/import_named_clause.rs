use crate::format_traits::FormatOptional;
use rome_formatter::group_elements;
use rome_formatter::FormatResult;
use rome_js_syntax::JsNamedImportSpecifier;
use rome_js_syntax::JsNamedImportSpecifiers;
use rome_js_syntax::JsSyntaxKind;
use rome_rowan::AstNode;

use crate::{format_elements, space_token, Format, FormatElement, FormatNode, Formatter};

use rome_js_syntax::JsImportNamedClause;
use rome_js_syntax::JsImportNamedClauseFields;

impl FormatNode for JsImportNamedClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsImportNamedClauseFields {
            type_token,
            default_specifier,
            named_import,
            from_token,
            source,
            assertion,
        } = self.as_fields();

        let type_token = type_token
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;

        let source = source.format(formatter)?;

        let default = default_specifier.format_with_or_empty(formatter, |specifier| {
            format_elements![specifier, space_token()]
        })?;
        let from = from_token.format(formatter)?;

        let name = named_import.format(formatter)?;
        let assertion = assertion.format_with_or_empty(formatter, |assertion| {
            format_elements![space_token(), assertion]
        })?;

        // let need_to_wrap_group_elements = {
        //     let named_import = named_import?;
        //     let syntax_node = named_import.syntax();
        //     match syntax_node.kind() {
        //         JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIERS => {
        //             let specifiers =
        //                 JsNamedImportSpecifiers::unwrap_cast(syntax_node.clone()).as_fields();
        //             specifiers.specifiers.syntax().children().count() > 1
        //         }
        //         _ => true,
        //     }
        // };
        Ok(format_elements![
            type_token,
            default,
            // if need_to_wrap_group_elements {
                group_elements(format_elements![
                    name,
                    space_token(),
                    from,
                    space_token(),
                    source,
                    assertion,
                ])
            // } else {
            //     format_elements![name, space_token(), from, space_token(), source, assertion,]
            // }
        ])
    }
}
