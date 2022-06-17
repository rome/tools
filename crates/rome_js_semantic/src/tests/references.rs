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
