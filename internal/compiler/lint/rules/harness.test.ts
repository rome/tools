import {test} from "rome";
import {tests} from "./tests";
import {testLint} from "../utils/testing";
import {createPath} from "@internal/path";

for (const name in tests) {
	test(
		name,
		async (t) => {
			t.extendTimeout(10_000);
			const def = tests[name];
			const {category} = def;

			for (const singleCase of def.cases) {
				const {filename, invalid, valid} = singleCase;
				await testLint(
					t,
					{
						invalid: invalid ?? [],
						valid: valid ?? [],
						category,
						snapshotFilename: `${name}.test.md`,
						path: createPath(filename),
					},
				);
			}
		},
	);
}
