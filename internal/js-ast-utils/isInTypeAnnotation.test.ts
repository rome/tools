import {test} from "rome";
import {isInTypeAnnotation} from "@internal/js-ast-utils/isInTypeAnnotation";
import {
	AnyNode,
	MOCK_PARENT,
	jsArrayExpression,
	jsCommentLine,
	jsRoot,
	tsArrayType,
	tsAsExpression,
	tsNonNullExpression,
	tsNullKeywordTypeAnnotation,
	tsThisType,
	tsTypeAssertion,
} from "@internal/ast";
import {CompilerContext, Path} from "@internal/compiler";
import { createUnknownPath } from "@internal/path";

function helper(node: AnyNode) {
	let path = new Path(
		MOCK_PARENT,
		new CompilerContext({
			ast: jsRoot.create({
				body: [],
				comments: [],
				corrupt: false,
				diagnostics: [],
				directives: [],
				path: createUnknownPath("unknown"),
				hasHoistedVars: false,
				interpreter: undefined,
				sourceType: "script",
				syntax: [],
				integrity: undefined,
			}),
		}),
		{},
	);

	path.parent = node;

	return isInTypeAnnotation(path);
}

test(
	"returns true if the node is in type annotation",
	async (t) => {
		t.false(helper(jsCommentLine.create({id: "", value: "hello"})));

		t.false(
			helper(
				tsAsExpression.create({
					expression: jsArrayExpression.quick([]),
					typeAnnotation: tsThisType.create({}),
				}),
			),
		);

		t.false(
			helper(
				tsTypeAssertion.create({
					expression: jsArrayExpression.quick([]),
					typeAnnotation: tsThisType.create({}),
				}),
			),
		);

		t.false(
			helper(
				tsNonNullExpression.create({expression: jsArrayExpression.quick([])}),
			),
		);

		t.true(helper(tsArrayType.create({elementType: tsThisType.create({})})));

		t.true(helper(tsNullKeywordTypeAnnotation.create({})));
	},
);
