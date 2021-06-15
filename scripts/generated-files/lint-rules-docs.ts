import {createMockWorker} from "@internal/test-helpers";
import {
	DIAGNOSTIC_CATEGORIES,
	DiagnosticCategory,
	DiagnosticLanguage,
	DiagnosticsProcessor,
	equalCategoryNames,
	joinCategoryName,
} from "@internal/diagnostics";
import {printDiagnosticsToString} from "@internal/cli-diagnostics";
import {highlightCode} from "@internal/markup-syntax-highlight";
import {inferDiagnosticLanguageFromPath} from "@internal/core/common/file-handlers";
import {joinMarkup, joinMarkupLines, markup} from "@internal/markup";
import {markupToHtml} from "@internal/cli-layout";
import {createPath} from "@internal/path";
import {dedent} from "@internal/string-utils";
import {tests} from "@internal/compiler/lint/rules/tests";
import {ROOT, modifyGeneratedFile} from "../_utils";
import {getDocRuleDescription, getLintDefs} from "./lint-rules";
import {OneIndexed} from "@internal/numbers";

const {worker, performFileOperation} = createMockWorker();

function pre(inner: string, language: DiagnosticLanguage = "text"): string {
	return `{% raw %}<pre class="language-${language}"><code class="language-${language}">${inner}</code></pre>{% endraw %}`;
}

function highlightPre(filename: string, code: string): string {
	const path = createPath(filename);
	const language = inferDiagnosticLanguageFromPath(path);
	return pre(
		joinMarkupLines(
			markupToHtml(
				joinMarkup(
					highlightCode({
						path,
						input: code,
						sourceTypeJS: undefined,
						language,
						highlight: true,
					}),
					markup`\n`,
				),
			),
		),
		language,
	);
}

// Extract the description field from the docs frontmatter
export function extractLintRuleInfo(
	content: string,
	type: "eslint" | "tslint" | "stylelint" = "eslint",
):
	| undefined
	| {
			url: string;
			name: string;
		} {
	const match = content.match(new RegExp(`${type}-rule:(.*)(\n|\r\n)`));
	if (match) {
		const url = match[1].trim();

		return {
			url,
			name: url.split("/").pop()!.split(".")[0],
		};
	} else {
		return undefined;
	}
}

async function run(
	category: DiagnosticCategory,
	i: number,
	filename: string,
	code: string,
): Promise<string> {
	const diagnosticsHTML = await performFileOperation(
		{
			uid: `${joinCategoryName(category)}/${i}/${filename}`,
			sourceText: code,
		},
		async (ref) => {
			const res = await worker.api.lint(
				ref,
				{
					applySafeFixes: true,
					prefetchedModuleSignatures: {},
					save: true,
				},
				{},
			);

			const processor = new DiagnosticsProcessor({
				markupOptions: {
					normalizePosition() {
						return {
							path: createPath(filename),
						};
					},
				},
			});
			processor.normalizer.setInlineSourceText(ref.uid, code);
			processor.addEliminationFilter({
				test(diag) {
					return (
						equalCategoryNames(diag.description.category, category) ||
						equalCategoryNames(
							diag.description.category,
							DIAGNOSTIC_CATEGORIES.parse,
						)
					);
				},
			});
			processor.addSuppressions(res.suppressions);
			processor.addDiagnostics(res.diagnostics);

			const diagnostics = processor.getDiagnostics();
			return await printDiagnosticsToString({
				diagnostics,
				suppressions: [],
				format: "html",
				features: {
					columns: new OneIndexed(75),
				},
				printerOptions: {
					fileHandlers: [
						{
							async exists() {
								return true;
							},
						},
					],
				},
			});
		},
	);

	return [highlightPre(filename, code), pre(diagnosticsHTML)].join("\n");
}

export async function main() {
	const defs = await getLintDefs();

	for (const {rules} of defs) {
		for (const {docs} of rules) {
			await modifyGeneratedFile(
				{
					path: docs,
					scriptName: "generated-files/lint-rules",
					id: "description",
				},
				async () => {
					const content = await docs.readFileText();
					const eslintInfo = extractLintRuleInfo(content, "eslint");
					const tslintInfo = extractLintRuleInfo(content, "tslint");
					const styleLintInfo = extractLintRuleInfo(content, "stylelint");

					const lines = [];

					const includeDefaultDescription = content.match(
						/\n# (.*?)([\n]+)<!--/,
					);
					if (includeDefaultDescription) {
						lines.push(getDocRuleDescription(docs, content), "");
					}

					if (eslintInfo !== undefined) {
						lines.push(
							`**ESLint Equivalent:** [${eslintInfo.name}](${eslintInfo.url})`,
						);
					}

					if (tslintInfo !== undefined) {
						lines.push(
							`**TSLint Equivalent:** [${tslintInfo.name}](${tslintInfo.url})`,
						);
					}

					if (styleLintInfo !== undefined) {
						lines.push(
							`**stylelint Equivalent:** [${styleLintInfo.name}](${styleLintInfo.url})`,
						);
					}

					return {lines};
				},
			);
		}
	}

	for (const ruleName in tests) {
		const def = tests[ruleName];
		const rawCases = def.cases;
		const cases = Array.isArray(rawCases) ? rawCases : [rawCases];

		await modifyGeneratedFile(
			{
				path: ROOT.append(`website/src/docs/lint/rules/${ruleName}.md`),
				scriptName: "generated-files/lint-rules-docs",
				id: "examples",
			},
			async () => {
				const lines = [];

				lines.push("## Examples");
				lines.push("\n");

				let hasInvalid = false;
				let hasValid = false;
				for (const elem of cases) {
					const cases = Array.isArray(elem) ? elem : [elem];
					for (const {invalid, valid} of cases) {
						if (invalid && invalid.length > 0) {
							hasInvalid = true;
						}
						if (valid && valid.length > 0) {
							hasValid = true;
						}
					}
				}

				if (hasInvalid) {
					lines.push("### Invalid");
					lines.push("\n");

					for (const singleCase of cases) {
						if (Array.isArray(singleCase)) {
							for (const {filename, invalid} of singleCase) {
								if (invalid) {
									for (let i = 0; i < invalid.length; i++) {
										if (i > 0) {
											lines.push("\n");
											lines.push("---");
											lines.push("\n");
										}
										lines.push(
											await run(def.category, i, filename, dedent(invalid[i])),
										);
									}
								}
							}
						} else {
							const {filename, invalid} = singleCase;
							if (invalid) {
								for (let i = 0; i < invalid.length; i++) {
									if (i > 0) {
										lines.push("\n");
										lines.push("---");
										lines.push("\n");
									}
									lines.push(
										await run(def.category, i, filename, dedent(invalid[i])),
									);
								}
							}
						}
					}
				}

				if (hasValid) {
					lines.push("\n");
					lines.push("### Valid");
					lines.push("\n");
					for (const singleCase of cases) {
						if (Array.isArray(singleCase)) {
							for (const {filename, valid} of singleCase) {
								if (valid) {
									for (const code of valid) {
										lines.push(highlightPre(filename, dedent(code)));
									}
								}
							}
						} else {
							const {filename, valid} = singleCase;
							if (valid) {
								for (const code of valid) {
									lines.push(highlightPre(filename, dedent(code)));
								}
							}
						}
					}
					lines.push("");
				}

				return {lines};
			},
		);
	}
}
