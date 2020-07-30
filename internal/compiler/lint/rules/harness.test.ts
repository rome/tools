import {test} from "rome";
import {tests} from "./tests";
import {testLint} from "../utils/testing";
import {DiagnosticCategory} from "@internal/diagnostics";
import {dedent} from "@internal/string-utils";

for (const name in tests) {
	test(
		name,
		async (t) => {
			let cases = tests[name];

			if (!Array.isArray(cases)) {
				cases = [cases];
			}

			for (const {invalid, valid, filename} of cases) {
				await testLint(
					t,
					{
						invalid: invalid ? invalid.map((str) => dedent(str)) : [],
						valid: valid ? valid.map((str) => dedent(str)) : [],
						category: (`lint/${name}` as DiagnosticCategory),
						snapshotFilename: `${name}.test.md`,
						filename,
					},
				);
			}
		},
	);
}
