import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";
import {createRelativePath} from "@internal/path";
import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"ts use expect error suppressions",
	async (t) => {
		await testLint(
			t,
			{
				path: createRelativePath("lint_ts_useTsExpectError.ts"),
				category: DIAGNOSTIC_CATEGORIES["lint/ts/useTsExpectError"],
			},
		);
	},
);
