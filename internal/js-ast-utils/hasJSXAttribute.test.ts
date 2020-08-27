import {test} from "rome";
import {hasJSXAttribute, template} from "@internal/js-ast-utils";
import {jsxElement} from "@internal/ast";

test(
	"verify hasJSXAttribute returns the correct value",
	async (t) => {
		const jsx = jsxElement.assert(
			template.expression`<div className="class" other={true} onClick={() => { foo()}}/>`,
		);

		t.true(hasJSXAttribute(jsx, "className"));
		t.true(hasJSXAttribute(jsx, "other"));
		t.false(hasJSXAttribute(jsx, "style"));
		t.true(hasJSXAttribute(jsx, "onClick"));
		t.false(hasJSXAttribute(jsx, "ref"));
	},
);
