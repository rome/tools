import {test} from "rome";
import {tests} from "./tests";
import {testLint} from "../utils/testing";
import {DiagnosticCategory} from "@romefrontend/diagnostics";
import {dedent} from "@romefrontend/string-utils";

for (const name in tests) {
	const {invalid, valid, path} = tests[name];

	test(
		name,
		async (t) => {
			await testLint(
				t,
				{
					invalid: invalid ? invalid.map((str) => dedent(str)) : [],
					valid: valid ? valid.map((str) => dedent(str)) : [],
				},
				{
					category: (`lint/${name}` as DiagnosticCategory),
					snapshotFilename: `${name}.test.md`,
					path,
				},
			);
		},
	);
}
