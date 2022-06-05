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
        formatter: &JsFormatter,
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
        // `standaloneSpecifiers` corresponding our `JsDefaultImportSpecifier` + part of `JsNamespaceImportSpecifier`,
        // `groupedSpecifiers` corresponding our `JsNamedImportSpecifiers`

        //  Here we use an opposite way of thinking, we only thinking about the way that can not break
        // That's to say
        // 1. `default_specifier` need to be none.
        // 2. length of `JsNamedImportSpecifiers` at least is one
        // 3. Surrounding of the only `JsNamedImportSpecifiers` should not have any comments
        let formatted_named_import = if default_specifier.is_some() {
            // `can_break` is true.
            formatted![formatter, [named_import.format()]]
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
                                formatted![formatter, [named_import.format()]]
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
                        _ => formatted![formatter, [named_import.format()]],
                    }
                }
                _ => formatted![formatter, [named_import.format()]],
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
