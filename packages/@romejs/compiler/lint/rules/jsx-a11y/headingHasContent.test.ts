import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"jsx a11y heading has content",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				"<h1 />",
				"<h1><TextWrapper aria-hidden /></h1>",
				"<h1><div aria-hidden /></h1>",
				// VALID
				"<h1>heading</h1>",
				"<h1><TextWrapper /></h1>",
				"<h1 dangerouslySetInnerHTML={{ __html: 'heading' }} />",
				"<h1><div aria-hidden />visible content</h1>",
			],
			{category: "lint/jsx-a11y/headingHasContent"},
		);
	},
);
