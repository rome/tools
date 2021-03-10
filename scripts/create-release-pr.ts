import {markup} from "@internal/markup";
import {Commit, parseCommit} from "@internal/codec-commit";
import {ROOT, modifyGeneratedFile, reporter, updateVersion} from "./_utils";
import child = require("child_process");
import https = require("https");
import http = require("http");
import {VERSION} from "@internal/core";
import {
	SemverModifier,
	incrementSemver,
	parseSemverVersion,
	stringifySemver,
} from "@internal/codec-semver";

const COMMIT_DELIM = "<--ROME-LINE-->";

export function git(args: string[]): string {
	const res = child.spawnSync(
		"git",
		args,
		{
			encoding: "utf8",
			cwd: ROOT.join(),
		},
	);
	if (res.error !== undefined) {
		throw res.error;
	}

	return res.stdout.toString().trim();
}

export function parseCommitLog(
	opts: {
		from: string;
		to: string;
	},
): Commit[] {
	const out = git([
		"log",
		`--pretty='format:%B${COMMIT_DELIM}'`,
		`${opts.from}..${opts.to}`,
		"internal",
	]);

	const commitBodies = out.split(COMMIT_DELIM);
	const commits: Commit[] = [];

	for (const commitBody of commitBodies) {
		const root = parseCommit(commitBody);
		commits.push(root);

		// Try to parse each other line in the commit body as a commit
		for (const line of commitBody.split("\n").slice(1)) {
			const commit = parseCommit(line);
			// Only consider this valid if it has a type
			if (commit.type !== undefined) {
				commits.push({
					...commit,
					// Other lines wont have the suffix but inherit it if it exists anyway
					pullRequest: commit.pullRequest ?? root.pullRequest,
				});
			}
		}
	}

	return commits;
}

function generateChangelogMarkdown(version: string, commits: Commit[]): string[] {
	function renderItems(
		items: Commit[],
		title: string,
		expandable: boolean = false,
	) {
		if (items.length === 0) {
			return [];
		}

		let lines = [`### ${title}`, ""];

		if (expandable) {
			lines.push("<details><summary>Click to expand</summary>");
			lines.push("");
		}

		for (const {type, scope, pullRequest, description} of items) {
			let summary = "";

			const hasTypePrefix = expandable && type !== undefined;
			const hasPrefix = hasTypePrefix || scope !== undefined;
			if (hasPrefix) {
				summary += "`";
			}
			if (hasTypePrefix) {
				summary += type;
			}
			if (scope !== undefined) {
				if (hasTypePrefix) {
					summary += "(";
				}
				summary += scope;
				if (hasTypePrefix) {
					summary += ")";
				}
			}
			if (hasPrefix) {
				summary += ": ";
			}

			if (pullRequest === undefined) {
				summary += description;
			} else {
				summary += `[${description}](https://github.com/rome/tools/pull/${pullRequest})`;
			}

			lines.push(`- ${summary}`);
		}

		if (expandable) {
			lines.push("");
			lines.push("</details>");
		}

		lines.push("");
		return lines;
	}

	const features = [];
	const fixes = [];
	const breaking = [];
	const misc = [];
	const internal = [];

	for (const commit of commits) {
		if (commit.breaking) {
			breaking.push(commit);
			continue;
		}

		switch (commit.type) {
			case "feat": {
				features.push(commit);
				break;
			}

			case "fix": {
				fixes.push(commit);
				break;
			}

			case "test":
			case "refactor":
			case "perf":
			case "internal":
			case "chore": {
				internal.push(commit);
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
		"",
		...renderItems(breaking, "BREAKING CHANGES"),
		...renderItems(features, "Features"),
		...renderItems(fixes, "Bug fixes"),
		...renderItems(misc, "Miscellaneous", true),
		...renderItems(internal, "Internal", true),
		"",
	];
}

// Determine if the next release should be a major, minor, or patch release
// based on the commits since the last release commit
function inferSemverModifier(commits: Commit[]): SemverModifier {
	let hasFeature = false;

	for (const commit of commits) {
		if (commit.breaking) {
			return SemverModifier.MAJOR;
		}

		if (commit.type === "feat") {
			hasFeature = true;
		}
	}

	if (hasFeature) {
		return SemverModifier.MINOR;
	} else {
		return SemverModifier.PATCH;
	}
}

async function inferNewVersion(commits: Commit[]): Promise<string> {
	const modifier = inferSemverModifier(commits);
	const currentVersion = parseSemverVersion({input: VERSION});

	let newVersion = incrementSemver(currentVersion, modifier);
	let stringified = stringifySemver(newVersion);

	// Since the `rome` package was used before us, some versions may be taken
	while (!(await isNewVersion(stringified))) {
		newVersion = incrementSemver(newVersion, SemverModifier.PATCH);
		stringified = stringifySemver(newVersion);
	}

	return stringified;
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
		//reporter.error("Uncommitted changes.");
		//return 1;
	}

	const previousBranch = git(["rev-parse", "--abbrev-ref", "HEAD"]);
	const lastVersion = git([
		"describe",
		"--tags",
		"--abbrev=0",
		"--match",
		"v[0-9]*",
	]);

	const commits = parseCommitLog({
		from: lastVersion,
		to: "HEAD",
	});

	async function revert() {
		git(["stash"]);
		git(["checkout", previousBranch]);
	}

	try {
		const newVersion = explicitNewVersion || (await inferNewVersion(commits));
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

		const changelogPath = ROOT.append("CHANGELOG.md");
		await modifyGeneratedFile(
			{
				path: changelogPath,
			},
			async () => {
				return {
					prepend: true,
					lines: generateChangelogMarkdown(newVersion, commits),
				};
			},
		);
		git(["add", "CHANGELOG.md"]);

		git(["commit", "-m", `Release v${newVersion}`]);
		git(["push", "--set-upstream", "origin", `${releaseBranch}`]);

		reporter.success(
			markup`Created release branch <emphasis>${releaseBranch}</emphasis>.`,
		);
		reporter.info(
			markup`Please review <filelink target="${changelogPath.join()}">CHANGELOG.md</filelink> and remove unrelated entries.`,
		);
		reporter.br();
		reporter.info(
			markup`Once verified, you can create a pull request at: <hyperlink target="https://github.com/rome/tools/pull/new/${releaseBranch}" />`,
		);
		reporter.info(
			markup`When merged and approved, the release will be automatically published`,
		);

		return 0;
	} catch (err) {
		await revert();
		throw err;
	}
}
