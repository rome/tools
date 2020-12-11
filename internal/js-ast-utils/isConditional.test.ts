import {test} from "rome";
import {parseJS} from "@internal/js-parser";
import {jsExpressionStatement, jsIfStatement} from "@internal/ast";
import {isConditional} from "./isConditional";

function createNode(input: string) {
	return parseJS({
		path: "unknown",
		input,
	}).body[0];
}

test(
	"returns true for if-statements",
	(t) => {
		function isConditionalHelper(input: string) {
			const node = jsIfStatement.assert(createNode(input));
			return isConditional(node);
		}

		t.true(isConditionalHelper("if (x) { return 1; }"));
		t.true(isConditionalHelper("if (x) { return 1; } else { return 2; }"));
		t.true(
			isConditionalHelper(
				"if (x) { return 1; } else if (y) { return 2; } else { return 3; }",
			),
		);
	},
);

test(
	"returns true for conditional expressions",
	(t) => {
		function isConditionalHelper(input: string) {
			const node = jsExpressionStatement.assert(createNode(input));
			return isConditional(node.expression);
		}

		t.true(isConditionalHelper("foo ? foo.bar : undefined"));
	},
);

test(
	"returns true for logical expressions",
	(t) => {
		function isConditionalHelper(input: string) {
			const node = jsExpressionStatement.assert(createNode(input));
			return isConditional(node.expression);
		}

		t.true(isConditionalHelper("x && y"));
		t.true(isConditionalHelper("x || y"));
	},
);
