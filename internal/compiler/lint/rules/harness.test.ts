import {test} from "rome";
import {tests} from "./tests";
import {testLint} from "../utils/testing";
import {dedent} from "@internal/string-utils";
import {createPath} from "@internal/path";

for (const name in tests) {
	test(
		name,
		async (t) => {
			t.extendTimeout(10_000);
			const def = tests[name];
			const {category} = def;

			let {cases} = def;
			if (!Array.isArray(cases)) {
				cases = [cases];
			}

			for (const singleCase of cases) {
				if (Array.isArray(singleCase)) {
					for (const {filename, invalid, valid} of singleCase) {
						await testLint(
							t,
							{
								invalid: invalid ? invalid.map((str) => dedent(str)) : [],
								valid: valid ? valid.map((str) => dedent(str)) : [],
								category,
								snapshotFilename: `${name}.test.md`,
								path: createPath(filename),
							},
						);
					}
				} else {
					const {filename, invalid, valid} = singleCase;
					await testLint(
						t,
						{
							invalid: invalid ? invalid.map((str) => dedent(str)) : [],
							valid: valid ? valid.map((str) => dedent(str)) : [],
							category,
							snapshotFilename: `${name}.test.md`,
							path: createPath(filename),
						},
					);
				}
			}
		},
	);
}
