import {test} from "rome";
import {template} from "./template";
import {isConditional} from "./isConditional";

test(
	"returns true for if-statements",
	(t) => {
		t.true(isConditional(template.statement`if(x){}`));
	},
);

test(
	"returns true for conditional expressions",
	(t) => {
		t.true(isConditional(template.expression`x ? 1 : 2`));
	},
);

test(
	"returns true for logical expressions",
	(t) => {
		t.true(isConditional(template.expression`x && y`));
		t.true(isConditional(template.expression`x || y`));
		t.true(isConditional(template.expression`x || y && z`));
	},
);

test(
	"returns false for non-conditionals",
	(t) => {
		t.false(isConditional(template.statement`function x(){}`));
		t.false(isConditional(template.statement`const y = 1`));
		t.false(isConditional(template.statement`return z`));
	},
);
