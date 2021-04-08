import {unescapeString} from "@internal/string-escape";
import {test} from "rome";

test(
	"unescapeString",
	(t) => {
		t.is(
			unescapeString("test \\ud83d\\ude80 rome", {mode: "json"}),
			"test \u{1f680} rome",
		);

		// does not handle Unicode code point escapes
		t.throws(() => {
			unescapeString("test \\u{1f680} rome", {mode: "json"});
		});

		// JSON doesn't support \x escapes
		t.is(unescapeString("\\x0a", {mode: "json"}), "x0a");

		t.is(
			unescapeString("\\b \\f \\n \\r \\t \\\\", {mode: "json"}),
			"\b \f \n \r \t \\",
		);
	},
);
