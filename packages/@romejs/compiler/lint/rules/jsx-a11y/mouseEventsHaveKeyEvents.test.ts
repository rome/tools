import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"mouse events have key events",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				"<div onMouseOver={ () => void 0 } />",
				"<div onMouseOut={ () => void 0 } />",
				"<div onMouseOver={ () => void 0 } >{props}</div>",
				"<div onMouseOut={ () => void 0 } >{props}</div>",

				// VALID
				"<div onMouseOver={ () => void 0 } onFocus={() => void 0} />",
				"<div onMouseOut={ () => void 0 } onBlur={() => void 0} />",
				"<div onMouseOver={ () => void 0 }  onFocus={() => void 0} >{props}</div>",
				"<div onMouseOut={ () => void 0 }  onBlur={() => void 0} >{props}</div>",
			],
			{category: "lint/jsx-a11y/mouseEventsHaveKeyEvents"},
		);
	},
);
