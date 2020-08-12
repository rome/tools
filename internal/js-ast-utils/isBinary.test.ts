import {test} from "rome";
import {isBinary} from "./isBinary";
import {parseJS} from "@internal/js-parser";
import {jsExpressionStatement} from "@internal/ast";

function binaryExpressionHelper(input: string) {
	const node = jsExpressionStatement.assert(
		parseJS({
			path: "unknown",
			input,
		}).body[0],
	);

	return isBinary(node.expression);
}

test(
	"isBinary",
	(t) => {
		t.true(binaryExpressionHelper("2+2;"));
		t.true(binaryExpressionHelper("variableToReference * 10;"));
		t.true(binaryExpressionHelper("100 / constantDeclared;"));
		t.true(binaryExpressionHelper("100 << 2;"));
		t.true(binaryExpressionHelper("listlike instanceof array;"));

		t.true(binaryExpressionHelper("true && false;"));
		t.true(binaryExpressionHelper("identifierLookup || true;"));
		t.true(binaryExpressionHelper("nullishValue ?? defaultArgument;"));

		t.false(binaryExpressionHelper("[list, of, values];"));
		t.false(binaryExpressionHelper("-1;"));
		t.false(binaryExpressionHelper("!true;"));
	},
);
