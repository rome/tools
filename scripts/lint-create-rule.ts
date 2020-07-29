import {PACKAGES, ROOT, reporter, writeFile} from "./_utils";
import {dedent, toCamelCase} from "@romefrontend/string-utils";
import {readFileText} from "@romefrontend/fs";
import {main as generateLintRules} from "./generated-files/lint-rules";
import {markup} from "@romefrontend/cli-layout";

const rulesPath = PACKAGES.append("compiler", "lint", "rules");

export async function main([ruleName]: Array<string>): Promise<number> {
	if (ruleName === undefined) {
		reporter.error(
			markup`./rome run scripts/ast-create-node scripts/lint/add.cjs [ruleName]`,
		);
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
			import {createVisitor, signals} from "@romefrontend/compiler";
			import {descriptions} from "@romefrontend/diagnostics";

			export default createVisitor({
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
		`
			---
			title: Lint Rule ${ruleName}
			layout: layouts/page.liquid
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
	const diagDescriptionsPath = PACKAGES.append(
		"diagnostics",
		"descriptions",
		"lint.ts",
	);
	let descriptions = await readFileText(diagDescriptionsPath);
	descriptions = descriptions.replace(
		"createDiagnosticsCategory({",
		`createDiagnosticsCategory({\n	${descriptionKey}: {
			category: "${categoryName}",
			message: "INSERT MESSAGE HERE",
		},`,
	);
	await writeFile(diagDescriptionsPath, descriptions);

	await generateLintRules();

	return 0;
}
