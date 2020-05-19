import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"avoid positive tab index",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				"<span tabIndex='5'>foo</span>",
				"<span tabIndex={5}>foo</span>",
				// VALID
				"<span tabIndex={0}>baz</span>",
				"<span tabIndex={-1}>baz</span>",
				"<span tabIndex='-1'>baz</span>",
				"<span tabIndex='0'>baz</span>",
			],
			{category: "lint/jsx-a11y/tabindexNoPositive"},
		);
	},
);
