use rome_js_syntax::TsType;
use rome_rowan::AstSeparatedList;

/// Utility function that checks if the current type is can categorized as "simple"
pub(crate) fn is_simple_type(ty: &TsType) -> bool {
    if matches!(
        ty,
        TsType::TsAnyType(_)
            | TsType::TsNullLiteralType(_)
            | TsType::TsThisType(_)
            | TsType::TsVoidType(_)
            | TsType::TsNumberType(_)
            | TsType::TsNumberLiteralType(_)
            | TsType::TsBooleanType(_)
            | TsType::TsBooleanLiteralType(_)
            | TsType::TsBigintType(_)
            | TsType::TsBigIntLiteralType(_)
            | TsType::TsStringType(_)
            | TsType::TsStringLiteralType(_)
            | TsType::TsSymbolType(_)
            | TsType::TsTemplateLiteralType(_)
            | TsType::TsNeverType(_)
            | TsType::TsNonPrimitiveType(_)
            | TsType::TsUndefinedType(_)
            | TsType::TsUnknownType(_)
    ) {
        return true;
    }

    if let TsType::TsReferenceType(reference) = ty {
        return reference.type_arguments().is_none();
    }

    false
}

/// Logic ported from [prettier], function `shouldHugType`
///
/// [prettier]: https://github.com/prettier/prettier/blob/main/src/language-js/print/type-annotation.js#L27-L56
pub(crate) fn should_hug_type(ty: &TsType) -> bool {
    if is_simple_type(ty) {
        return true;
    }

    // Checking for unions where all types but one are "void types", so things like `TypeName | null | void`
    if let TsType::TsUnionType(union_type) = ty {
        let mut iter = union_type.types().iter();

        let has_object_type =
            iter.any(|ty| matches!(ty, Ok(TsType::TsObjectType(_) | TsType::TsReferenceType(_))));

        let void_count = union_type
            .types()
            .iter()
            .filter(|node| {
                matches!(
                    node,
                    Ok(TsType::TsVoidType(_) | TsType::TsNullLiteralType(_))
                )
            })
            .count();

        union_type.types().len() - 1 == void_count && has_object_type
    } else {
        false
    }
}
