import {test} from "rome";
import {isFor} from "./isFor";
import {parseJS} from "@internal/js-parser";
import {
	jsExpressionStatement,
	jsForInStatement,
	jsForOfStatement,
	jsForStatement,
} from "@internal/ast";

function forStatementHelper(input: string): boolean {
	return isFor(
		jsForStatement.assert(
			parseJS({
				path: "unknown",
				input,
			}).body[0],
		),
	);
}

function forOfStatementHelper(input: string): boolean {
	return isFor(
		jsForOfStatement.assert(
			parseJS({
				path: "unknown",
				input,
			}).body[0],
		),
	);
}

function forInStatementHelper(input: string): boolean {
	return isFor(
		jsForInStatement.assert(
			parseJS({
				path: "unknown",
				input,
			}).body[0],
		),
	);
}

function jsExpressionStatementHelper(input: string): boolean {
	return isFor(
		jsExpressionStatement.assert(
			parseJS({
				path: "unknown",
				input,
			}).body[0],
		),
	);
}

test(
	"isFor returns true when JSForStatement",
	(t) => {
		t.true(forStatementHelper("for (let i = 0; i < 1; i++) {};"));
	},
);

test(
	"isFor returns true when JSForInStatement",
	(t) => {
		t.true(forInStatementHelper("for (let key in mapLike) {};"));
	},
);

test(
	"isFor returns true when JSForOfStatement",
	(t) => {
		t.true(forOfStatementHelper("for (let key of arrayLike) {};"));
	},
);

test(
	"isFor returns false when JSExpressionStatement",
	(t) => {
		t.false(jsExpressionStatementHelper("2 + 2 == 4;"));
	},
);
