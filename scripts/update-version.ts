import {PUBLIC_PACKAGES, ROOT, exec, reporter} from "./_utils";
import {AbsoluteFilePath} from "@internal/path";
import https = require("https");
import http = require("http");

import child = require("child_process");
import {readFileText} from "@internal/fs";
import {markup} from "@internal/markup";

async function runNPMVersion(
	args: Array<string>,
	cwd: AbsoluteFilePath,
): Promise<string> {
	const res = child.spawnSync(
		"npm",
		["--no-git-tag-version", "version", ...args],
		{
			cwd: cwd.join(),
		},
	);

	if (res.status !== 0) {
		reporter.error(
			markup`npm version failed. stderr: ${res.stderr.toString()}, args: ${args.join()}, cwd: ${cwd.join()}`,
		);
		process.exit(1);
	}

	return res.stdout.toString().trim();
}

export async function main(args: Array<string>) {
	// Ensure we're on the main branch
	const branchRes = child.spawnSync(
		"git",
		["rev-parse", "--abbrev-ref", "HEAD"],
	);
	const branch = branchRes.stdout.toString().trim();
	if (branch !== "main") {
		reporter.error(markup`On branch ${branch} instead of main`);
		return 1;
	}

	// Ensure git isn"t dirty
	const gitRes = child.spawnSync("git", ["diff", "--exit-code"]);
	if (gitRes.status === 1) {
		//reporter.error(markup`Uncommitted changes`);
		//return 1;
	}

	// Get current version so we can revert to it later if necessary
	const currentVersion = String(
		JSON.parse(await readFileText(ROOT.append("package.json"))).version,
	);

	// Update root
	const version = (await runNPMVersion(args, ROOT)).slice(1);
	reporter.info(markup`New version: ${version}`);

	// Check this isn't an already published version
	const res: http.IncomingMessage = await new Promise((resolve) => {
		https.get(
			`https://registry.npmjs.org/rome/${version}`,
			(res) => {
				resolve(res);
			},
		);
	});

	if (res.statusCode !== 404) {
		reporter.error(
			markup`This version already exists. Reverting root version update.`,
		);
		await runNPMVersion([currentVersion], ROOT);
		return 1;
	}

	// Update rome package
	await runNPMVersion([version], PUBLIC_PACKAGES.append("rome"));

	// Create commit and tag
	await exec("git", ["commit", "-am", `Release v${version}`]);
	await exec("git", ["tag", `v${version}`]);
	await exec("git", ["push"]);
	await exec("git", ["push", "origin", `v${version}`]);

	reporter.success(markup`Created tag and release commit. To publish run:`);
	reporter.info(markup`./rome run scripts/publish`);
	return 0;
}
