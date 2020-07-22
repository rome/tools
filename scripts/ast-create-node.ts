import {PACKAGES, reporter, writeFile} from "./_utils";
import {exists} from "@romefrontend/fs";
import {dedent, toCamelCase} from "@romefrontend/string-utils";
import {markup} from "@romefrontend/cli-layout";
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

	const joined = `${language}/${category}/${nodeType}`;

	// This will convert the pascal case to camel
	const builderName = toCamelCase(nodeType);

	// Write AST def
	const astDefPath = PACKAGES.appendList("ast", `${joined}.ts`);
	if (await exists(astDefPath)) {
		reporter.error(markup`AST node ${joined} already exists`);
		return 1;
	}
	await writeFile(
		astDefPath,
		dedent`
			import {NodeBaseWithComments} from "@romefrontend/ast";
			import {createBuilder} from "../../utils";

			export type ${nodeType} = NodeBaseWithComments & {
				type: "${nodeType}";
			};

			export const ${builderName} = createBuilder<${nodeType}>("${nodeType}", {
				bindingKeys: {},
				visitorKeys: {},
			});
		`,
	);

	// Write builder
	await writeFile(
		PACKAGES.appendList("formatter", "builders", `${joined}.ts`),
		dedent`
			import {${nodeType}} from "@romefrontend/ast";
			import {Builder, Token} from "@romefrontend/formatter";

			export default function ${nodeType}(builder: Builder, node: ${nodeType}): Token {
				throw new Error("unimplemented");
			}
		`,
	);

	// Write analysis
	if (language === "js") {
		await writeFile(
			PACKAGES.appendList(
				"js-analysis",
				"evaluators",
				category,
				`${nodeType}.ts`,
			),
			dedent`
				import {AnyNode, ${nodeType}, ${builderName}} from "@romefrontend/ast";

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
