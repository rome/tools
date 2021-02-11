import {test} from "rome";
import {removeShallowLoc} from "@internal/js-ast-utils/removeShallowLoc";
import {parseJS} from "@internal/js-parser";

test(
	"should remove the location of the node",
	async (t) => {
		const node = parseJS({
			input: "function foo() { return bar; }",
		});

		t.not(node.loc, undefined);
		t.not(node.body[0].loc, undefined);

		t.is(removeShallowLoc(node).loc, undefined);
		t.not(removeShallowLoc(node).body[0].loc, undefined);
	},
);
