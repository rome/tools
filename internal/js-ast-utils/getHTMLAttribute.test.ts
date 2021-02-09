import {test} from "rome";
import {parseHTML} from "@internal/html-parser";
import {dedent} from "@internal/string-utils";
import {
	htmlAttribute,
	htmlElement,
	htmlIdentifier,
	htmlString,
} from "@internal/ast";
import getHTMLAttribute from "@internal/js-ast-utils/getHTMLAttribute";

test(
	"retrieve the correct attribute",
	(t) => {
		const element = htmlElement.assert(
			parseHTML({
				path: "unknown",
				input: dedent`
					<input type="image" name="foo" title=""  />
				`,
			}).body[0],
		);

		const type = htmlAttribute.assert(getHTMLAttribute(element, "type"));
		const name = htmlAttribute.assert(getHTMLAttribute(element, "name"));

		t.is(htmlIdentifier.assert(type.name).name, "type");

		t.is(htmlString.assert(type.value).value, "image");

		t.is(htmlIdentifier.assert(name.name).name, "name");

		t.is(htmlString.assert(name.value).value, "foo");

		t.is(getHTMLAttribute(element, "title"), undefined);
	},
);
