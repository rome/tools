/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {isBinary} from "./isBinary";
import {parseJS} from "@internal/js-parser";
import {JSExpressionStatement} from "@internal/ast";

function binaryExpressionHelper(input: string) {
	const node = (parseJS({
		path: "unknown",
		input,
	}).body[0] as JSExpressionStatement);

	return isBinary(node.expression);
}

test(
	"isBinary",
	(t) => {
		t.inlineSnapshot(binaryExpressionHelper("2+2;"), true);
		t.inlineSnapshot(binaryExpressionHelper("variableToReference * 10;"), true);
		t.inlineSnapshot(binaryExpressionHelper("100 / constantDeclared;"), true);
		t.inlineSnapshot(binaryExpressionHelper("100 << 2;"), true);
		t.inlineSnapshot(binaryExpressionHelper("listlike instanceof array;"), true);

		t.inlineSnapshot(binaryExpressionHelper("true && false;"), true);
		t.inlineSnapshot(binaryExpressionHelper("identifierLookup || true;"), true);
		t.inlineSnapshot(binaryExpressionHelper("identifierLookup || true;"), true);
		t.inlineSnapshot(
			binaryExpressionHelper("nullishValue ?? defaultArgument;"),
			true,
		);

		t.inlineSnapshot(binaryExpressionHelper("[list, of, values];"), false);
		t.inlineSnapshot(binaryExpressionHelper("-1;"), false);
		t.inlineSnapshot(binaryExpressionHelper("!true;"), false);
	},
);
