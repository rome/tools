mod spec_test;

mod formatter {
    mod json {
        use crate::spec_test;
        tests_macros::gen_tests! {"tests/specs/json/*.json", spec_test::run, "json"}
    }

    mod js_module {
        use crate::spec_test;
        tests_macros::gen_tests! {"tests/specs/js/module/**.js", spec_test::run, "module"}
    }

    mod js_script {
        use crate::spec_test;
        tests_macros::gen_tests! {"tests/specs/js/script/**.js", spec_test::run, "script"}
    }
}
