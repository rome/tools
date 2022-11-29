use rome_js_syntax::{
    AnyTsType, JsSyntaxKind, JsSyntaxNode, TsIntersectionTypeElementList, TsUnionTypeVariantList,
};
use rome_rowan::AstSeparatedList;

use crate::parentheses::{
    is_in_many_type_union_or_intersection_list, operator_type_or_higher_needs_parens,
};

/// Utility function that checks if the current type is object like type
/// ```ts
///     type A = {};
///     type B = {
///         [key in A]: number;
///     };
/// ```
pub(crate) fn is_object_like_type(ty: &AnyTsType) -> bool {
    matches!(ty, AnyTsType::TsMappedType(_) | AnyTsType::TsObjectType(_))
}

/// Utility function that checks if the current type is can categorized as "simple"
pub(crate) fn is_simple_type(ty: &AnyTsType) -> bool {
    if matches!(
        ty,
        AnyTsType::TsAnyType(_)
            | AnyTsType::TsNullLiteralType(_)
            | AnyTsType::TsThisType(_)
            | AnyTsType::TsVoidType(_)
            | AnyTsType::TsNumberType(_)
            | AnyTsType::TsNumberLiteralType(_)
            | AnyTsType::TsBooleanType(_)
            | AnyTsType::TsBooleanLiteralType(_)
            | AnyTsType::TsBigintType(_)
            | AnyTsType::TsBigIntLiteralType(_)
            | AnyTsType::TsStringType(_)
            | AnyTsType::TsStringLiteralType(_)
            | AnyTsType::TsSymbolType(_)
            | AnyTsType::TsTemplateLiteralType(_)
            | AnyTsType::TsNeverType(_)
            | AnyTsType::TsNonPrimitiveType(_)
            | AnyTsType::TsUndefinedType(_)
            | AnyTsType::TsUnknownType(_)
    ) {
        return true;
    }

    if let AnyTsType::TsReferenceType(reference) = ty {
        return reference.type_arguments().is_none();
    }

    false
}

/// Logic ported from [prettier], function `shouldHugType`
///
/// [prettier]: https://github.com/prettier/prettier/blob/main/src/language-js/print/type-annotation.js#L27-L56
pub(crate) fn should_hug_type(ty: &AnyTsType) -> bool {
    if is_simple_type(ty) || is_object_like_type(ty) {
        return true;
    }

    // Checking for unions where all types but one are "void types", so things like `TypeName | null | void`
    if let AnyTsType::TsUnionType(union_type) = ty {
        let mut iter = union_type.types().iter();

        let has_object_type = iter.any(|ty| {
            matches!(
                ty,
                Ok(AnyTsType::TsObjectType(_) | AnyTsType::TsReferenceType(_))
            )
        });

        let void_count = union_type
            .types()
            .iter()
            .filter(|node| {
                matches!(
                    node,
                    Ok(AnyTsType::TsVoidType(_) | AnyTsType::TsNullLiteralType(_))
                )
            })
            .count();

        union_type.types().len() - 1 == void_count && has_object_type
    } else {
        false
    }
}

pub(crate) fn union_or_intersection_type_needs_parentheses(
    node: &JsSyntaxNode,
    parent: &JsSyntaxNode,
    types: &TsIntersectionOrUnionTypeList,
) -> bool {
    debug_assert!(matches!(
        node.kind(),
        JsSyntaxKind::TS_INTERSECTION_TYPE | JsSyntaxKind::TS_UNION_TYPE
    ));

    if is_in_many_type_union_or_intersection_list(node, parent) {
        types.len() > 1
    } else {
        operator_type_or_higher_needs_parens(node, parent)
    }
}

pub(crate) enum TsIntersectionOrUnionTypeList {
    TsIntersectionTypeElementList(TsIntersectionTypeElementList),
    TsUnionTypeVariantList(TsUnionTypeVariantList),
}

impl TsIntersectionOrUnionTypeList {
    fn len(&self) -> usize {
        match self {
            TsIntersectionOrUnionTypeList::TsIntersectionTypeElementList(list) => list.len(),
            TsIntersectionOrUnionTypeList::TsUnionTypeVariantList(list) => list.len(),
        }
    }
}
