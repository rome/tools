import {test} from "rome";
import {parseJS} from "@internal/js-parser";
import {parseCSS} from "@internal/css-parser";
import {assertRoot} from "./assertRoot";

test(
	"root node should be returned",
	async (t) => {
		const jsNode = parseJS({
			input: "function test() { return true; }",
		});

		t.is(assertRoot(jsNode), jsNode);

		const cssNode = parseCSS({
			input: "div { color: red; }",
		});

		t.is(assertRoot(cssNode), cssNode);
	},
);

test(
	"non-root node should throw error",
	async (t) => {
		const jsNode = parseJS({
			input: "function test() { return true; }",
		});

		t.throws(() => {
			assertRoot(jsNode.body[0]);
		});
	},
);
