import {escapeJSString} from "@internal/string-escape";
import {test} from "rome";

test(
	"escapeJSString",
	(t) => {
		t.is(escapeJSString("test' rome"), "test' rome");
		t.is(escapeJSString("test' rome", {quote: "'"}), "'test\\' rome'");

		t.is(escapeJSString("\u{1f680} \n \\"), "\\u{1f680} \\n \\\\");
		t.is(
			escapeJSString("\u{1f680} \n \\", {json: true}),
			"\\ud83d\\ude80 \\n \\\\",
		);
		t.is(
			escapeJSString("\u{1f680} \n \\", {ignoreWhitespaceEscapes: true}),
			"\\u{1f680} \n \\\\",
		);
		t.is(
			escapeJSString("\u{1f680} \n \\", {unicodeOnly: true}),
			"\\u{1f680} \\x0a \\x5c",
		);
	},
);
