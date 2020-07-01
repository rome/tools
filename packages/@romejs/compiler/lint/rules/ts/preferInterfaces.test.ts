import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"ts prefer interfaces",
	async (t) => {
		await testLint(
			t,
			{
				// TODO
			},
			{category: "lint/ts/preferInterfaces"},
		);
	},
);
