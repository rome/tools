import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"no noninteractive tabindex",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"<div tabIndex='0' ></div>",
					"<div role='article' tabIndex='0' ></div>",
					"<article tabIndex={0} />",
				],
				valid: [
					"<button tabIndex='0' ></button>",
					"<span role='button' tabIndex='0' ></span>",
					"<span role='article' tabIndex='-1' ></span>",
					"<MyButton tabIndex={0} />",
					"<article tabIndex='-1'></article>",
					"<div tabIndex='-1' ></div>",
					"<article tabIndex={-1}></article>",
					"<div tabIndex={-1} ></div>",
					"<div></div>",
					"<button tabindex='-1'></button>",
				],
			},
			{category: "lint/jsx-a11y/noNoninteractiveTabindex"},
		);
	},
);
