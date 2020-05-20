import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"jsx a11y no onChange",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				"<select onChange={() => {}} />;",
				"<select onChange={handleOnChange} />;",
				"<option onChange={() => {}} />",
				"<option onChange={() => {}} {...props} />",
				// VALID
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
			{category: "lint/jsx-a11y/noOnChange"},
		);
	},
);
