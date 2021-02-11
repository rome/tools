import {createRelativeFilePath} from "@internal/path";
import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"ts prefer interfaces",
	async (t) => {
		await testLint(
			t,
			{
				path: createRelativeFilePath("file.ts"),
				category: "lint/ts/useInterfaces",
			},
		);
	},
);
