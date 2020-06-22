import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"jsx-a11y tabIndex no positive",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"<span tabIndex='5'>foo</span>",
					"<span tabIndex={5}>foo</span>",
					"<span tabIndex={'5'}>foo</span>",
				],
				valid: [
					"<span tabIndex={0}>baz</span>",
					"<span tabIndex={-1}>baz</span>",
					"<span tabIndex='-1'>baz</span>",
					"<span tabIndex='0'>baz</span>",
					"<span tabIndex={dynamic}>baz</span>",
					"<span tabIndex={undefined}>baz</span>",
				],
			},
			{category: "lint/jsx-a11y/tabindexNoPositive"},
		);
	},
);
