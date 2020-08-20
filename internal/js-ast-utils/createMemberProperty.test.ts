import {test} from "rome";
import {createMemberProperty} from "@internal/js-ast-utils/createMemberProperty";
import {
	jsComputedMemberProperty,
	jsIdentifier,
	jsStaticMemberProperty,
	jsStringLiteral,
} from "@internal/ast";

test(
	"verify createMemberProperty return the correct type",
	async (t) => {
		const expectedHello = jsStaticMemberProperty.quick(
			jsIdentifier.quick("hello"),
		);

		t.looksLike(createMemberProperty("hello"), expectedHello);

		const expectedTrue = jsComputedMemberProperty.quick(
			jsStringLiteral.quick("true"),
		);

		t.looksLike(createMemberProperty("true"), expectedTrue);
	},
);
