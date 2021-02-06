import {test} from "rome";
import {renameBindings} from "@internal/js-ast-utils/renameBindings";
import {CompilerContext, Path} from "@internal/compiler";
import {
	MOCK_PROGRAM,
	jsAssignmentExpression,
	jsExpressionStatement,
	jsIdentifier,
	jsMemberExpression,
	jsNullLiteral,
	jsReferenceIdentifier,
	jsStaticMemberProperty,
} from "@internal/ast";
import {createDefaultProjectConfig} from "@internal/project";

test(
	"should rename biding",
	async (t) => {
		const js = jsExpressionStatement.create({
			expression: jsAssignmentExpression.create({
				operator: "=",
				left: jsMemberExpression.create({
					object: jsReferenceIdentifier.quick("foo"),
					property: jsStaticMemberProperty.quick(jsIdentifier.quick("bar")),
				}),
				right: jsNullLiteral.create({}),
			}),
		});

		t.is(
			jsReferenceIdentifier.assert(
				jsMemberExpression.assert(
					jsAssignmentExpression.assert(js.expression).left,
				).object,
			).name,
			"foo",
		);

		const map = new Map();
		map.set("foo", "hello");

		const context = new CompilerContext({
			ast: MOCK_PROGRAM,
			project: {
				configCacheKeys: [],
				directory: undefined,
				config: createDefaultProjectConfig(),
			},
		});

		const path = new Path(js, context, {});

		t.is(
			jsReferenceIdentifier.assert(
				jsMemberExpression.assert(
					jsAssignmentExpression.assert(
						jsExpressionStatement.assert(renameBindings(path, map)).expression,
					).left,
				).object,
			).name,
			"hello",
		);
	},
);
