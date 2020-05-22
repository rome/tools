import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"jsx a11y has caption",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				"<audio {...props} />",
				"<video {...props} />",
				"<audio>child</audio>",
				"<video>child</audio>",
				// VALID
				"<audio><track kind='captions' {...props} /></audio>",
				"<video><track kind='captions' {...props} /></video>",
				"<video muted {...props} ></video>",
			],
			{category: "lint/jsx-a11y/mediaHasCaption"},
		);
	},
);
