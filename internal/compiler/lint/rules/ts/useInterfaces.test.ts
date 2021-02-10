import {createUnknownPath} from "@internal/path";
import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"ts prefer interfaces",
	async (t) => {
		await testLint(
			t,
			{
				path: createUnknownPath("file.ts"),
				category: "lint/ts/useInterfaces",
			},
		);
	},
);
