import {test} from "rome";
import {parseJS} from "@internal/js-parser";
import {LineComparison, compareLines} from "./compareLines";
import {jsStringLiteral} from "@internal/ast";

test(
	"different lines",
	async (t) => {
		const jsNode = parseJS({
			input: "var a;\nvar b;",
		});

		let nodeA = jsNode.body[0];
		let nodeB = jsNode.body[1];

		t.is(compareLines(nodeA, nodeB), LineComparison.After);
		t.is(compareLines(nodeB, nodeA), LineComparison.Before);
	},
);

test(
	"same line",
	async (t) => {
		const jsNode = parseJS({
			input: "var a; var b;",
		});

		let nodeA = jsNode.body[0];
		let nodeB = jsNode.body[1];

		t.is(compareLines(nodeA, nodeB), LineComparison.Same);
	},
);

test(
	"unknown lines",
	async (t) => {
		let nodeA = jsStringLiteral.quick("a");
		let nodeB = jsStringLiteral.quick("b");

		t.is(compareLines(nodeA, nodeB), LineComparison.Unknown);
	},
);
