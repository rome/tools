import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"jsx-a11y has caption",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"<audio {...props} />",
					"<video {...props} />",
					"<audio>child</audio>",
					"<video>child</audio>",
				],
				valid: [
					"<audio><track kind='captions' {...props} /></audio>",
					"<video><track kind='captions' {...props} /></video>",
					"<video muted {...props} ></video>",
				],
				filename: "file.tsx",
				category: "lint/jsx-a11y/mediaHasCaption",
			},
		);
	},
);
