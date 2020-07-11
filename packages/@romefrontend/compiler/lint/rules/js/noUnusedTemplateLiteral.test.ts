import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"js no unused template literal",
	async (t) => {
		await testLint(
			t,
			{
				invalid: ["const foo = `bar`", "const foo = `bar `"],
				valid: [
					"const foo = `bar\n`",
					'const foo = `"bar"`',
					"const foo = `'bar'`",
				],
			},
			{category: "lint/js/noUnusedTemplateLiteral"},
		);
	},
);
