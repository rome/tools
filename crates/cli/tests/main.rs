use std::{env, path::PathBuf, process::Command};

fn cargo_bin(name: &str) -> PathBuf {
	env::current_exe()
		.ok()
		.map(|mut path| {
			path.pop();
			if path.ends_with("deps") {
				path.pop();
			}
			path.join(name)
		})
		.expect("cannot get current exe")
}

#[test]
fn test_format_cli() {
	let res = Command::new(cargo_bin("cli"))
		.args(&["format", "fixtures/input.json"])
		.output()
		.expect("fail to run cli format");

	assert!(res.status.success(), "cli format command failed");
	let output = String::from_utf8(res.stdout).expect("cannot read stdout, not utf8 compliant");

	assert_eq!(
		output,
		r#"Running formatter to:
- file "fixtures/input.json"
- with options Tab
{"string": "foo", "boolean": false, "number": 15, "object": {"something": 15}}

"#
	);
}
