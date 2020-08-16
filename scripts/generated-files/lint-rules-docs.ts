import {createMockWorker} from "@internal/test-helpers";
import {DiagnosticCategory, DiagnosticsProcessor} from "@internal/diagnostics";
import {printDiagnosticsToString} from "@internal/cli-diagnostics";
import {highlightCode} from "@internal/markup-syntax-highlight";
import {inferDiagnosticLanguageFromFilename} from "@internal/core/common/file-handlers";
import {concatMarkup, joinMarkupLines, markup} from "@internal/markup";
import {markupToHtml} from "@internal/cli-layout";
import {createUnknownPath} from "@internal/path";
import {dedent} from "@internal/string-utils";
import {tests} from "@internal/compiler/lint/rules/tests";
import {ob1Coerce1} from "@internal/ob1";
import {ROOT, modifyGeneratedFile} from "../_utils";
import {getDocRuleDescription, getLintDefs} from "./lint-rules";
import {readFileText} from "@internal/fs";

const {worker, performFileOperation} = createMockWorker();

function pre(inner: string): string {
	return `{% raw %}<pre class="language-text"><code class="language-text">${inner}</code></pre>{% endraw %}`;
}

function highlightPre(filename: string, code: string): string {
	const path = createUnknownPath(filename);
	return pre(
		joinMarkupLines(
			markupToHtml(
				concatMarkup(
					highlightCode({
						path,
						input: code,
						sourceTypeJS: undefined,
						language: inferDiagnosticLanguageFromFilename(path),
						highlight: true,
					}),
					markup`\n`,
				),
			),
		),
	);
}

// Extract the description field from the docs frontmatter
export function extractESLintRuleInfo(
	content: string,
):
	| undefined
	| {
			url: string;
			name: string;
		} {
	const match = content.match(/eslint-rule:(.*)(\n|\r\n)/);
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
) {
	const diagnosticsHTML = await performFileOperation(
		{
			uid: `${category}/${i}/${filename}`,
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
						return {filename};
					},
				},
			});
			processor.normalizer.setInlineSourceText(ref.uid, code);
			processor.addFilter({
				test(diag) {
					return (
						diag.description.category === category ||
						diag.description.category === "parse/js"
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
				excludeFooter: true,
				features: {
					columns: ob1Coerce1(75),
				},
			});
		},
	);

	return [highlightPre(filename, code), pre(diagnosticsHTML)].join("\n");
}

export async function main() {
	const defs = await getLintDefs();

	for (const {docs} of defs) {
		await modifyGeneratedFile(
			{
				path: docs,
				scriptName: "generated-files/lint-rules",
				id: "description",
			},
			async () => {
				const content = await readFileText(docs);
				const eslintInfo = extractESLintRuleInfo(content);

				const lines = [];

				const includeDefaultDescription = content.match(/\n# (.*?)([\n]+)<!--/);
				if (includeDefaultDescription) {
					lines.push(getDocRuleDescription(docs, content), "");
				}

				if (eslintInfo !== undefined) {
					lines.push(
						`**ESLint Equivalent:** [${eslintInfo.name}](${eslintInfo.url})`,
					);
				}

				return {lines};
			},
		);
	}

	for (const ruleName in tests) {
		const rawCases = tests[ruleName];
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

				let hasInvalid = false;
				let hasValid = false;
				for (const {invalid, valid} of cases) {
					if (invalid && invalid.length > 0) {
						hasInvalid = true;
					}
					if (valid && valid.length > 0) {
						hasValid = true;
					}
				}

				if (hasInvalid) {
					lines.push("### Invalid");

					for (const {filename, invalid} of cases) {
						if (invalid) {
							for (let i = 0; i < invalid.length; i++) {
								if (i > 0) {
									lines.push("");
									lines.push("---------------");
									lines.push("");
								}
								lines.push(
									await run(
										(`lint/${ruleName}` as DiagnosticCategory),
										i,
										filename,
										dedent(invalid[i]),
									),
								);
							}
						}
					}
				}

				if (hasValid) {
					lines.push("### Valid");
					for (const {filename, valid} of cases) {
						if (valid) {
							for (const code of valid) {
								lines.push(highlightPre(filename, dedent(code)));
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
