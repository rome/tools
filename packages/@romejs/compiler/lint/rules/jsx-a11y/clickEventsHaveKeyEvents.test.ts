import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"jsx-a11y click events have key events",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				"<div onClick={() => {}} />",
				// VALID
				"<div onClick={() => {}} onKeyDown={this.handleKeyDown} />",
				"<div onClick={() => {}} onKeyUp={this.handleKeyUp} />",
				"<div onClick={() => {}} onKeyPress={this.handleKeyPress} />",
			],
			{category: "lint/jsx-a11y/clickEventsHaveKeyEvents"},
		);
	},
);
