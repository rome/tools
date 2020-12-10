import {test} from "rome";
import {AnyNode} from "@internal/ast";
import {isRoot} from "./isRoot";

function createNode(type: string): AnyNode {
	return ({
		type,
	} as AnyNode);
}

test(
	"valid roots return true",
	async (t) => {
		function createNode(type: string) {
			return ({
				type,
			} as AnyNode);
		}

		const validRoots = [
			"JSRoot",
			"CSSRoot",
			"CommitRoot",
			"MarkdownRoot",
			"HTMLRoot",
		];

		validRoots.forEach((r) => {
			t.is(isRoot(createNode(r)), true);
		});
	},
);

test(
	"invalid roots return false",
	async (t) => {
		const invalidRoots = [
			"CommentBlock",
			"CommentLine",
			"JSUpdateExpression",
			"TSTypeQuery",
			"TSUnionTypeAnnotation",
		];

		invalidRoots.forEach((r) => {
			t.is(isRoot(createNode(r)), false);
		});
	},
);
