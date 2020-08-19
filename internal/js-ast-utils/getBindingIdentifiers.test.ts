import {test} from "rome";
import {parseJS} from "@internal/js-parser";
import {dedent} from "@internal/string-utils";
import {getBindingIdentifiers} from "@internal/js-ast-utils/getBindingIdentifiers";

test(
	"verify identified names",
	async (t) => {
		const identifiers = getBindingIdentifiers(
			parseJS({
				path: "unknown",
				input: dedent`
					const foo = "bar";

					function hello() {
						return "world";
					}

					function test() {
						return "passed";
					}
				`,
			}).body,
		);

		// getBindingIdentifiers reverses the array
		t.looksLike(identifiers[0].name, "test");
		t.looksLike(identifiers[1].name, "hello");
		t.looksLike(identifiers[2].name, "foo");
	},
);
