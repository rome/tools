use crate::assert_semantics;

// Statements
assert_semantics! {
    ok_scope_if, "if(true) {/*START A*/}/*END A*/",
    ok_scope_if_no_block, "if(true) ;/*NOEVENT*/;",
    ok_scope_if_without_block_else_with_block, "if(true) ;/*NOEVENT*/ else {/*START A*/}/*END A*/;",
    ok_scope_if_without_block_else_without_block, "if(true) ;/*NOEVENT*/ else ;/*NOEVENT*/;",
    ok_scope_for_with_block, ";for/*START A*/(;;) {/*START B*/}/*END A*//*END B*/;",
    ok_scope_for_without_block, "for/*START A*/(;;) ;/*END A*//*UNIQUE*/;",
    ok_scope_for_of, "for/*START A*/(const a of []) {/*START B*/}/*END A*//*END B*/;",
    ok_scope_for_of_without_block, "for/*START A*/(const a of []) ;/*END A*//*UNIQUE*/;",
    ok_scope_for_in, "for/*START A*/(const a in []) {/*START B*/}/*END A*//*END B*/;",
    ok_scope_for_in_without_block, "for/*START A*/(const a in []) ;/*END A*//*UNIQUE*/;",
    ok_scope_try_catch, "try {/*START A*/}/*END A*/ catch/*START B*/ (e) {}/*END B*/",
    ok_scope_try_catch_finally, "try {/*START A*/}/*END A*/ catch/*START B1*/ (e) {/*START B2*/}/*END B1*//*END B2*/ finally {/*START C*/}/*END C*/",
}

// Functions
assert_semantics! {
    ok_scope_function, ";function/*START A*/ f() {}/*END A*/",
    ok_scope_function_expression, ";var a = function/*START A*/ f() {}/*END A*/",

    ok_scope_arrow_function, ";(/*START A*/) => {}/*END A*/",
}

// Classes
assert_semantics! {
    ok_scope_class_constructor, ";class A { constructor/*START A*/ () {}/*END A*/ }",
    ok_scope_class_getter, ";class A { get/*START A*/ name() {}/*END A*/ }",
    ok_scope_class_setter, ";class A { set/*START A*/ name(v) {}/*END A*/ }",
}

// Others
assert_semantics! {
    ok_scope_global, "/*START GLOBAL*//*END GLOBAL*/",
}
