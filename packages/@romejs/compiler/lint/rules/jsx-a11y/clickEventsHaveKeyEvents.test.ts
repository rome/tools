import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"jsx-a11y click events have key events",
	async (t) => {
		await testLint(
			t,
			{
				invalid: ["<div onClick={() => {}} />"],
				valid: [
					"<div onClick={() => {}} onKeyDown={this.handleKeyDown} />",
					"<div onClick={() => {}} onKeyUp={this.handleKeyUp} />",
					"<div onClick={() => {}} onKeyPress={this.handleKeyPress} />",
				],
			},
			{category: "lint/jsx-a11y/clickEventsHaveKeyEvents"},
		);
	},
);
