//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_banned_types;
mod no_confusing_arrow;
mod no_duplicate_jsx_props;
mod no_for_each;
mod no_self_assign;
mod no_static_only_class;
mod use_grouped_type_import;
mod use_heading_content;
mod use_is_nan;
mod use_literal_enum_members;
mod use_literal_keys;
mod use_simple_number_keys;
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: no_banned_types :: NoBannedTypes , self :: no_confusing_arrow :: NoConfusingArrow , self :: no_duplicate_jsx_props :: NoDuplicateJsxProps , self :: no_for_each :: NoForEach , self :: no_self_assign :: NoSelfAssign , self :: no_static_only_class :: NoStaticOnlyClass , self :: use_grouped_type_import :: UseGroupedTypeImport , self :: use_heading_content :: UseHeadingContent , self :: use_is_nan :: UseIsNan , self :: use_literal_enum_members :: UseLiteralEnumMembers , self :: use_literal_keys :: UseLiteralKeys , self :: use_simple_number_keys :: UseSimpleNumberKeys ,] } }
