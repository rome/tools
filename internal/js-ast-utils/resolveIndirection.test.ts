import {test} from "rome";
import {
	jsReferenceIdentifier,
	jsStringLiteral,
	tsAsExpression,
	tsStringLiteralTypeAnnotation,
} from "@internal/ast";
import {resolveIndirection} from "@internal/js-ast-utils/resolveIndirection";
import {ConstBinding, Scope} from "@internal/compiler";

test(
	"resolve indirection",
	(t) => {
		const scope = new Scope({
			kind: "root",
			node: undefined,
			parentScope: undefined,
			rootScope: undefined,
		});

		const js = jsStringLiteral.quick("hello");

		t.is(js, resolveIndirection(js, scope).node);

		const js1 = tsAsExpression.create({
			expression: js,
			typeAnnotation: tsStringLiteralTypeAnnotation.create({
				value: "",
			}),
		});

		t.is(js, resolveIndirection(js1, scope).node);

		const js2 = jsReferenceIdentifier.quick("bar");

		const biding = new ConstBinding(
			{
				name: "bar",
				node: js,
				scope,
			},
			js,
		);

		t.is(js2, resolveIndirection(js2, scope).node);

		scope.addBinding(biding);

		t.is(js, resolveIndirection(js2, scope).node);
	},
);
