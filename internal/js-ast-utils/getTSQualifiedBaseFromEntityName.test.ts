import {test} from "rome";
import {parseJS} from "../js-parser";
import {dedent} from "../string-utils";
import {getTSQualifiedBaseFromEntityName} from "./getTSQualifiedBaseFromEntityName";
import {
	jsExpressionStatement,
	jsReferenceIdentifier,
	tsImportEqualsDeclaration,
	tsQualifiedName,
} from "../ast";

test(
	"verify reference returned by getTSQualifiedBaseFromEntityName",
	async (t) => {
		const js = parseJS({
			input: dedent`
				import A = B.C;
				E;
			`,
		});

		t.is(
			getTSQualifiedBaseFromEntityName(
				tsQualifiedName.assert(
					tsImportEqualsDeclaration.assert(js.body[0]).moduleReference,
				),
			).name,
			"B",
		);
		t.is(
			getTSQualifiedBaseFromEntityName(
				jsReferenceIdentifier.assert(
					jsExpressionStatement.assert(js.body[1]).expression,
				),
			).name,
			"E",
		);
	},
);
