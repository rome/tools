import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"jsx-a11y anchor has content",
	async (t) => {
		await testLint(
			t,
			{
				invalid: ["<a />", "<a><TextWrapper aria-hidden /></a>"],
				valid: [
					"<a>Anchor Content!</a>",
					"<a><TextWrapper /></a>",
					"<a dangerouslySetInnerHTML={{ __html: 'foo' }} />",
					"<a><TextWrapper aria-hidden /> visible content</a>",
				],
			},
			{category: "lint/jsx-a11y/anchorHasContent"},
		);
	},
);
