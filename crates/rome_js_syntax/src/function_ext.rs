use crate::{
    AnyJsFunction, AnyJsFunctionBody, JsMethodClassMember, JsMethodObjectMember, JsStatementList,
};
use rome_rowan::{declare_node_union, AstNode, SyntaxResult, TextRange};

declare_node_union! {
    pub AnyFunctionLike = AnyJsFunction | JsMethodObjectMember | JsMethodClassMember
}

impl AnyFunctionLike {
    pub fn body(&self) -> SyntaxResult<AnyJsFunctionBody> {
        match self {
            AnyFunctionLike::AnyJsFunction(js_function) => js_function.body(),
            AnyFunctionLike::JsMethodObjectMember(js_object_method) => js_object_method
                .body()
                .map(AnyJsFunctionBody::JsFunctionBody),
            AnyFunctionLike::JsMethodClassMember(js_class_method) => js_class_method
                .body()
                .map(AnyJsFunctionBody::JsFunctionBody),
        }
    }

    pub fn is_generator(&self) -> bool {
        match self {
            AnyFunctionLike::AnyJsFunction(any_js_function) => any_js_function.is_generator(),
            AnyFunctionLike::JsMethodClassMember(method_class_member) => {
                method_class_member.star_token().is_some()
            }
            AnyFunctionLike::JsMethodObjectMember(method_obj_member) => {
                method_obj_member.star_token().is_some()
            }
        }
    }

    pub fn name_range(&self) -> Option<TextRange> {
        match self {
            AnyFunctionLike::AnyJsFunction(js_function) => {
                js_function.id().ok().flatten().map(|id| id.range())
            }
            AnyFunctionLike::JsMethodObjectMember(js_object_method) => {
                js_object_method.name().ok().map(|name| name.range())
            }
            AnyFunctionLike::JsMethodClassMember(js_class_method) => {
                js_class_method.name().ok().map(|name| name.range())
            }
        }
    }

    pub fn statements(&self) -> Option<JsStatementList> {
        Some(match self {
            AnyFunctionLike::AnyJsFunction(any_js_function) => any_js_function
                .body()
                .ok()?
                .as_js_function_body()?
                .statements(),
            AnyFunctionLike::JsMethodClassMember(method_class_member) => {
                method_class_member.body().ok()?.statements()
            }
            AnyFunctionLike::JsMethodObjectMember(method_obj_member) => {
                method_obj_member.body().ok()?.statements()
            }
        })
    }
}
