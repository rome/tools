import {test} from "rome";
import {AnyNode, jsExpressionStatement} from "@internal/ast";
import {parseJS} from "@internal/js-parser";
import {isJSXElement} from "./isJSXElement";
import {createBuilder} from "@internal/ast/utils";

function helper(input: string): AnyNode {
	const node = jsExpressionStatement.assert(
		parseJS({
			input,
			path: "unknown",
		}).body[0],
	);
	return node.expression;
}

test(
	"returns true for jsx elements",
	(t) => {
		t.true(isJSXElement(helper("<div>a</div>"), "div"));
		t.true(
			isJSXElement(
				createBuilder(
					"JSXElement",
					{
						bindingKeys: {},
						visitorKeys: {
							name: true,
							typeArguments: true,
							attributes: true,
							children: true,
						},
					},
				).create(helper("<ul></ul>")),
				"ul",
			),
		);
		t.true(isJSXElement(helper("<ul>text</ul>"), "ul"));
		t.true(isJSXElement(helper(`<span id="key">{var}</span>`), "span"));
		t.true(isJSXElement(helper("<CustomComp></CustomComp>"), "CustomComp"));
		t.true(isJSXElement(helper("<component>"), "component"));
	},
);

test(
	"returns false for non-jsx elements",
	(t) => {
		t.false(isJSXElement(helper("2+3"), "div"));
		t.false(isJSXElement(helper("someValue / 4"), "ul"));
		t.false(isJSXElement(helper("2;"), "li"));
		t.false(isJSXElement(helper("true"), "span"));
		t.false(
			isJSXElement(
				createBuilder(
					"jsIfStatement",
					{
						bindingKeys: {},
						visitorKeys: {test: true, consequent: true, alternate: true},
					},
				).create({}),
				"li",
			),
		);
	},
);

test(
	"returns false when there is name mismatch",
	(t) => {
		t.false(isJSXElement(helper("<div></div>"), "ul"));
		t.false(isJSXElement(helper("<span></span>"), "img"));
		t.false(isJSXElement(helper("<test>"), "otherTest"));
	},
);
