mod quick_test;
mod spec_test;

mod formatter {

    mod json_module {
        tests_macros::gen_tests! {"tests/specs/json/**/*.json", crate::spec_test::run, ""}
    }
}
