import {test} from "rome";
import {isFunctionNode} from "./isFunctionNode";
import {template} from "./template";
import {createBuilder} from "@internal/ast/utils";
import {
	AnyNode,
	jsClassDeclaration,
	jsVariableDeclarationStatement,
} from "@internal/ast";

test(
	"returns true for all function node types",
	async (t) => {
		t.true(isFunctionNode(template.statement`function foo(){}`));
		t.true(
			isFunctionNode(
				(jsVariableDeclarationStatement.assert(
					template.statement`const foo = function () { }`,
				).declaration.declarations[0].init as AnyNode),
			),
		);
		t.true(
			isFunctionNode(
				createBuilder(
					"JSObjectMethod",
					{
						bindingKeys: {},
						visitorKeys: {
							key: true,
							head: true,
							body: true,
						},
					},
				).create({}),
			),
		);
		t.true(
			isFunctionNode(
				(jsVariableDeclarationStatement.assert(
					template.statement`const foo = () => {}`,
				).declaration.declarations[0].init as AnyNode),
			),
		);
		t.snapshot(
			jsClassDeclaration.assert(template.statement`class foo(){ bar(){};}`),
		);
		t.true(
			isFunctionNode(
				jsClassDeclaration.assert(template.statement`class foo(){ bar(){};}`).meta.body[0],
			),
		);
	},
);

test(
	"returns false for all nodes except function node types",
	(t) => {
		t.false(isFunctionNode(template.statement`if(a>2){}`));
		t.false(isFunctionNode(template.expression`2+someValue`));
		t.false(isFunctionNode(template.statement`while(1){}`));
		t.false(isFunctionNode(template.statement`for(let i0;i<5;i++){}`));
	},
);
