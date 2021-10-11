mod spec_test;

mod formatter {
	mod json {
		use crate::spec_test;
		tests_macros::gen_tests! {"tests/specs/json/*.json", spec_test::run}
	}

	mod js {
		use crate::spec_test;
		tests_macros::gen_tests! {"tests/specs/js/*.js", spec_test::run}
	}
}
