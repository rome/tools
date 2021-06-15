import {test} from "rome";
import {LintRequest, LintResult, lint} from "@internal/compiler";
import {ProjectConfig, createDefaultProjectConfig} from "@internal/project";
import {parseJS} from "@internal/js-parser";
import {dedent} from "@internal/string-utils";
import {DIAGNOSTIC_CATEGORIES, equalCategoryNames} from "@internal/diagnostics";

function createLintTransformOptions(
	sourceText: string,
	mutateConfig: (config: ProjectConfig) => ProjectConfig,
): LintRequest {
	return {
		applySafeFixes: false,
		suppressionExplanation: "",
		sourceText,
		ast: parseJS({
			input: sourceText,
		}),
		options: {},
		project: {
			config: mutateConfig(createDefaultProjectConfig()),
		},
	};
}

function createLintTransformSuppressions(
	sourceText: string,
	mutateConfig: (config: ProjectConfig) => ProjectConfig,
): LintRequest {
	return {
		applySafeFixes: true,
		suppressionExplanation: "test suppression",
		sourceText,
		ast: parseJS({
			input: sourceText,
		}),
		options: {
			lint: {
				hasDecisions: true,
				globalDecisions: [
					{
						action: "suppress",
						category: DIAGNOSTIC_CATEGORIES["lint/js/noVar"],
						categoryValue: undefined,
					},
				],
			},
		},
		project: {
			config: mutateConfig(createDefaultProjectConfig()),
		},
	};
}

test(
	"disabledLintRules single",
	async (t) => {
		function hasUndeclaredDiag(res: LintResult): boolean {
			for (const diag of res.diagnostics) {
				if (
					equalCategoryNames(
						diag.description.category,
						DIAGNOSTIC_CATEGORIES["lint/js/noUndeclaredVariables"],
					)
				) {
					return true;
				}
			}
			return false;
		}

		// Make sure when it's not disabled the diagnostic is present
		const res = await lint(
			createLintTransformOptions("foo;", (config) => config),
		);
		t.true(hasUndeclaredDiag(res));

		// Make sure when it's not disabled the diagnostic it is not present
		const res2 = await lint(
			createLintTransformOptions(
				"foo;",
				(config) => ({
					...config,
					lint: {
						...config.lint,
						rules: {
							...config.lint.rules,
							js: new Map([["noUndeclaredVariables", false]]),
						},
					},
				}),
			),
		);
		t.false(hasUndeclaredDiag(res2));
	},
);

test(
	"disable a whole category",
	async (t) => {
		function hasUndeclaredDiag(res: LintResult): boolean {
			const results = {
				undeclared: false,
				unused: false,
			};
			for (const diag of res.diagnostics) {
				if (
					equalCategoryNames(
						diag.description.category,
						DIAGNOSTIC_CATEGORIES["lint/js/noUndeclaredVariables"],
					)
				) {
					results.undeclared = true;
				}
				if (
					equalCategoryNames(
						diag.description.category,
						DIAGNOSTIC_CATEGORIES["lint/js/noUnusedVariables"],
					)
				) {
					results.unused = true;
				}
			}
			return results.unused && results.undeclared;
		}

		// Make sure when it's not disabled the diagnostic is present
		const res = await lint(
			createLintTransformOptions(
				`foo; const something = "lorem";`,
				(config) => config,
			),
		);
		t.true(hasUndeclaredDiag(res));

		// Make sure when it's not disabled the diagnostic it is not present
		const res2 = await lint(
			createLintTransformOptions(
				`foo; const something = "lorem";`,
				(config) => ({
					...config,
					lint: {
						...config.lint,
						rules: {
							...config.lint.rules,
							js: false,
						},
					},
				}),
			),
		);
		t.false(hasUndeclaredDiag(res2));
	},
);

test(
	"disabledLintRules all rules",
	async (t) => {
		const res = await lint(
			createLintTransformOptions(
				"foo;",
				(config) => ({
					...config,
					lint: {
						...config.lint,
						enabled: false,
					},
				}),
			),
		);
		t.is(res.diagnostics.length, 0);
	},
);

test(
	"format disabled",
	async (t) => {
		const code = "wacky\n\tformatting( yes,\nok );";
		const res = await lint(
			createLintTransformOptions(
				code,
				(config) => ({
					...config,
					format: {
						...config.format,
						enabled: false,
					},
				}),
			),
		);
		t.is(res.formatted, code);
	},
);

test(
	"should add a new suppression on an existing suppression",
	async (t) => {
		const code = dedent`
			// rome-ignore lint/js/noUnusedVariables: suppressed via --review
			var foo = 5;
		`;
		const res = await lint(
			createLintTransformSuppressions(
				code,
				(config) => ({
					...config,
					lint: {
						...config.lint,
						requireSuppressionExplanations: true,
					},
					format: {
						...config.format,
						enabled: true,
					},
				}),
			),
		);

		t.true(
			res.formatted.includes(
				"rome-ignore lint/js/noVar lint/js/noUnusedVariables: suppressed via --review",
			),
		);
	},
);
