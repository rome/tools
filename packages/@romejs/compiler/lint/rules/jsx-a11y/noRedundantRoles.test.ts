import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"jsx-a11y no redundant roles",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				"<article role='article'></article>",
				"<button role='button'></button>",
				"<h1 role='heading' aria-level='1'></h1>",
				"<h1 role='heading'></h1>",
				"<dialog role='dialog'></dialog>",
				"<input  type='checkbox' role='checkbox' />",
				// VALID
				"<article role='presentation' ></article>",
				"<Button role='button' ></Button>",
				"<span></span>",
			],
			{category: "lint/jsx-a11y/noRedundantRoles"},
		);
	},
);
