import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"jsx a11y tabIndex no positive",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				"<span tabIndex='5'>foo</span>",
				"<span tabIndex={5}>foo</span>",
				"<span tabIndex={'5'}>foo</span>",
				// VALID
				"<span tabIndex={0}>baz</span>",
				"<span tabIndex={-1}>baz</span>",
				"<span tabIndex='-1'>baz</span>",
				"<span tabIndex='0'>baz</span>",
				"<span tabIndex={dynamic}>baz</span>",
				"<span tabIndex={undefined}>baz</span>",
			],
			{category: "lint/jsx-a11y/tabindexNoPositive"},
		);
	},
);
