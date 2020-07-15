import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"jsx-a11y aria props",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"<input className='' aria-labell='' />",
					"<div aria-='foobar' />",
					"<div aria-labeledby='foobar' />",
					"<div aria-skldjfaria-klajsd='foobar' />",
				],
				valid: [
					"<div />",
					"<div></div>",
					// needs to have "aria-"
					"<div aria='wee'></div>",
					"<div abcARIAdef='true'></div>",
					"<div fooaria-foobar='true'></div>",
					"<div fooaria-hidden='true'></div>",
					"<input type='text' aria-errormessage='foobar' />",
				],
				category: "lint/jsx-a11y/ariaProps",
				filename: "file.tsx",
			},
		);
	},
);
