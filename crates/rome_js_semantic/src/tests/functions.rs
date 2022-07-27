use crate::assert_semantics;

assert_semantics! {
    ok_function_declaration, "function f/*#F*/ () {}",
    ok_function_call, "function f/*#F*/ () {} f/*READ F*/();",
    ok_function_hoisted_call, "function f/*#F*/ () { g/*READ G*/(); } function g/*#G*/() {}",
    ok_function_inner_function,
        "function b/*#B1*/() { function b/*#B2*/() {console.log(2)}; console.log(1); b/*READ B2*/(); } b/*READ B1*/();",
}
