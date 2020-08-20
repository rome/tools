import {test} from "rome";
import {template} from "./template";
import {jsxElement} from "@internal/ast";
import {getJSXElementName} from "./getJSXElementName";

test(
	"returns the name of the jsx element for 'JSXIdentifier', 'JSXReferenceIdentifier' and 'JSXNamespacedName' jsx node types",
	(t) => {
		t.is(
			getJSXElementName(jsxElement.assert(template.expression`<div>`)),
			"div",
		);
		t.is(
			getJSXElementName(
				jsxElement.assert(template.expression`<Foo attr="value" />`),
			),
			"Foo",
		);
		t.is(
			getJSXElementName(jsxElement.assert(template.expression`<Foo:Bar />`)),
			"Bar",
		);
	},
);

test(
	"returns empty string for other jsx element node types",
	(t) => {
		t.is(
			getJSXElementName(jsxElement.assert(template.expression`<My.Comp />`)),
			"",
		);
	},
);
