import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"audio and video element has caption",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				"<audio {...props} />",
				"<video {...props} />",
				// VALID
				"<audio><track kind='captions' {...props} /></audio>",
				"<video><track kind='captions' {...props} /></video>",
				"<video muted {...props} ></video>",
			],
			{category: "lint/jsx-a11y/mediaHasCaption"},
		);
	},
);
