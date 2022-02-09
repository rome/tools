mod spec_test;

mod formatter {

    mod js_module {
        use crate::spec_test;
        tests_macros::gen_tests! {"tests/specs/js/module/**/*.js", spec_test::run, "module"}
    }

    mod js_script {
        use crate::spec_test;
        tests_macros::gen_tests! {"tests/specs/js/script/**/*.js", spec_test::run, "script"}
    }
}
