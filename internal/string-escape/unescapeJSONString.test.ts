import {unescapeJSONString} from "@internal/string-escape";
import {test} from "rome";

test(
	"unescapeJSONString",
	(t) => {
		t.is(unescapeJSONString("test \\ud83d\\ude80 rome"), "test \u{1f680} rome");

		// does not handle Unicode code point escapes
		t.throws(() => {
			unescapeJSONString("test \\u{1f680} rome");
		});

		// JSON doesn't support \x escapes
		t.is(unescapeJSONString("\\x0a"), "x0a");

		t.is(unescapeJSONString("\\b \\f \\n \\r \\t \\\\"), "\b \f \n \r \t \\");
	},
);
