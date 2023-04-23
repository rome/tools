#![allow(non_snake_case)]

mod spec_test;

mod ok {
    //! Tests that must pass according to the JSON specification
    tests_macros::gen_tests! {"tests/md_test_suite/ok/*.md", crate::spec_test::run, "ok"}
}

mod err {
    //! Tests that must fail according to the JSON specification
    tests_macros::gen_tests! {"tests/md_test_suite/err/*.md", crate::spec_test::run, "error"}
}

mod undefined {
    //! parsers are free to accept or reject content
    tests_macros::gen_tests! {"tests/md_test_suite/undefined/*.md", crate::spec_test::run, "undefined"}
}
