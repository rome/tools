import {test} from "rome";
import {parseJS} from "@internal/js-parser";
import {parseCSS} from "@internal/css-parser";
import {dedent} from "@internal/string-utils";
import {removeLoc} from "@internal/ast-utils/removeLoc";

// Needed to access non existing property loc
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
		t.true(jsNode.loc == null);

		// rome-ignore lint/ts/noExplicitAny
		jsNode.body.map((child: any) => {
			t.true(child.loc == null);
		});

		t.true(cssNode.loc == null);

		// rome-ignore lint/ts/noExplicitAny
		cssNode.body.map((child: any) => {
			t.true(child.loc == null);
		});
	},
);
