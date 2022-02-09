use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    empty_element, format_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::TsPropertySignatureTypeMember;

impl ToFormatElement for TsPropertySignatureTypeMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let readonly = self.readonly_token().format_or_empty(formatter)?;
        let name = self.name().format(formatter)?;
        let type_annotation = self.type_annotation().format_or_empty(formatter)?;
        let optional = self.optional_token_token().format_or_empty(formatter)?;
        // Here, we can have two kind of separators: `,`, `;` or ASI.
        // Because of how the grammar crafts the nodes, the parent will add the separator to the node.
        // So here, we create - on purpose - an empty node.
        let separator = if let Some(separator) = self.separator_token() {
            formatter.format_replaced(&separator, empty_element())?
        } else {
            empty_element()
        };
        Ok(format_elements![
            readonly,
            space_token(),
            name,
            optional,
            type_annotation,
            separator
        ])
    }
}
