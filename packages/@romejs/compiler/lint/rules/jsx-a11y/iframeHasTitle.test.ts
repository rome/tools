import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"jsx-a11y iframe has title",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"<iframe />",
					"<iframe {...props} />",
					'<iframe title="" />',
					'<iframe title={""} />',
					"<iframe title={``} />",
					"<iframe title={undefined} />",
					"<iframe title={false} />",
					"<iframe title={true} />",
					"<iframe title={42} />",
				],
				valid: ['<iframe title="title" />', "<iframe title={title} >"],
			},
			{category: "lint/jsx-a11y/iframeHasTitle"},
		);
	},
);
