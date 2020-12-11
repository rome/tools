import {test} from "rome";
import {parseJS} from "@internal/js-parser";
import {
	jsExpressionStatement,
	jsFunctionDeclaration,
	jsIfStatement,
	jsReturnStatement,
	jsVariableDeclarationStatement,
} from "@internal/ast";
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
		const conditionalExpression = jsExpressionStatement.assert(
			createNode("foo ? foo.bar : undefined"),
		).expression;
		t.true(isConditional(conditionalExpression));
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
		t.true(isConditionalHelper("x || y && z"));
	},
);

test(
	"returns false for non-conditionals",
	(t) => {
		const functionDeclaration = jsFunctionDeclaration.assert(
			createNode("function x() {}"),
		);
		t.false(isConditional(functionDeclaration));

		const variableStatement = jsVariableDeclarationStatement.assert(
			createNode("const y = 1;"),
		);
		t.false(isConditional(variableStatement));

		const returnStatement = jsReturnStatement.assert(createNode("return z;"));
		t.false(isConditional(returnStatement));
	},
);
