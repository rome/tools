import {test} from "rome";
import {isBinary} from "./isBinary";
import {parseJS} from "@internal/js-parser";
import {jsExpressionStatement} from "@internal/ast";

function binaryExpressionHelper(input: string): boolean {
	const node = jsExpressionStatement.assert(
		parseJS({
			path: "unknown",
			input,
		}).body[0],
	);

	return isBinary(node.expression);
}

test(
	"returns true for binary expressions",
	(t) => {
		t.true(binaryExpressionHelper("2+2;"));
		t.true(binaryExpressionHelper("variableToReference * 10;"));
		t.true(binaryExpressionHelper("100 / constantDeclared;"));
		t.true(binaryExpressionHelper("100 << 2;"));
		t.true(binaryExpressionHelper("listlike instanceof array;"));
	},
);

test(
	"returns true for logical expressions",
	(t) => {
		t.true(binaryExpressionHelper("true && false;"));
		t.true(binaryExpressionHelper("identifierLookup || true;"));
		t.true(binaryExpressionHelper("nullishValue ?? defaultArgument;"));
	},
);

test(
	"returns false if neither binary nor logical expression",
	(t) => {
		t.false(binaryExpressionHelper("[list, of, values];"));
		t.false(binaryExpressionHelper("-1;"));
		t.false(binaryExpressionHelper("!true;"));
	},
);
