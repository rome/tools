import {test} from "rome";
import {jsIdentifier, jsStringLiteral} from "@internal/ast";
import {createPropertyKey} from "@internal/js-ast-utils/createPropertyKey";

test(
	"verify createPropertyKey return the correct type",
	async (t) => {
		const expectedWorld = jsIdentifier.quick("world");

		t.looksLike(createPropertyKey("world"), expectedWorld);

		const expectedFalse = jsStringLiteral.quick("false");

		t.looksLike(createPropertyKey("false"), expectedFalse);
	},
);
