import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"jsx-a11y aria proptypes",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					// mixed
					"<span role='checkbox' aria-checked='test' ></span>",
					// token
					"<span aria-autocomplete='test' ></span>",
					"<span aria-invalid='foo'></span>",
					// id
					"<span aria-errormessage='' ></span>",
					// tokenlist
					"<span aria-relevant='fancy' ></span>",
					// idlist
					"<span aria-labelledby='' ></span>",
					"<span aria-labelledby={``} ></span>",
					// id
					"<span aria-details='' ></span>",
				],
				valid: [
					// various
					"<span role='checkbox' aria-checked={checked} ></span>",

					// mixed
					"<span role='checkbox' aria-checked='true' ></span>",
					"<span role='checkbox' aria-checked={true} ></span>",
					"<span role='checkbox' aria-checked='false' ></span>",
					"<span role='checkbox' aria-checked='mixed' ></span>",

					// token
					"<span role='checkbox' aria-autocomplete='both' ></span>",
					"<span role='checkbox' aria-autocomplete='inline' ></span>",
					"<span role='checkbox' aria-autocomplete='list' ></span>",
					"<span role='checkbox' aria-autocomplete='none' ></span>",
					"<span aria-invalid='true'></span>",
					"<span aria-invalid='grammar'></span>",
					"<span aria-invalid='false'></span>",
					"<span aria-invalid={false}></span>",
					// id
					"<span role='checkbox' aria-errormessage='someid' ></span>",
					// tokenlist
					"<span role='checkbox' aria-relevant='additions' ></span>",
					"<span role='checkbox' aria-relevant='additions all' ></span>",
					// idlist
					"<span aria-labelledby='id' ></span>",
					"<span aria-labelledby='fooId barId' ></span>",

					// id
					"<span aria-details='someid' ></span>",
				],
				filename: "file.tsx",
				category: "lint/jsx-a11y/ariaProptypes",
			},
		);
	},
);
