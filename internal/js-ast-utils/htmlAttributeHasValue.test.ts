import {test} from "rome";
import {htmlElement} from "@internal/ast";
import {parseHTML} from "@internal/html-parser";
import {dedent} from "@internal/string-utils";
import htmlAttributeHasValue from "@internal/js-ast-utils/htmlAttributeHasValue";

test(
	"verify hasHTMLAttribute returns correct values",
	(t) => {
		const element = htmlElement.assert(
			parseHTML({
				path: "unknown",
				input: dedent`
					<input type="image" name="foo" disabled  />
				`,
			}).body[0],
		);

		t.true(htmlAttributeHasValue(element, "type"));
		t.true(htmlAttributeHasValue(element, "name"));
		t.false(htmlAttributeHasValue(element, "disabled"));
		t.false(htmlAttributeHasValue(element, "id"));
	},
);
