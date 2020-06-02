import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"react button has type",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"<button>child</button>",
					'<button type="foo">child</button>',
					"React.createElement('button', {type: 'foo'}, ['child'])",
					"createElement('button', {type: 'foo'}, ['child'])",
				],
				valid: [
					"<div>child</div>",
					'<div type="type">child</div>',
					'<button type="button">child</button>',
					'<button type="submit">child</button>',
					'<button type="reset">child</button>',
					"React.createElement('div', ['child'])",
					"React.createElement('div', {type: 'type'}, ['child'])",
					"React.createElement('button', {type: 'button'}, ['child'])",
					"React.createElement('button', {type: 'submit'}, ['child'])",
					"React.createElement('button', {type: 'reset'}, ['child'])",
					"createElement('div', ['child'])",
					"createElement('div', {type: 'type'}, ['child'])",
					"createElement('button', {type: 'button'}, ['child'])",
					"createElement('button', {type: 'submit'}, ['child'])",
					"createElement('button', {type: 'reset'}, ['child'])",
				],
			},
			{category: "lint/react/buttonHasType"},
		);
	},
);
