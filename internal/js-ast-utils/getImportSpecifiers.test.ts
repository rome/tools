import {test} from "rome";
import {parseJS} from "@internal/js-parser";
import {dedent} from "@internal/string-utils";
import {getImportSpecifiers} from "@internal/js-ast-utils/getImportSpecifiers";
import {
	jsImportDeclaration,
	jsImportDefaultSpecifier,
	jsImportSpecifier,
} from "@internal/ast";

test(
	"verify import specifiers",
	async (t) => {
		const imports = parseJS({
			path: "unknown",
			input: dedent`
				import {foo} from "bar";
				import Hello from "world";
				import {one, two, three} from "words/numbers";
			`,
		}).body;

		const jsImport1 = getImportSpecifiers(
			jsImportDeclaration.assert(imports[0]),
		);
		const jsImport2 = getImportSpecifiers(
			jsImportDeclaration.assert(imports[1]),
		);
		const jsImport3 = getImportSpecifiers(
			jsImportDeclaration.assert(imports[2]),
		);

		t.is(jsImportSpecifier.assert(jsImport1[0]).local.name.name, "foo");

		t.is(jsImportDefaultSpecifier.assert(jsImport2[0]).local.name.name, "Hello");

		t.is(jsImportSpecifier.assert(jsImport3[0]).local.name.name, "one");
		t.is(jsImportSpecifier.assert(jsImport3[1]).local.name.name, "two");
		t.is(jsImportSpecifier.assert(jsImport3[2]).local.name.name, "three");
	},
);
