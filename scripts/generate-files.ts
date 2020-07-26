import {main as virtualModulesMain} from "./generated-files/virtual-modules";
import {main as ast} from "./generated-files/ast";
import {main as lintRules} from "./generated-files/lint-rules";
import {main as lintRulesDocs} from "./generated-files/lint-rules-docs";
import {reporter, setForceGenerated} from "./_utils";
import {parseCLIFlags} from "@romefrontend/cli-flags";
import child = require("child_process");
import {markup} from "@romefrontend/cli-layout";

export async function main(args: Array<string>) {
	const flags = await parseCLIFlags(
		reporter,
		args,
		{
			programName: "./rome run scripts/lint-rules-docs",
			defineFlags(c) {
				return {
					force: c.get("force").asBoolean(false),
				};
			},
		},
	).init();

	if (flags.force) {
		setForceGenerated(true);
	}

	reporter.info(markup`Generating files`);

	await Promise.all([
		virtualModulesMain(),
		ast(),
		lintRules(),
		lintRulesDocs(),
	]);

	reporter.hr();

	// Check that `git status` is fine
	const out = child.spawnSync("git", ["ls-files", "-m"]).stdout.toString();
	if (out === "") {
		reporter.success(markup`Generated files up-to-date`);
	} else {
		reporter.info(markup`Modified uncomitted files:`);
		reporter.list(out.trim().split("\n").map((filename) => markup`${filename}`));
		reporter.info(
			markup`To fix this run <code>./rome run scripts/generate-files</code> and commit the results`,
		);
		return 1;
	}

	return 0;
}
