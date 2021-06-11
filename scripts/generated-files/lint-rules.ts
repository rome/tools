import {INTERNAL, ROOT, modifyGeneratedFile} from "../_utils";
import {AbsoluteFilePath} from "@internal/path";
import {pretty} from "@internal/pretty-format";
import {dedent, toCamelCase} from "@internal/string-utils";
import {escapeXHTMLEntities} from "@internal/html-parser";

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

const categories: Map<string, string[]> = new Map();

export async function getLintDefs(): Promise<LintDefinition[]> {
	let defs: LintDefinition[] = [];

	for (const categoryPath of await lintRulesFolder.readDirectory()) {
		const category = categoryPath.getBasename();
		if ((await categoryPath.lstat()).isFile()) {
			continue;
		}

		categories.set(category, []);

		const categoryPaths = await categoryPath.readDirectory();
		for (const path of categoryPaths) {
			if (
				path.getBasename()[0] !== "." &&
				path.hasEndExtension("ts") &&
				!path.hasEndExtension("test.ts")
			) {
				const basename = path.getExtensionlessBasename();
				const ruleName = `${category}/${basename}`;

				const currentCategory = categories.get(category);

				currentCategory?.push(basename);

				defs.push({
					docs: lintRulesDocFolder.append(`${ruleName}.md`),
					hasRJSON: categoryPaths.has(
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

	return defs;
}

// Extract the description field from the docs frontmatter
export function getDocRuleDescription(
	path: AbsoluteFilePath,
	content: string,
): string {
	const description = content.match(/description:(.*)(\n|\r\n)/);
	if (description) {
		return description[1].trim();
	} else {
		throw new Error(
			pretty`${path.join()} did not contain a description: ${content}`,
		);
	}
}

export async function main() {
	const defs = await getLintDefs();

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
			lines.push(`import {AnyVisitor} from "@internal/compiler";`);
			lines.push(`import {LintRuleName} from "./categories";`);
			lines.push("");
			lines.push(
				"export const lintTransforms: Map<LintRuleName, AnyVisitor> = new Map();",
			);
			for (const {basename, ruleName} of defs) {
				lines.push(`lintTransforms.set("${ruleName}", ${basename});`);
			}
			lines.push("");

			lines.push("export const lintRuleNames: LintRuleName[] = [");
			for (const {ruleName} of defs) {
				lines.push(`	"${ruleName}",`);
			}
			lines.push("];");
			lines.push("");

			return {lines};
		},
	);

	// Generate compiler rules index
	await modifyGeneratedFile(
		{
			path: lintRulesFolder.append("categories.ts"),
			scriptName: "generated-files/lint-rules",
		},
		async () => {
			let lines = [];

			lines.push("export type LintCategories = ");
			for (const [category] of categories) {
				lines.push(`	| "${category}"`);
			}
			lines.push("");

			lines.push("export const lintCategories: Set<LintCategories> = new Set();");
			for (const [category] of categories) {
				lines.push(`lintCategories.add("${category}");`);
			}

			lines.push("");

			const typedCategories: Map<string, string> = new Map();
			for (const [category, rules] of categories) {
				const typeName = `${toCamelCase(category, {forcePascal: true})}Rules`;
				typedCategories.set(category, typeName);
				lines.push(`export type ${typeName} = `);
				for (const rule of rules) {
					lines.push(`	| "${rule}"`);
				}
				lines.push(";");
				lines.push("");
			}

			const finalTypes: Map<string, string> = new Map();
			for (const [category, typedCategory] of typedCategories) {
				const type = `${typedCategory}CategoryRules`;
				finalTypes.set(category, type);
				lines.push(`export type ${type} = { `);
				lines.push(`	[key in ${typedCategory}]?: boolean`);
				lines.push("}");
				lines.push("");
			}

			lines.push(
				// rome-ignore lint/js/noTemplateCurlyInString: needed to generate template literal type in TS
				"export type LintRuleName = `${LintCategories}/${" +
				Array.from(typedCategories.values()).join(" | ") +
				"}`",
			);
			lines.push("");

			lines.push("export type LintRules = { ");
			for (const [category, typedCategory] of finalTypes) {
				lines.push(`	"${category}"?: ${typedCategory}`);
			}
			lines.push("}");
			lines.push("");

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
			const lines = ["export type DiagnosticLintCategory ="];
			for (const {category, basename} of defs) {
				lines.push(`	| ["lint", "${category}", "${basename}"]`);
			}
			lines.push(";");

			lines.push(
				"export const lintCategoryNameMap: {[name in DiagnosticLintCategoryString]: DiagnosticLintCategory} = {",
			);
			for (const {ruleName, category, basename} of defs) {
				lines.push(
					`  "lint/${ruleName}": ["lint", "${category}", "${basename}"],`,
				);
			}
			lines.push("};");
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
					lines.push("// @ts-expect-error");
					lines.push(`import ${basename} from "./${ruleName}.test.rjson";`);
				}
			}
			lines.push("");
			lines.push("export const tests: Tests = {");
			for (const {basename, ruleName, category, hasRJSON} of defs) {
				if (hasRJSON) {
					lines.push(`	"${ruleName}": {`);
					lines.push(`    category: ["lint", "${category}", "${basename}"],`);
					lines.push(`    cases: ${basename},`);
					lines.push("  },");
				}
			}
			lines.push("};");
			return {lines};
		},
	);

	// Used to map lint category name to docs headings
	const categoryDocsAliases = {
		js: {
			title: "JavaScript",
			credits: `<a href="https://eslint.org/">ESLint</a>`,
		},
		ts: {
			title: "TypeScript",
			credits: undefined,
		},
		a11y: {
			title: "Accessibility (JSX and HTML)",
			credits: dedent`
				<a href="https://github.com/jsx-eslint/eslint-plugin-jsx-a11y">eslint-plugin-jsx-a11y</a>
				and <a href="https://axe-linter.deque.com/">axe Linter</a>
			`,
		},
		"jsx-a11y": {
			title: "JSX Accessibility",
			credits: `<a href="https://github.com/jsx-eslint/eslint-plugin-jsx-a11y">eslint-plugin-jsx-a11y</a>`,
		},
		react: {
			title: "React",
			credits: `<a href="https://github.com/yannickcr/eslint-plugin-react">eslint-plugin-react</a>`,
		},
		css: {
			title: "CSS",
			credits: `<a href="https://stylelint.io/">stylelint</a>`,
		},
	};

	// Order we want to display the categories
	const categoryDocsOrder: Array<keyof typeof categoryDocsAliases> = [
		"js",
		"ts",
		"a11y",
		"jsx-a11y",
		"react",
		"css",
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
				const {title, credits} = categoryDocsAliases[rootCategory];
				lines.push("<section>");
				lines.push(`<h2>${title}</h2>`);

				if (credits !== undefined) {
					lines.push(
						`<p>Rule semantics and descriptions taken from ${credits}. See individual rule docs for direct references.</p>`,
					);
				}

				for (const {basename, ruleName, category, docs} of defs) {
					if (category !== rootCategory) {
						continue;
					}

					const content = await docs.readFileText();
					const description = getDocRuleDescription(docs, content);
					lines.push(`<div class="rule">`);
					lines.push(
						dedent`
							<h3 data-toc-exclude id="${basename}">
								<a href="/docs/lint/rules/${ruleName}">${basename}</a>
								<a class="header-anchor" href="#${basename}"></a>
							</h3>
						`,
					);
					lines.push(escapeXHTMLEntities(description));
					lines.push("</div>");
				}

				lines.push("</section>");
			}

			return {lines};
		},
	);
}
