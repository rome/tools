import {test} from "rome";
import {parseJS} from "@internal/js-parser";
import {parseCSS} from "@internal/css-parser";
import {dedent} from "@internal/string-utils";
import {removeLoc} from "@internal/ast-utils/removeLoc";
import {AnyNode} from "@internal/ast";

test(
	"nodes and their children should no longer have locations",
	async (t) => {
		// Needed to access non existing property body
		// rome-ignore lint/ts/noExplicitAny: future cleanup
		const jsNode: any = removeLoc(
			parseJS({
				input: "function foo() { return bar; }",
			}),
		);

		// rome-ignore lint/ts/noExplicitAny: future cleanup
		const cssNode: any = removeLoc(
			parseCSS({
				input: dedent`
					div {
						background: red;
					}
				`,
			}),
		);
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
