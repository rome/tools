import {test} from "rome";
import {parseJS} from "@internal/js-parser";
import {dedent} from "@internal/string-utils";
import {
	JSXElement,
	jsArrowFunctionExpression,
	jsBooleanLiteral,
	jsExpressionStatement,
	jsObjectExpression,
	jsStringLiteral,
	jsxAttribute,
	jsxExpressionContainer,
} from "@internal/ast";
import {getJSXAttribute} from "@internal/js-ast-utils/getJSXAttribute";

test(
	"verify attribute resolution",
	async (t) => {
		const jsxElement = (jsExpressionStatement.assert(
			parseJS({
				path: "unknown",
				input: dedent`
					<div className="foo" onClick={() => {alert("hello")}} custom={true} other={{one: "first", two: "second"}}/>
				`,
			}).body[0],
		).expression as JSXElement);

		t.is(
			jsStringLiteral.assert(
				jsxAttribute.assert(getJSXAttribute(jsxElement, "className")).value,
			).value,
			"foo",
		);
		t.notThrows(() => {
			jsArrowFunctionExpression.assert(
				jsxExpressionContainer.assert(
					jsxAttribute.assert(getJSXAttribute(jsxElement, "onClick")).value,
				).expression,
			);
		});
		t.is(
			jsBooleanLiteral.assert(
				jsxExpressionContainer.assert(
					jsxAttribute.assert(getJSXAttribute(jsxElement, "custom")).value,
				).expression,
			).value,
			true,
		);
		t.notThrows(() => {
			jsObjectExpression.assert(
				jsxExpressionContainer.assert(
					jsxAttribute.assert(getJSXAttribute(jsxElement, "other")).value,
				).expression,
			);
		});
	},
);
