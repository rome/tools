import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"jsx a11y role has required aria props",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"<span role='checkbox'></span>",
					"<span role='switch'></span>",
					"<span role='spinbutton'></span>",
					"<span role='spinbutton' aria-valuemax='100'></span>",
					"<span role='spinbutton' aria-valuemin='0'></span>",
					"<span role='spinbutton' aria-valuemin='0' aria-valuemax='100'></span>",
					"<span role='slider' aria-valuemin='0'></span>",
					"<span role='slider' aria-valuemax='100'></span>",
					"<span role='slider' aria-valuemin='0' aria-valuemax='100'></span>",
					"<span role='separator' aria-valuemin='0'></span>",
					"<span role='separator' aria-valuemax='100'></span>",
					"<span role='separator' aria-valuemin='0' aria-valuemax='100'></span>",
					"<span role='scrollbar' aria-valuemin='0'></span>",
					"<span role='scrollbar' aria-valuemax='100'></span>",
					"<span role='scrollbar' aria-valuemin='0' aria-valuemax='100'></span>",
					"<span role='scrollbar' aria-valuemin='0' aria-valuemax='100' aria-orientation='horizontal'></span>",
					"<span role='radio'></span>",
					"<span role='option'></span>",
					"<span role='heading'></span>",
					"<span role='combobox'></span>",
					"<span role='combobox' aria-expanded='true'></span>",
					"<span role='combobox' aria-controls='true'></span>",
				],
				valid: [
					"<span role='checkbox' aria-checked='true'></span>",
					"<span role='switch' aria-checked='true'></span>",
					"<span role='spinbutton' aria-valuemax='100' aria-valuemin='0' aria-valuenow='50'></span>",
					"<span role='slider' aria-valuemax='100' aria-valuemin='0' aria-valuenow='50'></span>",
					"<span role='separator' aria-valuemax='100' aria-valuemin='0' aria-valuenow='50'></span>",
					"<span role='scrollbar' aria-valuemax='100' aria-valuemin='0' aria-valuenow='50' aria-orientation='horizontal' aria-controls='123'></span>",
					"<span role='radio' aria-checked='true'></span>",
					"<span role='option' aria-selected='true'></span>",
					"<span role='heading' aria-level='1'></span>",
					"<span role='combobox' aria-controls='true' aria-expanded='true'></span>",
				],
			},
			{category: "lint/jsx-a11y/roleHasRequiredAriaProps"},
		);
	},
);
