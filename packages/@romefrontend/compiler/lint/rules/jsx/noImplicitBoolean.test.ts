import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"jsx no implicit boolean",
	async (t) => {
		await testLint(
			t,
			{
				invalid: ["<input disabled />"],
				valid: [
					"<input disabled={false} />",
					"<input disabled={''} />",
					"<input disabled={0} />",
					"<input disabled={undefined} />",
					'<input disabled="false" />',
				],
			},
			{category: "lint/jsx/noImplicitBoolean"},
		);
	},
);
