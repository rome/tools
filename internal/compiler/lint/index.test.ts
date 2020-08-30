import {test} from "rome";
import {LintRequest, LintResult, lint, lintRuleNames} from "@internal/compiler";
import {template} from "@internal/js-ast-utils";
import {ProjectConfig, createDefaultProjectConfig} from "@internal/project";
import {JSRoot} from "@internal/ast";

function createLintTransformOptions(
	ast: JSRoot,
	mutateConfig: (config: ProjectConfig) => ProjectConfig,
): LintRequest {
	return {
		applySafeFixes: false,
		suppressionExplanation: "",
		sourceText: "",
		ast,
		options: {},
		project: {
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
			createLintTransformOptions(template.root`foo;`, (config) => config),
		);
		t.true(hasUndeclaredDiag(res));

		// Make sure when it's not disabled the diagnostic it is not present
		const res2 = await lint(
			createLintTransformOptions(
				template.root`foo;`,
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
				template.root`foo;`,
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
		const res = await lint(
			createLintTransformOptions(
				template.root`wacky\n\tformatting( yes,\nok );`,
				(config) => ({
					...config,
					format: {
						...config.format,
						enabled: false,
					},
				}),
			),
		);
		t.snapshot(res);
	},
);
