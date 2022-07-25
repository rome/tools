use crate::assert_semantics;

assert_semantics! {
    ok_function_declaration, "function f/*#F*/ () {}",
    ok_function_call, "function f/*#F*/ () {} f/*READ F*/();",
    ok_function_hoisted_call, "function f/*#F*/ () { g/*READ G*/(); } function g/*#G*/() {}",
}
