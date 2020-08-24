import {test} from "rome";
import {parseJS} from "@internal/js-parser";
import {dedent} from "@internal/string-utils";
import {hasJSXAttribute} from "@internal/js-ast-utils/hasJSXAttribute";
import {jsExpressionStatement, jsxElement} from "@internal/ast";

test(
	"verify hasJSXAttribute returns the correct value",
	async (t) => {
		const jsx = jsxElement.assert(
			jsExpressionStatement.assert(
				parseJS({
					path: "unknown",
					input: dedent`
				<div className="class" other={true} onClick={() => { foo()}}/>
			`,
				}).body[0],
			).expression,
		);

		t.true(hasJSXAttribute(jsx, "className"));
		t.true(hasJSXAttribute(jsx, "other"));
		t.false(hasJSXAttribute(jsx, "style"));
		t.true(hasJSXAttribute(jsx, "onClick"));
		t.false(hasJSXAttribute(jsx, "ref"));
	},
);
