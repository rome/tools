import {test} from "rome";
import {valueToNode} from "./valueToNode";
import {
	jsArrayExpression,
	jsBigIntLiteral,
	jsBooleanLiteral,
	jsNullLiteral,
	jsNumericLiteral,
	jsObjectExpression,
	jsObjectProperty,
	jsReferenceIdentifier,
	jsStaticPropertyKey,
	jsStringLiteral,
} from "@internal/ast";
import {createPropertyKey} from "./createPropertyKey";

test(
	"verify valueToNode returns the correct supported primitive type",
	(t) => {
		t.looksLike(valueToNode("string"), jsStringLiteral.quick("string"));
		t.looksLike(valueToNode(true), jsBooleanLiteral.quick(true));
		t.looksLike(valueToNode(123), jsNumericLiteral.quick(123));
		t.looksLike(
			valueToNode(BigInt(10)),
			jsBigIntLiteral.quick("10"),
		);
		t.looksLike(
			valueToNode(undefined),
			jsReferenceIdentifier.quick("undefined"),
		);
		t.looksLike(valueToNode(null), jsNullLiteral.create({}));
	},
);

test(
	"verify valueToNode returns the correct structural type",
	(t) => {
		t.looksLike(valueToNode([1]), jsArrayExpression.quick([valueToNode(1)]));
		t.looksLike(
			valueToNode({
				key: "value",
			}),
			jsObjectExpression.quick([
				jsObjectProperty.create({
					key: jsStaticPropertyKey.create({
						value: createPropertyKey("key"),
					}),
					value: valueToNode("value"),
				}),
			]),
		);
	},
);

test(
	"fail with error when do not know how to turn this value into a literal",
	(t) => {
		t.throws(
			() => {
				valueToNode(Symbol("Sym"));
			},
			"Do not know how to turn this value into a literal",
		);
		t.throws(
			() => {
				valueToNode(() => {});
			},
			"Do not know how to turn this value into a literal",
		);
	},
);

test(
	"fail with error when recursion detected",
	(t) => {
		t.throws(
			() => {
				valueToNode(1, [1, 2]);
			},
			"Recursion detected",
		);
	},
);
