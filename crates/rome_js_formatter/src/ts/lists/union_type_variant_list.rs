use crate::prelude::*;
use crate::ts::expressions::template_literal_type::FormatTsTemplateLiteralType;
use crate::ts::module::import_type::FormatTsImportType;
use crate::ts::types::any_type::FormatTsAnyType;
use crate::ts::types::array_type::FormatTsArrayType;
use crate::ts::types::big_int_literal_type::FormatTsBigIntLiteralType;
use crate::ts::types::bigint_type::FormatTsBigintType;
use crate::ts::types::boolean_literal_type::FormatTsBooleanLiteralType;
use crate::ts::types::boolean_type::FormatTsBooleanType;
use crate::ts::types::conditional_type::FormatTsConditionalType;
use crate::ts::types::constructor_type::FormatTsConstructorType;
use crate::ts::types::function_type::FormatTsFunctionType;
use crate::ts::types::indexed_access_type::FormatTsIndexedAccessType;
use crate::ts::types::infer_type::FormatTsInferType;
use crate::ts::types::intersection_type::FormatTsIntersectionType;
use crate::ts::types::mapped_type::FormatTsMappedType;
use crate::ts::types::never_type::FormatTsNeverType;
use crate::ts::types::non_primitive_type::FormatTsNonPrimitiveType;
use crate::ts::types::null_literal_type::FormatTsNullLiteralType;
use crate::ts::types::number_literal_type::FormatTsNumberLiteralType;
use crate::ts::types::number_type::FormatTsNumberType;
use crate::ts::types::object_type::FormatTsObjectType;
use crate::ts::types::parenthesized_type::FormatTsParenthesizedType;
use crate::ts::types::reference_type::FormatTsReferenceType;
use crate::ts::types::string_literal_type::FormatTsStringLiteralType;
use crate::ts::types::string_type::FormatTsStringType;
use crate::ts::types::symbol_type::FormatTsSymbolType;
use crate::ts::types::this_type::FormatTsThisType;
use crate::ts::types::tuple_type::FormatTsTupleType;
use crate::ts::types::type_operator_type::FormatTsTypeOperatorType;
use crate::ts::types::typeof_type::FormatTsTypeofType;
use crate::ts::types::undefined_type::FormatTsUndefinedType;
use crate::ts::types::union_type::FormatTsUnionType;
use crate::ts::types::unknown_type::FormatTsUnknownType;
use crate::ts::types::void_type::FormatTsVoidType;
use crate::utils::should_hug_type;
use crate::JsCommentStyle;
use rome_formatter::{comments::CommentStyle, write};
use rome_js_syntax::{JsLanguage, TsType, TsUnionType, TsUnionTypeVariantList};
use rome_rowan::{AstSeparatedElement, AstSeparatedList};

#[derive(Debug, Clone, Default)]
pub struct FormatTsUnionTypeVariantList;

impl FormatRule<TsUnionTypeVariantList> for FormatTsUnionTypeVariantList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &TsUnionTypeVariantList, f: &mut JsFormatter) -> FormatResult<()> {
        // ```ts
        // {
        //   a: string
        // } | null | void
        // ```
        // should be inlined and not be printed in the multi-line variant
        let should_hug = node
            .parent::<TsType>()
            .as_ref()
            .map_or(false, should_hug_type);

        let last_index = node.len().saturating_sub(1);

        f.join_with(space())
            .entries(
                node.elements()
                    .enumerate()
                    .map(|(index, item)| FormatTypeVariant {
                        last: index == last_index,
                        list: node,
                        element: item,
                        should_hug,
                    }),
            )
            .finish()
    }
}

pub struct FormatTypeVariant<'a> {
    last: bool,
    should_hug: bool,
    element: AstSeparatedElement<JsLanguage, TsType>,
    list: &'a TsUnionTypeVariantList,
}

impl Format<JsFormatContext> for FormatTypeVariant<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let separator = self.element.trailing_separator()?;

        let node = self.element.node()?;

        let is_suppressed = is_type_suppressed(node, self.list, f.comments());

        // This is a hack: It by passes the regular format node to only format the node without its comments.
        let format_node = format_with(|f: &mut JsFormatter| {
            if is_suppressed {
                write!(f, [format_suppressed_node(node.syntax()).skip_comments()])
            } else {
                match node {
                    TsType::TsAnyType(ty) => FormatTsAnyType::default().fmt_node(ty, f),
                    TsType::TsArrayType(ty) => FormatTsArrayType::default().fmt_node(ty, f),
                    TsType::TsBigIntLiteralType(ty) => {
                        FormatTsBigIntLiteralType::default().fmt_node(ty, f)
                    }
                    TsType::TsBigintType(ty) => FormatTsBigintType::default().fmt_node(ty, f),
                    TsType::TsBooleanLiteralType(ty) => {
                        FormatTsBooleanLiteralType::default().fmt_node(ty, f)
                    }
                    TsType::TsBooleanType(ty) => FormatTsBooleanType::default().fmt_node(ty, f),
                    TsType::TsConditionalType(ty) => {
                        FormatTsConditionalType::default().fmt_node(ty, f)
                    }
                    TsType::TsConstructorType(ty) => {
                        FormatTsConstructorType::default().fmt_node(ty, f)
                    }
                    TsType::TsFunctionType(ty) => FormatTsFunctionType::default().fmt_node(ty, f),
                    TsType::TsImportType(ty) => FormatTsImportType::default().fmt_node(ty, f),
                    TsType::TsIndexedAccessType(ty) => {
                        FormatTsIndexedAccessType::default().fmt_node(ty, f)
                    }
                    TsType::TsInferType(ty) => FormatTsInferType::default().fmt_node(ty, f),
                    TsType::TsIntersectionType(ty) => {
                        FormatTsIntersectionType::default().fmt_node(ty, f)
                    }
                    TsType::TsMappedType(ty) => FormatTsMappedType::default().fmt_node(ty, f),
                    TsType::TsNeverType(ty) => FormatTsNeverType::default().fmt_node(ty, f),
                    TsType::TsNonPrimitiveType(ty) => {
                        FormatTsNonPrimitiveType::default().fmt_node(ty, f)
                    }
                    TsType::TsNullLiteralType(ty) => {
                        FormatTsNullLiteralType::default().fmt_node(ty, f)
                    }
                    TsType::TsNumberLiteralType(ty) => {
                        FormatTsNumberLiteralType::default().fmt_node(ty, f)
                    }
                    TsType::TsNumberType(ty) => FormatTsNumberType::default().fmt_node(ty, f),
                    TsType::TsObjectType(ty) => FormatTsObjectType::default().fmt_node(ty, f),
                    TsType::TsParenthesizedType(ty) => {
                        FormatTsParenthesizedType::default().fmt_node(ty, f)
                    }
                    TsType::TsReferenceType(ty) => FormatTsReferenceType::default().fmt_node(ty, f),
                    TsType::TsStringLiteralType(ty) => {
                        FormatTsStringLiteralType::default().fmt_node(ty, f)
                    }
                    TsType::TsStringType(ty) => FormatTsStringType::default().fmt_node(ty, f),
                    TsType::TsSymbolType(ty) => FormatTsSymbolType::default().fmt_node(ty, f),
                    TsType::TsTemplateLiteralType(ty) => {
                        FormatTsTemplateLiteralType::default().fmt_node(ty, f)
                    }
                    TsType::TsThisType(ty) => FormatTsThisType::default().fmt_node(ty, f),
                    TsType::TsTupleType(ty) => FormatTsTupleType::default().fmt_node(ty, f),
                    TsType::TsTypeOperatorType(ty) => {
                        FormatTsTypeOperatorType::default().fmt_node(ty, f)
                    }
                    TsType::TsTypeofType(ty) => FormatTsTypeofType::default().fmt_node(ty, f),
                    TsType::TsUndefinedType(ty) => FormatTsUndefinedType::default().fmt_node(ty, f),
                    TsType::TsUnionType(ty) => FormatTsUnionType::default().fmt_node(ty, f),
                    TsType::TsUnknownType(ty) => FormatTsUnknownType::default().fmt_node(ty, f),
                    TsType::TsVoidType(ty) => FormatTsVoidType::default().fmt_node(ty, f),
                }
            }
        });

        write!(f, [format_leading_comments(node.syntax())])?;

        if self.should_hug {
            write!(f, [format_node])?;
        } else {
            write!(f, [align(2, &format_node)])?;
        }

        if !is_suppressed {
            write!(f, [format_dangling_comments(node.syntax())])?;
        }

        write!(f, [format_trailing_comments(node.syntax())])?;

        if let Some(token) = separator {
            if self.last {
                write!(f, [format_removed(token)])?;
            } else {
                if self.should_hug {
                    write!(f, [space()])?;
                } else {
                    write!(f, [soft_line_break_or_space()])?;
                }
                write![f, [token.format()]]?;
            }
        }

        Ok(())
    }
}

fn is_type_suppressed(ty: &TsType, list: &TsUnionTypeVariantList, comments: &JsComments) -> bool {
    comments.mark_suppression_checked(ty.syntax());

    if let TsType::TsUnionType(union) = ty {
        // If the union isn't empty, than the suppression applies to the first variant
        if !union.types().is_empty() {
            return false;
        }
    }

    // Otherwise check if the node has a suppression in its leading or dangling comments
    // before then checking the previous variants trailing "OwnLine" comments (comments that are on their own line)
    let leading_dangling = comments
        .leading_comments(ty.syntax())
        .iter()
        .chain(comments.dangling_comments(ty.syntax()));

    for comment in leading_dangling {
        if JsCommentStyle::is_suppression(comment.piece().text()) {
            return true;
        }
    }

    for comment in comments
        .trailing_comments(ty.syntax())
        .iter()
        .take_while(|comment| comment.lines_before() == 0)
    {
        if JsCommentStyle::is_suppression(comment.piece().text()) {
            return true;
        }
    }

    // Test if the preceding node as a trailing own line comment that is a suppression
    if let Some(preceding_variant) = ty.syntax().prev_sibling() {
        comments
            .trailing_comments(&preceding_variant)
            .iter()
            .skip_while(|comment| comment.lines_before() == 0)
            .any(|comment| JsCommentStyle::is_suppression(comment.piece().text()))
    }
    // If this is the first variant, then see if the union has a leading suppression comment.
    else if let Some(union) = list.parent::<TsUnionType>() {
        comments
            .leading_comments(union.syntax())
            .iter()
            .any(|comment| JsCommentStyle::is_suppression(comment.piece().text()))
    } else {
        false
    }
}
