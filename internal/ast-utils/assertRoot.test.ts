import {test} from "rome";
import {parseHTML} from "@internal/html-parser";
import {assertRoot} from "@internal/ast-utils/assertRoot";
import {parseJS} from "@internal/js-parser";
import {parseCSS} from "@internal/css-parser";
import {dedent} from "@internal/string-utils";
import {parseCommit} from "@internal/commit-parser";
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

const mdNode = parseMarkdown({
	path: "unknown",
	input: "**foo**_bar_",
});

const htmlNode = parseHTML({
	path: "unknown",
	input: "<div></div>",
});

test(
	"returns the same if it is a root node",
	async (t) => {
		t.is(assertRoot(jsNode), jsNode);
		t.is(assertRoot(cssNode), cssNode);
		t.is(assertRoot(commitNode), commitNode);
		t.is(assertRoot(mdNode), mdNode);
		t.is(assertRoot(htmlNode), htmlNode);
	},
);

test(
	"throws if not a root node",
	async (t) => {
		t.throws(() => {
			assertRoot(jsNode.body[0]);
		});
		t.throws(() => {
			assertRoot(cssNode.body[0]);
		});
		t.throws(() => {
			assertRoot(mdNode.body[0]);
		});
		t.throws(() => {
			assertRoot(mdNode.body[0]);
		});
		t.throws(() => {
			assertRoot(htmlNode.body[0]);
		});
	},
);
