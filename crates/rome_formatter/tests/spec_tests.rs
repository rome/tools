mod spec_test;

mod formatter {

    mod js_module {
        #[cfg(feature = "test")]
        use crate::spec_test;
        #[cfg(feature = "test")]
        tests_macros::gen_tests! {"tests/specs/js/module/**/**/**/*.js", spec_test::test_formatter::run, "module"}
    }

    mod js_script {
        #[cfg(feature = "test")]
        use crate::spec_test;
        #[cfg(feature = "test")]
        tests_macros::gen_tests! {"tests/specs/js/script/**/**/**/*.js", spec_test::test_formatter::run, "script"}
    }
}
