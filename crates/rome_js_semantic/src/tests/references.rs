use crate::assert_semantics;

assert_semantics! {
    ok_reference_read_global, "let a/*#A*/ = 1; let b = a/*READ A*/ + 1;",

    ok_reference_read_inner_scope, r#"function f(a/*#A1*/) {
    let b = a/*READ A1*/ + 1;
    console.log(b);
    if (true) {
        let a/*#A2*/ = 2;
        let b = a/*READ A2*/ + 1;
        console.log(b);
    }
    let c = a/*READ A1*/ + 1;
    console.log(b);
}
f(1);"#,
}

// hoisting
assert_semantics! {
    ok_hoisting_inside_function, "function f() {
    a = 2;
    let b = a/*READ A*/ + 1;
    console.log(a, b);
    
    var a/*#A*/;
}
f();",
    ok_hoisting_var_inside_if, r#"function f() {
    a = 1;
    let b = a/*READ A*/ + 1;
    console.log(a, b);
    if (true) {  
        var a/*#A*/;      
    }
}
f();"#,
    ok_hoisting_redeclaration_before_use, r#"var a/*#A1*/ = 1;
function f() {
    var a/*#A2*/ = 10;
    console.log(a/*READ A2*/);
}
f();"#,

ok_hoisting_redeclaration_after_use, r#"var a/*#A1*/ = 1;
function f() {
    console.log(a/*READ A2*/);
    var a/*#A2*/ = 10;
}
f();"#,

        ok_hoisting_for_of, r#"function f() {
    for (var a/*#A*/ of [1,2,3]) {
        console.log(a/*READ A*/)
    }
    console.log(a/*READ A*/);
}
f()"#,

    ok_hoisting_for_in, r#"function f() {
    for (var a/*#A*/ in [1,2,3]) {
        console.log(a/*READ A*/)
    }
    console.log(a/*READ A*/);
}
f()"#,
}
