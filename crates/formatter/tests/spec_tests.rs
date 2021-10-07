mod spec_test;

mod formatter {
	mod json {
		use crate::spec_test;
		tests_macros::gen_tests! {"tests/specs/json/*.json", spec_test::run}
	}
}
