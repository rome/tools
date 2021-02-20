import {test} from "rome";
import {hasPotentialSideEffects} from "@internal/js-ast-utils/hasPotentialSideEffects";
import Scope, {RootScope} from "@internal/compiler/scope/Scope";
import {
	AnyNode,
	MOCK_JS_ROOT,
	jsArrayExpression,
	jsAssignmentIdentifier,
	jsBindingIdentifier,
	jsBlockStatement,
	jsClassDeclaration,
	jsClassHead,
	jsExportExternalDeclaration,
	jsExportLocalDeclaration,
	jsFunctionDeclaration,
	jsFunctionExpression,
	jsFunctionHead,
	jsNullLiteral,
	jsNumericLiteral,
	jsReferenceIdentifier,
	jsSpreadProperty,
	jsStaticPropertyKey,
	jsStringLiteral,
	jsVariableDeclaration,
	jsVariableDeclarator,
} from "@internal/ast";
import {CompilerContext} from "@internal/compiler";

function helper(node: AnyNode): boolean {
	return hasPotentialSideEffects(
		node,
		new Scope({
			kind: "root",
			node: undefined,
			parentScope: undefined,
			rootScope: undefined,
		}),
	);
}

test(
	"returns true if the js statement has a potential side effect",
	async (t) => {
		t.false(helper(jsExportLocalDeclaration.create({}, undefined)));

		t.true(
			helper(
				jsExportExternalDeclaration.create(
					{namedSpecifiers: [], source: jsStringLiteral.quick("foo")},
					undefined,
				),
			),
		);

		t.false(
			helper(
				jsFunctionExpression.create(
					{body: jsBlockStatement.quick([]), head: jsFunctionHead.quick([])},
					undefined,
				),
			),
		);

		t.false(
			helper(
				jsFunctionDeclaration.create({
					body: jsBlockStatement.quick([]),
					head: jsFunctionHead.quick([]),
					id: jsBindingIdentifier.quick("bar"),
				}),
			),
		);

		t.true(
			helper(
				jsClassDeclaration.create({
					id: jsBindingIdentifier.quick("hello"),
					meta: jsClassHead.quick([]),
				}),
			),
		);

		let scope = new Scope({
			kind: "root",
			node: undefined,
			parentScope: undefined,
			rootScope: new RootScope(
				new CompilerContext({
					ast: MOCK_JS_ROOT,
				}),
				MOCK_JS_ROOT,
			),
		});

		t.false(hasPotentialSideEffects(jsReferenceIdentifier.quick("foo"), scope));

		scope.getRootScope().addGlobal("foo");

		t.true(hasPotentialSideEffects(jsReferenceIdentifier.quick("foo"), scope));

		t.true(
			helper(
				jsVariableDeclaration.create({
					declarations: [
						jsVariableDeclarator.create({id: jsBindingIdentifier.quick("bar")}),
					],
					kind: "const",
				}),
			),
		);

		t.false(
			helper(
				jsSpreadProperty.create({argument: jsStringLiteral.quick("hello")}),
			),
		);

		t.false(
			helper(
				jsStaticPropertyKey.create({value: jsStringLiteral.quick("world")}),
			),
		);

		t.false(
			helper(
				jsArrayExpression.quick([
					jsStringLiteral.quick("foo"),
					jsNumericLiteral.quick(4),
					jsNullLiteral.create({}),
				]),
			),
		);

		t.true(helper(jsAssignmentIdentifier.quick("bar")));
	},
);
