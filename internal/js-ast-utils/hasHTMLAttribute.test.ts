import {test} from "rome";
import {htmlElement} from "@internal/ast";
import {parseHTML} from "@internal/html-parser";
import {dedent} from "@internal/string-utils";
import hasHTMLAttribute from "@internal/js-ast-utils/hasHTMLAttribute";

test(
	"verify hasHTMLAttribute returns correct values",
	(t) => {
		const element = htmlElement.assert(
			parseHTML({
				input: dedent`
					<input type="image" name="foo" disabled  />
				`,
			}).body[0],
		);

		t.true(hasHTMLAttribute(element, "type"));
		t.true(hasHTMLAttribute(element, "name"));
		t.true(hasHTMLAttribute(element, "disabled"));
		t.false(hasHTMLAttribute(element, "id"));
	},
);
