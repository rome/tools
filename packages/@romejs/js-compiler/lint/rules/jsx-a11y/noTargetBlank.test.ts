import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	'require anchors with target="_blank" to use rel="noreferrer"',
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				'<a href="http://external.link" target="_blank">child</a>',
				'<a href={dynamicLink} target="_blank">child</a>',
				// VALID
				'<p href="http://external.link" target="_blank">child</p>',
				'<a href="http://external.link" rel="noreferrer" target="_blank">child</a>',
				'<a href="http://external.link" rel="noopener noreferrer" target="_blank">child</a>',
				'<a href="relative/link" rel="noreferrer" target="_blank">child</a>',
				'<a href="/absolute/link" rel="noreferrer" target="_blank">child</a>',
			],
			{category: "lint/jsx-a11y/noTargetBlank"},
		);
	},
);
