import {test} from "rome";
import {parseJS} from "@internal/js-parser";
import CompilerContext from "../lib/CompilerContext";
import Scope from "./Scope";
import {dedent} from "@internal/string-utils";
import {signals} from "..";
import {ExtendedMap} from "@internal/collections";

type ChildScopeMap = ExtendedMap<Scope, Set<Scope>>;

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
	const scopeToChildScopes: ChildScopeMap = new ExtendedMap(
		"scopeToChildScopes",
		() => new Set(),
	);
	context.reduceRoot({
		name: "test",
		enter({node, scope}) {
			// Enter a new scope
			if (scope.node === node) {
				const {parentScope} = scope;
				if (parentScope !== undefined) {
					const childScopeNodes = scopeToChildScopes.assert(parentScope);
					childScopeNodes.add(scope);
				}
			}

			return signals.retain;
		},
	});

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
