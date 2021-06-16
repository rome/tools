import {INTERNAL, ROOT, reporter, writeFile} from "./_utils";
import {dedent, toCamelCase} from "@internal/string-utils";
import {main as generateLintRules} from "./generated-files/lint-rules";
import {markup} from "@internal/markup";

const rulesPath = INTERNAL.append("compiler", "lint", "rules");

export async function main([ruleName]: string[]): Promise<number> {
	if (ruleName === undefined) {
		reporter.error(markup`./scripts lint-create-node [ruleName]`);
		return 1;
	}

	const descriptionKey = toCamelCase(ruleName).replace(/([A-Z+])/g, " $1").trim().toUpperCase().replace(
		/[\s\-\/]/g,
		"_",
	);
	const categoryName = `lint/${ruleName}`;

	// Write rule
	await writeFile(
		rulesPath.append(`${ruleName}.ts`),
		dedent`
			import {createLintVisitor, signals} from "@internal/compiler";
			import {descriptions} from "@internal/diagnostics";

			export default createLintVisitor({
				name: "${ruleName}",
				enter(path) {
					const {node} = path;

					if (false) {
						path.context.addNodeDiagnostic(
							node,
							descriptions.LINT.${descriptionKey},
						);
					}

					return signals.retain;
				},
			});
		`,
	);

	// Write test fixture
	await writeFile(
		rulesPath.append(`${ruleName}.test.rjson`),
		dedent`
			filename: "filename.ts"
			invalid: [
				"
					// insert invalid examples here
				"
			]
			valid: [
				"
					// insert valid examples here
				"
			]
		`,
	);

	// Write docs
	await writeFile(
		ROOT.append("website", "src", "docs", "lint", "rules", `${ruleName}.md`),
		dedent`
			---
			title: Lint Rule ${ruleName}
			layout: layouts/rule.liquid
			showHero: false
			description: MISSING DOCUMENTATION
			eleventyNavigation:
				key: lint-rules/${ruleName}
				parent: lint-rules
				title: ${ruleName}
			---

			# ${ruleName}

			MISSING DOCUMENTATION
		`,
	);

	// Add description
	const diagDescriptionsPath = INTERNAL.append(
		"diagnostics",
		"descriptions",
		"lint.ts",
	);
	let descriptions = await diagDescriptionsPath.readFileText();
	let message = "markup`INSERT MESSAGE HERE`";
	descriptions = descriptions.replace(
		"createDiagnosticsCategory({",
		`createDiagnosticsCategory({\n	${descriptionKey}: {
			category: DIAGNOSTIC_CATEGORIES["${categoryName}"],
			message: ${message},
		},`,
	);
	await writeFile(diagDescriptionsPath, descriptions);

	await generateLintRules();

	return 0;
}
