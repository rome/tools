import {test} from "rome";
import {parseJS} from "@internal/js-parser";
import {parseCSS} from "@internal/css-parser";
import {dedent} from "@internal/string-utils";
import {removeLoc} from "@internal/ast-utils/removeLoc";
import {AnyNode} from "@internal/ast";

// Needed to access non existing property body
// rome-ignore lint/ts/noExplicitAny
const jsNode: any = removeLoc(
	parseJS({
		path: "unknown",
		input: "function foo() { return bar; }",
	}),
);

// rome-ignore lint/ts/noExplicitAny
const cssNode: any = removeLoc(
	parseCSS({
		path: "unknown",
		input: dedent`
		div {
			background: red;
		}
		`,
	}),
);

test(
	"nodes and their children should no longer have locations",
	async (t) => {
		t.is(jsNode.loc, undefined);

		jsNode.body.map((child: AnyNode) => {
			t.is(child.loc, undefined);
		});

		t.is(cssNode.loc, undefined);

		cssNode.body.map((child: AnyNode) => {
			t.is(child.loc, undefined);
		});
	},
);
