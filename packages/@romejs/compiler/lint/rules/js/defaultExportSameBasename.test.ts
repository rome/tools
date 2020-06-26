import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"defaultExportSameBasename",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"export default function test() {}\n",
					"export default class Test {}\n",
				],
				valid: [
					"export default function foo() {}\n",
					"export default class Foo {}\n",
					"export default 'rome';\n",
				],
			},
			{category: "lint/js/defaultExportSameBasename", path: "foo.ts"},
		);
	},
);
