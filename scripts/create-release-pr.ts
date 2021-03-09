import {markup} from "@internal/markup";
import {
	updateVersion,
	ROOT,
	reporter,
	modifyGeneratedFile,
} from "./_utils";
import child = require("child_process");
import https = require("https");
import http = require("http");
import {Commit, parseCommitLog, git} from "./vcs/utils";
import {VERSION} from "@internal/core";
import {incrementSemver, parseSemverVersion, SemverModifier, stringifySemver} from "@internal/codec-semver";

function generateChangelogMarkdown(version: string, commits: Commit[]): string[] {
	function renderItems(items: Commit[], title: string, expandable = false) {
		if (items.length === 0) {
			return [];
		}

		let lines = [
			`### ${title}`,
			``,
		];
		if (expandable) {
			lines.push("<details><summary>Click to expand</summary>");
			lines.push("");
		}
		for (const commit of items) {
			lines.push(`- ${commit.subject ? commit.subject.trim() : "_no subject provided_"}`);
		}
		if (expandable) {
			lines.push("");
			lines.push("</details>");
		}
		return lines;
	}

	const features = [];
	const fixes = [];
	const breaking = [];
	const misc = [];

	for (const commit of commits) {
		const {meta} = commit;
		if (meta === undefined) {
			continue;
		}

		if (meta.breaking) {
			breaking.push(commit);
			continue;
		}

		switch (meta.commitType) {
			case "feat": {
				features.push(commit);
				break;
			}

			case "fix": {
				fixes.push(commit);
				break;
			}

			default: {
				misc.push(commit);
				break;
			}
		}
	}

	return [
		`## [${version}](https://github.com/rome/tools/releases/tag/v${version})`,
		...renderItems(features, "Features"),
		...renderItems(fixes, "Bug fixes"),
		...renderItems(breaking, "BREAKING CHANGES"),
		...renderItems(misc, "Miscellaneous", true),
		``,
	];
}

// Determine if the next release should be a major, minor, or patch release
// based on the commits since the last release commit
function inferSemverModifier(commits: Commit[]): SemverModifier {
	for (const commit of commits) {
		if (commit.meta?.breaking) {
			return SemverModifier.MAJOR;
		}

		if (commit.meta?.commitType === "feat") {
			return SemverModifier.MINOR;
		}
	}

	return SemverModifier.PATCH;
}

function inferNewVersion(commits: Commit[]): string {
	const modifier = inferSemverModifier(commits);
	const currentVersion = parseSemverVersion({input: VERSION});
	const newVersion = incrementSemver(currentVersion, modifier);
	return stringifySemver(newVersion);
}

// Check if the current checkout has any uncommitted changes
function isDirty(): boolean {
	const diffStatus = child.spawnSync("git", ["diff", "--exit-code"]).status;
	return diffStatus === 1;
}

/**
 * Check if a given version does not exist on NPM
 *
 * @param version - Version to check against the NPM registry
 * @returns - True if the version does not exist
 */
async function isNewVersion(version: string): Promise<boolean> {
	const res: http.IncomingMessage = await new Promise((resolve) => {
		https.get(
			`https://registry.npmjs.org/rome/${version}`,
			(res) => {
				resolve(res);
			},
		);
	});
	return res.statusCode === 404;
}

export async function main([explicitNewVersion]: string[]): Promise<number> {
	if (isDirty()) {
		reporter.error("Uncommitted changes.");
		return 1;
	}

	const previousBranch = git(["rev-parse", "--abbrev-ref", "HEAD"]);
	const lastVersion = git(["describe", "--tags", "--abbrev=0", "--match", "v[0-9]*"]);

	const commits = parseCommitLog(
		{
			from: lastVersion,
			to: "HEAD",
		},
	);

	async function revert() {
		git(["stash"]);
		git(["checkout", previousBranch]);
	}

	try {
		const newVersion = explicitNewVersion || inferNewVersion(commits);
		const releaseBranch = `release/${newVersion}`;
		git(["checkout", "-b", releaseBranch]);
		await updateVersion(newVersion);

		if (await isNewVersion(newVersion)) {
			git(["add", "package.json"]);
		} else {
			await revert();
			reporter.error(
				`Version <emphasis>${newVersion}</emphasis> already exists, reverting.`,
			);
			return 1;
		}

		reporter.success(
			markup`Version <emphasis>${newVersion}</emphasis> is free.`,
		);

		await modifyGeneratedFile({
			path: ROOT.append("CHANGELOG.md"),
		}, async () => {
			return {
				prepend: true,
				lines: generateChangelogMarkdown(newVersion, commits),
			};
		});
		git(["add", "CHANGELOG.md"]);

		git(["commit", "-m", `Release v${newVersion}`]);
		git(["push", "--set-upstream", "origin", `${releaseBranch}`]);

		git(["checkout", previousBranch]);
		reporter.success(markup`Created release branch ${releaseBranch}`);
		reporter.info(markup`You can now create a pull request at <hyperlink target="https://github.com/rome/tools/pull/new/${releaseBranch}" />.`);
		reporter.info(markup`Once merged it will be automatically published`);

		return 0;
	} catch (err) {
		await revert();
		throw err;
	}
}
