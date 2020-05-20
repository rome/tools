import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"jsx a11y anchor has content",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				"<a />",
				"<a><TextWrapper aria-hidden /></a>",
				// VALID
				"<a>Anchor Content!</a>",
				"<a><TextWrapper /></a>",
				"<a dangerouslySetInnerHTML={{ __html: 'foo' }} />",
			],
			{category: "lint/jsx-a11y/anchorHasContent"},
		);
	},
);
