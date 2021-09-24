mod spec_test;

#[test]
pub fn json_boolean() {
	spec_test::spec_test("json/boolean");
}

#[test]
pub fn json_null() {
	spec_test::spec_test("json/null");
}

#[test]
pub fn json_string() {
	spec_test::spec_test("json/string");
}

#[test]
pub fn json_number() {
	spec_test::spec_test("json/number");
}

#[test]
pub fn json_single_line() {
	spec_test::spec_test("json/single_line");
}

#[test]
pub fn json_array() {
	spec_test::spec_test("json/array");
}

#[test]
pub fn json_key_value() {
	spec_test::spec_test("json/key_value");
}

#[test]
pub fn json_multi_line() {
	spec_test::spec_test("json/multi_line");
}

#[test]
pub fn json_int1() {
	spec_test::spec_test("json/int1");
}
