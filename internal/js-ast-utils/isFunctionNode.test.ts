import {test} from "rome";
import {isFunctionNode} from "./isFunctionNode";
import {template} from "./template";
import {jsClassDeclaration, jsObjectExpression} from "@internal/ast";

test(
	"returns true for all function node types",
	(t) => {
		t.true(isFunctionNode(template.statement`function foo(){}`));
		t.true(isFunctionNode(template.expression`(()=>{})`));
		t.true(
			isFunctionNode(
				jsObjectExpression.assert(template.expression`({foo() {}})`).properties[0],
			),
		);
		t.true(isFunctionNode(template.expression`(function() {})`));
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
