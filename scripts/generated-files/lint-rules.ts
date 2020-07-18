import {PACKAGES, ROOT, modifyGeneratedFile} from "../_utils";
import {lstat, readDirectory, readFileText} from "@romefrontend/fs";
import {AbsoluteFilePath} from "@romefrontend/path";

const lintRulesFolder = PACKAGES.appendList("compiler", "lint", "rules");

const lintRulesDocFolder = ROOT.appendList(
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
		lintRulesFolder.append("index.ts"),
		async () => {
			let lines = [];
			for (const {basename, ruleName} of defs) {
				lines.push(`import ${basename} from "./${ruleName}";`);
			}
			lines.push("");
			lines.push("export const lintTransforms = [");
			for (const {basename} of defs) {
				lines.push(`	${basename},`);
			}
			lines.push("];");
			return {lines};
		},
	);

	// Generate diagnostic categories
	await modifyGeneratedFile(
		PACKAGES.appendList("diagnostics", "categories.ts"),
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
		lintRulesFolder.append("tests.ts"),
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
	function getDocRuleDescription(content: string): string {
		const description = content.match(/description:(.*)/);
		if (description) {
			return description[1];
		} else {
			return "";
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
		lintRulesDocFolder.append("index.md"),
		async () => {
			const lines = [];

			for (const rootCategory of categoryDocsOrder) {
				lines.push(`## ${categoryDocsAliases[rootCategory]}`);

				for (const {basename, ruleName, category, docs} of defs) {
					if (category !== rootCategory) {
						continue;
					}

					const content = await readFileText(docs);
					const description = getDocRuleDescription(content);
					lines.push(
						`- [${basename}](/docs/lint/rules/${ruleName}): ${description}`,
					);
				}
			}

			return {lines};
		},
	);
}
