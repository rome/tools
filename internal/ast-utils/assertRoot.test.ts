import {test} from "rome";
import {parseHTML} from "@internal/html-parser";
import {assertRoot} from "@internal/ast-utils/assertRoot";
import {parseJS} from "@internal/js-parser";
import {parseCSS} from "@internal/css-parser";
import {dedent} from "@internal/string-utils";
import {parseCommit} from "@internal/commit-parser";
import {AnyRoot} from "@internal/ast";
import {parseMarkdown} from "@internal/markdown-parser";

const jsNode = parseJS({
	path: "unknown",
	input: "function foo() { return bar; }",
});

const cssNode = parseCSS({
	path: "unknown",
	input: dedent`
	div {
	  background: red;
	}
	`,
});

const commitNode = parseCommit({
	path: "unknown",
	input: "fix: changed foo to bar",
});

const mdRoot = parseMarkdown({
	path: "unknown",
	input: "**foo**_bar_",
});

const htmlNode = parseHTML({
	path: "unknown",
	input: "<div></div>",
});

const nodes = [jsNode, cssNode, commitNode, mdRoot, htmlNode];

test(
	"nodes",
	async (t) => {
		nodes.map((node) => {
			t.snapshot(node);
		});
	},
);

test(
	"returns the same if it is a root node",
	async (t) => {
		nodes.map((node) => {
			t.is(assertRoot(node), node);
		});
	},
);

test(
	"throws if not a root node",
	async (t) => {
		nodes.map((node: AnyRoot) => {
			// CommitRoot doesn't have a body nor other types
			if (node.type !== "CommitRoot") {
				t.throws(() => {
					assertRoot(node.body[0]);
				});
			}
		});
	},
);
