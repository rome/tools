import {main as virtualModulesMain} from "./generated-files/virtual-modules";
import {main as ast} from "./generated-files/ast";
import {main as lintRules} from "./generated-files/lint-rules";
import {main as lintRulesDocs} from "./generated-files/lint-rules-docs";
import {main as websiteIntro} from "./generated-files/website-intro";
import {reporter, setForceGenerated} from "./_utils";
import {parseCLIFlagsFromProcess} from "@internal/cli-flags";
import child = require("child_process");
import {markup} from "@internal/markup";

export async function main(args: Array<string>) {
	const flags = await parseCLIFlagsFromProcess({
		reporter,
		args,
		defineFlags(c) {
			return {
				force: c.get("force").asBoolean(false),
			};
		},
	}).init();

	if (flags.force) {
		setForceGenerated(true);
	}

	reporter.info(markup`Generating files`);

	for (const callback of [
		lintRules,
		virtualModulesMain,
		ast,
		websiteIntro,
		lintRulesDocs,
	]) {
		await callback();
	}

	reporter.hr();

	// Check that `git status` is fine
	const out = child.spawnSync("git", ["ls-files", "-m"]).stdout.toString();
	if (out === "") {
		reporter.success(markup`Generated files up-to-date`);
	} else {
		reporter.info(markup`Modified uncomitted files:`);
		reporter.list(out.trim().split("\n").map((filename) => markup`${filename}`));
		reporter.info(
			markup`To fix this run <code>./rome run scripts/generate-all-files</code> and commit the results`,
		);
		return 1;
	}

	return 0;
}
