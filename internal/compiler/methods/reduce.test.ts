import {TestHelper, test} from "rome";
import {parseJS} from "@internal/js-parser";
import {removeLoc} from "@internal/ast-utils";
import {AnyVisitor} from "../types";
import {CompilerContext, signals} from "@internal/compiler";
import {formatAST} from "@internal/formatter";

function createReduceTest(
	input: string,
	visitor: Omit<AnyVisitor, "name">,
): (t: TestHelper) => void {
	return (t) => {
		const oldAst = parseJS({input, path: "unknown"});
		const context = new CompilerContext({
			ast: oldAst,
		});
		const newAst = context.reduceRoot([
			{
				...visitor,
				name: "testVisitor",
			},
		]);
		t.namedSnapshot("ast", removeLoc(newAst));
		t.namedSnapshot("formatted", formatAST(newAst));
	};
}

test(
	"remove signal (element)",
	createReduceTest(
		"if (foo) {bar;}",
		{
			enter(path) {
				const {node} = path;

				if (node.type === "JSReferenceIdentifier" && node.name === "bar") {
					return signals.remove;
				}

				return signals.retain;
			},
		},
	),
);

test(
	"remove signal (property)",
	createReduceTest(
		"for (;bar;) {}",
		{
			enter(path) {
				const {node} = path;

				if (node.type === "JSReferenceIdentifier" && node.name === "bar") {
					return signals.remove;
				}

				return signals.retain;
			},
		},
	),
);

test(
	"parent replace signal",
	createReduceTest(
		"if (foo) { for (;bar;) {} }",
		{
			enter(path) {
				const {node, parent} = path;

				if (node.type === "JSReferenceIdentifier" && node.name === "bar") {
					return signals.parent(parent, signals.remove);
				}

				return signals.retain;
			},
		},
	),
);

// Replace a parent but also modify another subtree of the parent
test(
	"parent replace signal complicated",
	createReduceTest(
		"if (condition) { yes; }",
		{
			enter(path) {
				const {node, parent} = path;

				if (
					node.type === "JSReferenceIdentifier" &&
					node.name === "condition" &&
					parent.type === "JSIfStatement"
				) {
					return signals.parent(
						parent,
						signals.replace({
							type: "JSForStatement",
							init: node,
							body: parent.consequent,
						}),
					);
				}

				if (node.type === "JSReferenceIdentifier" && node.name === "yes") {
					return signals.replace({
						...node,
						name: "no",
					});
				}

				return signals.retain;
			},
		},
	),
);
