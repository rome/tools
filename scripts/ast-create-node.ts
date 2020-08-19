import {
	INTERNAL,
	getLanguageCategories,
	getLanguages,
	languageCategoryExists,
	languageExists,
	reporter,
	writeFile,
} from "./_utils";
import {exists} from "@internal/fs";
import {dedent, toCamelCase} from "@internal/string-utils";
import {markup} from "@internal/markup";
import {main as generateAST} from "./generated-files/ast";

export async function main(
	[language, nodeType, category]: Array<string>,
): Promise<number> {
	if (language === undefined || nodeType === undefined || category === undefined) {
		reporter.error(
			markup`./rome run scripts/ast-create-node [language] [node-type] [category]`,
		);
		return 1;
	}

	if (!nodeType.toLowerCase().startsWith(language)) {
		reporter.error(
			markup`Node type argument "${nodeType}" must have the language prefix "${language}"`,
		);
		return 1;
	}

	if (!(await languageExists(language))) {
		const languages = await getLanguages();

		reporter.error(
			markup`Language argument "${language}" is not a valid language`,
		);

		reporter.info(markup`The following languages are valid:`);

		reporter.list(languages.map((languageName) => markup`${languageName}`));
		return 1;
	}

	if (!(await languageCategoryExists(language, category))) {
		const categories = await getLanguageCategories(language);

		reporter.error(
			markup`Category argument "${category}" is not a valid category for "${language}"`,
		);

		reporter.info(markup`The following categories are valid for ${language}:`);

		reporter.list(categories.map((categoryName) => markup`${categoryName}`));
		return 1;
	}

	const joined = `${language}/${category}/${nodeType}`;

	// This will convert the pascal case to camel
	const builderName = toCamelCase(nodeType);

	// Write AST def
	const astDefPath = INTERNAL.append("ast", `${joined}.ts`);
	if (await exists(astDefPath)) {
		reporter.error(markup`AST node ${joined} already exists`);
		return 1;
	}
	await writeFile(
		astDefPath,
		dedent`
			import {NodeBaseWithComments} from "@internal/ast";
			import {createBuilder} from "../../utils";

			interface ${nodeType} extends NodeBaseWithComments {
				readonly type: "${nodeType}";
			}

			export const ${builderName} = createBuilder<${nodeType}>("${nodeType}", {
				bindingKeys: {},
				visitorKeys: {},
			});
		`,
	);

	// Write builder
	await writeFile(
		INTERNAL.append("formatter", "builders", `${joined}.ts`),
		dedent`
			import {${nodeType}} from "@internal/ast";
			import {Builder, Token} from "@internal/formatter";

			export default function ${nodeType}(builder: Builder, node: ${nodeType}): Token {
				throw new Error("unimplemented");
			}
		`,
	);

	// Write analysis
	if (language === "js") {
		await writeFile(
			INTERNAL.append("js-analysis", "evaluators", category, `${nodeType}.ts`),
			dedent`
				import {AnyNode, ${nodeType}, ${builderName}} from "@internal/ast";

				export default function ${nodeType}(node: AnyNode) {
					node = ${builderName}.assert(node);
					throw new Error("unimplemented");
				}
			`,
		);
	}

	await generateAST();

	return 0;
}
