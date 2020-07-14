import {test} from "rome";
import {testLint} from "../../utils/testing";

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
					"<a><TextWrapper aria-hidden={true} /> visible content</a>",
				],
				category: "lint/jsx-a11y/anchorHasContent",
				filename: "file.tsx",
			},
		);
	},
);
