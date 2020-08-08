import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"ts prefer interfaces",
	async (t) => {
		await testLint(
			t,
			{
				filename: "file.ts",
				category: "lint/ts/useInterfaces",
			},
		);
	},
);
