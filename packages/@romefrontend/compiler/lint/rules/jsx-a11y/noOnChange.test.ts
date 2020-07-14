import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"jsx-a11y no onChange",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"<select onChange={() => {}} />;",
					"<select onChange={handleOnChange} />;",
					"<option onChange={() => {}} />",
					"<option onChange={() => {}} {...props} />",
				],
				valid: [
					"<select onBlur={() => {}} />;",
					"<select onBlur={handleOnBlur} />;",
					"<option />;",
					"<option onBlur={() => {}} onChange={() => {}} />;",
					"<option {...props} />",
					"<input onChange={() => {}} />;",
					"<input onChange={handleOnChange} />;",
					"<input />;",
					"<input onChange={() => {}} onChange={() => {}} />;",
					"<input {...props} />",
				],
				filename: "file.tsx",
				category: "lint/jsx-a11y/noOnChange",
			},
		);
	},
);
