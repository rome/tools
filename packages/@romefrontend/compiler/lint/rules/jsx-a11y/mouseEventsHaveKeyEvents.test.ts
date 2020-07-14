import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"jsx-a11y mouse events have key events",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"<div onMouseOver={ () => void 0 } />",
					"<div onMouseOut={ () => void 0 } />",
					"<div onMouseOver={ () => void 0 } >{props}</div>",
					"<div onMouseOut={ () => void 0 } >{props}</div>",
				],
				valid: [
					"<div onMouseOver={ () => void 0 } onFocus={() => void 0} />",
					"<div onMouseOut={ () => void 0 } onBlur={() => void 0} />",
					"<div onMouseOver={ () => void 0 }  onFocus={() => void 0} >{props}</div>",
					"<div onMouseOut={ () => void 0 }  onBlur={() => void 0} >{props}</div>",
				],
				filename: "file.tsx",
				category: "lint/jsx-a11y/mouseEventsHaveKeyEvents",
			},
		);
	},
);
