import {test} from "rome";
import {parseJS} from "@internal/js-parser";
import CompilerContext from "../lib/CompilerContext";
import Scope from "./Scope";
import {dedent} from "@internal/string-utils";
import {signals} from "..";

type ChildScopeMap = Map<Scope, Set<Scope>>;

function indent(str: string): string {
	return `	${str.split("\n").join("\n\t")}`;
}

function dumpScope(
	scope: Scope,
	childScopeMap: ChildScopeMap,
	input?: string,
): string {
	const lines = [];

	if (input) {
		lines.push(`# Input \n ${input} \n`);
	}
	lines.push(scope.dump());

	const childScopeNodes = childScopeMap.get(scope);
	if (childScopeNodes !== undefined) {
		lines.push("## Child Scopes");
		for (const scope of childScopeNodes) {
			lines.push(indent(dumpScope(scope, childScopeMap)));
		}
	}

	return lines.join("\n");
}

function dumpScopeTree(input: string): string {
	const ast = parseJS({input, path: "unknown"});
	const context = new CompilerContext({ast});

	// Collect a map of scope to child scopes
	const scopeToChildScopes: ChildScopeMap = new Map();
	context.reduceRoot(
		ast,
		{
			name: "test",
			enter({node, scope}) {
				// Enter a new scope
				if (scope.node === node) {
					const {parentScope} = scope;
					if (parentScope !== undefined) {
						let childScopeNodes = scopeToChildScopes.get(parentScope);
						if (childScopeNodes === undefined) {
							childScopeNodes = new Set();
							scopeToChildScopes.set(parentScope, childScopeNodes);
						}
						childScopeNodes.add(scope);
					}
				}

				return signals.retain;
			},
		},
	);

	return dumpScope(context.rootScope, scopeToChildScopes, input);
}

test(
	"var",
	(t) => {
		// Attached to Program
		t.snapshot(
			dumpScopeTree(
				dedent`
		var foo;
	`,
			),
		);

		// Attached to BlockStatement
		t.snapshot(
			dumpScopeTree(
				dedent`
		function foo() {
			var bar;

			{
				var car;
			}
		}
	`,
			),
		);
	},
);
