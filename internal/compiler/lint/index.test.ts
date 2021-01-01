import {test} from "rome";
import {LintRequest, LintResult, lint, lintRuleNames} from "@internal/compiler";
import {ProjectConfig, createDefaultProjectConfig} from "@internal/project";
import {parseJS} from "@internal/js-parser";

function createLintTransformOptions(
	sourceText: string,
	mutateConfig: (config: ProjectConfig) => ProjectConfig,
): LintRequest {
	return {
		applySafeFixes: false,
		suppressionExplanation: "",
		sourceText,
		ast: parseJS({
			path: "unknown",
			input: sourceText,
		}),
		options: {},
		project: {
			configHashes: [],
			config: mutateConfig(createDefaultProjectConfig()),
			directory: undefined,
		},
	};
}

test(
	"disabledLintRules single",
	async (t) => {
		function hasUndeclaredDiag(res: LintResult): boolean {
			for (const diag of res.diagnostics) {
				if (diag.description.category === "lint/js/noUndeclaredVariables") {
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
						disabledRules: ["js/noUndeclaredVariables"],
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
						disabledRules: lintRuleNames,
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
		t.is(res.src, code);
	},
);
