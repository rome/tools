import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"jsx a11y aria props",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// VALID
				"<div />",
				"<div></div>",
				// needs to have "aria-"
				"<div aria='wee'></div>",
				"<div abcARIAdef='true'></div>",
				"<div fooaria-foobar='true'></div>",
				"<div fooaria-hidden='true'></div>",
				"<input type='text' aria-errormessage='foobar' />",
				// INVALID
				"<input className='' aria-labell='' />",
				"<div aria-='foobar' />",
				"<div aria-labeledby='foobar' />",
				"<div aria-skldjfaria-klajsd='foobar' />",
			],
			{category: "lint/jsx-a11y/ariaProps"},
		);
	},
);
