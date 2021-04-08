import {INTERNAL, ROOT, createDirectory, reporter, writeFile} from "./_utils";
import {dedent} from "@internal/string-utils";
import {main as generatedPrefixes} from "./generated-files/css-prefix";
import {markup} from "@internal/markup";

const basePath = INTERNAL.append("compiler", "transforms", "compile");
const compilerPath = basePath.append("css-handler", "prefix", "prefixes");
const testPath = basePath.append("test-fixtures", "css-handler", "prefix");

export async function main([prefixName]: string[]): Promise<number> {
	if (prefixName === undefined) {
		reporter.error(markup`./script compiler-create-prefix [prefixName]`);
		return 1;
	}

	// Write rule
	await writeFile(
		compilerPath.append(`${prefixName}.ts`),
		dedent`
			import {
				createPrefixVisitor,
				prefixCSSProperty,
			} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

			export default [
				createPrefixVisitor({
					name: "${prefixName}",
					enter(path) {
						return prefixCSSProperty({
							path,
							propertyName: "${prefixName}",
							browserFeaturesKey: "",
						});
					},
				}),
			];
		`,
	);

	const fileName = testPath.append(prefixName);
	await createDirectory(fileName);

	// Write test fixture
	await writeFile(
		fileName.append("input.css"),
		dedent`
			.style {
				${prefixName}: none;
			}
		`,
	);

	// Write docs
	await writeFile(
		ROOT.append(
			"website",
			"src",
			"docs",
			"css-handler",
			"prefix",
			`${prefixName}.md`,
		),
		dedent`
			---
			title: Prefix ${prefixName}
			layout: layouts/prefix.liquid
			showHero: false
			description: MISSING DOCUMENTATION
			eleventyNavigation:
				key: css-handler/prefix/${prefixName}
				parent: css-handler
				title: ${prefixName}
			---

			# ${prefixName}

			MISSING DOCUMENTATION
		`,
	);

	// TODO: check later
	// Add description
	// const diagDescriptionsPath = INTERNAL.append(
	// 	"diagnostics",
	// 	"descriptions",
	// 	"lint.ts",
	// );
	// let descriptions = await diagDescriptionsPath.readFileText();
	// let message = "markup`INSERT MESSAGE HERE`";
	// descriptions = descriptions.replace(
	// 	"createDiagnosticsCategory({",
	// 	`createDiagnosticsCategory({\n	${descriptionKey}: {
	// 		category: DIAGNOSTIC_CATEGORIES["${categoryName}"],
	// 		message: ${message},
	// 	},`,
	// );
	// await writeFile(diagDescriptionsPath, descriptions);

	await generatedPrefixes();

	return 0;
}
