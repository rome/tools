import {test} from "rome";
import {isInTypeAnnotation} from "@internal/js-ast-utils/isInTypeAnnotation";
import {
	AnyNode,
	MOCK_JS_ROOT,
	MOCK_PARENT,
	jsArrayExpression,
	jsCommentLine,
	tsArrayType,
	tsAsExpression,
	tsNonNullExpression,
	tsNullKeywordTypeAnnotation,
	tsThisType,
	tsTypeAssertion,
} from "@internal/ast";
import {CompilerContext, CompilerPath} from "@internal/compiler";

function helper(node: AnyNode) {
	let path = new CompilerPath(
		MOCK_PARENT,
		new CompilerContext({
			ast: MOCK_JS_ROOT,
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
