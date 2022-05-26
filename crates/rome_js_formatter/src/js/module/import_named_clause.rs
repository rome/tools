use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsAnyNamedImport;
use rome_js_syntax::JsAnyNamedImportSpecifier;
use rome_js_syntax::JsImportNamedClause;
use rome_js_syntax::JsImportNamedClauseFields;
use rome_js_syntax::JsNamedImportSpecifiersFields;

impl FormatNodeFields<JsImportNamedClause> for FormatNodeRule<JsImportNamedClause> {
    fn format_fields(
        node: &JsImportNamedClause,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsImportNamedClauseFields {
            type_token,
            default_specifier,
            named_import,
            from_token,
            source,
            assertion,
        } = node.as_fields();

        let named_import = named_import?;

        // can_break implementation, return `format_element` instead of boolean to reduce enum conversion overhead.
        // if `can_break` is true we just use the previous format strategy, otherwise we use the new format strategy.
        // reference https://github.com/prettier/prettier/blob/5b113e71b1808d6916f446c3aa49c3c53e3bdb98/src/language-js/print/module.js#L173

        // https://github.com/prettier/prettier/blob/5b113e71b1808d6916f446c3aa49c3c53e3bdb98/src/language-js/print/module.js#L184-L209v,
        // `standaloneSpecifiers` corresponding our `default_specifier` + part of `named_import`,
        // `groupedSpecifiers` corresponding our `named_import`
        // https://github.com/prettier/prettier/blob/5b113e71b1808d6916f446c3aa49c3c53e3bdb98/src/language-js/print/module.js#L216-L243
        // According L216-L219 we know that `canBreak` will be true only if 
        let formatted_named_import = if default_specifier.is_some() {
            // `can_break` is true.
            named_import.format().format(formatter)
        } else {
            match named_import {
                JsAnyNamedImport::JsNamedImportSpecifiers(ref specifiers)
                    if specifiers.specifiers().len() == 1 =>
                {
                    // SAFETY: we know that the `specifiers.specifiers().len() == 1`, so unwrap `iter().next()` is safe.
                    let first_specifier = specifiers.specifiers().iter().next().unwrap();
                    match first_specifier {
                        Ok(JsAnyNamedImportSpecifier::JsShorthandNamedImportSpecifier(_)) => {
                            let syntax_node = specifiers.syntax();
                            if syntax_node.has_comments_direct() {
                                named_import.format().format(formatter)
                            } else {
                                let JsNamedImportSpecifiersFields {
                                    l_curly_token,
                                    specifiers: _,
                                    r_curly_token,
                                } = specifiers.as_fields();
                                formatted![
                                    formatter,
                                    [
                                        l_curly_token.format(),
                                        space_token(),
                                        first_specifier.format(),
                                        space_token(),
                                        r_curly_token.format()
                                    ]
                                ]
                            }
                        }
                        _ => named_import.format().format(formatter),
                    }
                }
                _ => named_import.format().format(formatter),
            }
        };

        formatted![
            formatter,
            [
                type_token
                    .format()
                    .with_or_empty(|token| formatted![formatter, [token, space_token()]]),
                default_specifier
                    .format()
                    .with_or_empty(|specifier| formatted![formatter, [specifier, space_token()]]),
                formatted_named_import,
                space_token(),
                from_token.format(),
                space_token(),
                source.format(),
                assertion
                    .format()
                    .with_or_empty(|assertion| formatted![formatter, [space_token(), assertion]])
            ]
        ]
    }
}
