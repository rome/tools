use crate::generated::FormatTsIntersectionTypeElementList;
use crate::prelude::*;
use crate::ts::lists::union_type_variant_list::FormatTypeVariant;
use rome_js_syntax::TsIntersectionTypeElementList;
use rome_rowan::AstSeparatedList;

impl FormatRule<TsIntersectionTypeElementList> for FormatTsIntersectionTypeElementList {
    type Context = JsFormatContext;

    fn fmt(node: &TsIntersectionTypeElementList, f: &mut JsFormatter) -> FormatResult<()> {
        let last_index = node.len().saturating_sub(1);

        f.join()
            .entries(
                node.elements()
                    .enumerate()
                    .map(|(index, item)| FormatTypeVariant {
                        last: index == last_index,
                        element: item,
                    }),
            )
            .finish()
    }
}
