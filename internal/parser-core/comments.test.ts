import {test} from "rome";
import {parseJS} from "@internal/js-parser";
import {dedent} from "@internal/string-utils";

function helper(input: string) {
	return parseJS({
		path: "unknown",
		input,
	});
}

test(
	"test comment parsing",
	(t) => {
		t.snapshot(helper("/* block comment */ 42"));

		t.snapshot(helper("42 /* block comment 1 */ /* block comment 2 */"));

		t.snapshot(helper("var p1;/* block comment 1 */ /* block comment 2 */"));

		t.snapshot(helper("/*42*/"));

		t.snapshot(helper("(a + /* assignment */b ) * c"));

		t.snapshot(helper("(function(){ var version = 1; /* sync */ }).call(this)"));

		t.snapshot(helper("/**/ function a() {}"));

		t.snapshot(
			helper(
				dedent`
					// Hello, world!

					//   Another hello
					42
				`,
			),
		);

		t.snapshot(
			helper(
				dedent`
					while (true) {
						/**
						 * comments in empty block
						 */
					}
				`,
			),
		);
	},
);
