import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"jsx-a11y no target blank",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					'<a href="http://external.link" target="_blank">child</a>',
					'<a href={dynamicLink} target="_blank">child</a>',
				],
				valid: [
					'<p href="http://external.link" target="_blank">child</p>',
					'<a href="http://external.link" rel="noreferrer" target="_blank">child</a>',
					'<a href="http://external.link" rel="noopener noreferrer" target="_blank">child</a>',
					'<a href="relative/link" rel="noreferrer" target="_blank">child</a>',
					'<a href="/absolute/link" rel="noreferrer" target="_blank">child</a>',
					'<a href={dynamicLink} rel="noreferrer" target="_blank">child</a>',
				],
			},
			{category: "lint/jsx-a11y/noTargetBlank"},
		);
	},
);
