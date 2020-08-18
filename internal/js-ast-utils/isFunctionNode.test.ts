import {test} from "rome";
import {isFunctionNode} from "./isFunctionNode";
import {template} from "./template";
import {createBuilder} from "@internal/ast/utils";
import {AnyNode, jsVariableDeclarationStatement} from "@internal/ast";

function helper(input: AnyNode): AnyNode {
	return (jsVariableDeclarationStatement.assert(input).declaration.declarations[0].init as AnyNode);
}

test(
	"returns true for all function node types",
	async (t) => {
		t.true(isFunctionNode(template.statement`function foo(){}`));
		t.true(
			isFunctionNode(helper(template.statement`const foo = function () { }`)),
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
		t.true(isFunctionNode(helper(template.statement`const foo = () => {}`)));
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
