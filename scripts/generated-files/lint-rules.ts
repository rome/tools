import {INTERNAL, ROOT, modifyGeneratedFile} from "../_utils";
import {lstat, readDirectory, readFileText} from "@internal/fs";
import {AbsoluteFilePath} from "@internal/path";
import {pretty} from "@internal/pretty-format";

const lintRulesFolder = INTERNAL.append("compiler", "lint", "rules");

const lintRulesDocFolder = ROOT.append(
	"website",
	"src",
	"docs",
	"lint",
	"rules",
);

type LintDefinition = {
	docs: AbsoluteFilePath;
	hasRJSON: boolean;
	basename: string;
	category: string;
	ruleName: string;
};

export async function main() {
	let defs: Array<LintDefinition> = [];

	for (const categoryPath of await readDirectory(lintRulesFolder)) {
		const category = categoryPath.getBasename();
		if ((await lstat(categoryPath)).isFile()) {
			continue;
		}

		const categoryFiles = await readDirectory(categoryPath);
		for (const path of categoryFiles) {
			if (path.hasEndExtension("ts") && !path.hasEndExtension("test.ts")) {
				const basename = path.getExtensionlessBasename();
				const ruleName = `${category}/${basename}`;

				defs.push({
					docs: lintRulesDocFolder.append(`${ruleName}.md`),
					hasRJSON: categoryFiles.has(
						categoryPath.append(`${basename}.test.rjson`),
					),
					basename,
					category,
					ruleName,
				});
			}
		}
	}

	defs = defs.sort((a, b) => {
		return a.ruleName.localeCompare(b.ruleName);
	});

	// Generate compiler rules index
	await modifyGeneratedFile(
		{
			path: lintRulesFolder.append("index.ts"),
			scriptName: "generated-files/lint-rules",
		},
		async () => {
			let lines = [];
			for (const {basename, ruleName} of defs) {
				lines.push(`import ${basename} from "./${ruleName}";`);
			}
			lines.push(`import {AnyVisitors} from "@internal/compiler";`);
			lines.push("");
			lines.push("export const lintTransforms: AnyVisitors = [");
			for (const {basename} of defs) {
				lines.push(`	${basename},`);
			}
			lines.push("];");
			return {lines};
		},
	);

	// Generate diagnostic categories
	await modifyGeneratedFile(
		{
			path: INTERNAL.append("diagnostics", "categories.ts"),
			scriptName: "generated-files/lint-rules",
		},
		async () => {
			const lines = ["type LintDiagnosticCategory ="];
			for (const {ruleName} of defs) {
				lines.push(`	| "lint/${ruleName}"`);
			}
			lines.push(";");
			return {lines};
		},
	);

	// Generate tests index
	await modifyGeneratedFile(
		{
			path: lintRulesFolder.append("tests.ts"),
			scriptName: "generated-files/lint-rules",
		},
		async () => {
			const lines = [];
			for (const {basename, ruleName, hasRJSON} of defs) {
				if (hasRJSON) {
					lines.push("// @ts-ignore");
					lines.push(`import ${basename} from "./${ruleName}.test.rjson";`);
				}
			}
			lines.push("");
			lines.push("export const tests: Tests = {");
			for (const {basename, ruleName, hasRJSON} of defs) {
				if (hasRJSON) {
					lines.push(`	"${ruleName}": ${basename},`);
				}
			}
			lines.push("};");
			return {lines};
		},
	);

	// Extract the description field from the docs frontmatter
	function getDocRuleDescription(
		path: AbsoluteFilePath,
		content: string,
	): string {
		const description = content.match(/description:(.*)\n/);
		if (description) {
			return description[1].trim();
		} else {
			throw new Error(pretty`${path.join()} did not contain a description: ${content}`);
		}
	}

	// Used to map lint category name to docs headings
	const categoryDocsAliases = {
		js: "JavaScript",
		ts: "TypeScript",
		"jsx-a11y": "JSX Accessibility",
		react: "React",
	};

	// Order we want to display the categories
	const categoryDocsOrder: Array<keyof typeof categoryDocsAliases> = [
		"js",
		"ts",
		"jsx-a11y",
		"react",
	];

	// Generate lint docs index
	await modifyGeneratedFile(
		{
			path: lintRulesDocFolder.append("index.md"),
			scriptName: "generated-files/lint-rules",
		},
		async () => {
			const lines = [];

			for (const rootCategory of categoryDocsOrder) {
				lines.push(`<section class="rules">`);
				lines.push("");
				lines.push(`## ${categoryDocsAliases[rootCategory]}`);

				for (const {basename, ruleName, category, docs} of defs) {
					if (category !== rootCategory) {
						continue;
					}

					const content = await readFileText(docs);
					const description = getDocRuleDescription(docs, content);
					lines.push(`<div class="rule">`);
					lines.push(`<h3 data-toc-exclude id="${basename}">`);
					lines.push(`<a href="/docs/lint/rules/${ruleName}">${basename}</a>`);
					lines.push(`<a class="header-anchor" href="#${basename}"></a>`);
					lines.push("</h3>");

					lines.push(description);
					lines.push("</div>");
				}
				lines.push("</section>");
			}

			lines.push("");

			return {lines};
		},
	);
}
