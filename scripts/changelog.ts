import {markup} from "@internal/markup";
import {parseCommit} from "@internal/commit-parser";
import {readFileTextMeta} from "@internal/fs";
import {AbsoluteFilePath} from "@internal/path";
import {PUBLIC_PACKAGES, ROOT, reporter, writeFile} from "./_utils";
import {dedent} from "@internal/string-utils";
import {consumeJSON} from "@internal/codec-json";
import child = require("child_process");
import https = require("https");
import http = require("http");

const PROPERTY_DELIM = "<--ROME-PROPERTY-->";
const LINE_DELIM = "<--ROME-LINE-->";
const GIT_PLACEHOLDERS = {
	authorEmail: "%aE",
	authorName: "%aN",
	body: "%b",
	commit: "%H",
	date: "%ad",
	rawBody: "%B",
	refNames: "%d",
	subject: "%s",
};

interface Commit {
	authorEmail: string;
	authorName: string;
	body: string;
	commit: string;
	date: string;
	rawBody: string;
	refNames?: string;
	subject: string;
	meta?: {
		breaking: boolean;
		commitType: string;
		custom: boolean;
		scope: string;
	};
}

const ReleaseType = {
	major: "major",
	minor: "minor",
	patch: "patch",
};

/**
 * Creates a new git tag based on a version string
 *
 * @param version - Version to name tag
 */
function createTag(version: string): void {
	child.spawnSync("git", ["commit", "-am", `Release ${version}`]);
	child.spawnSync("git", ["tag", version]);
	child.spawnSync("git", ["push"]);
	child.spawnSync("git", ["push", "origin", `v${version}`]);
}

/**
 * Transform a list of commit objects into a map of version names to commits
 *
 * @param commits - Commit list to transform
 * @param currentVersion - Version used to categorize commits since the last tag
 * @returns - Map of version names to commits within a version
 */
function createTagMap(
	commits: Array<Commit>,
	currentVersion: string,
): Record<string, Array<Commit>> {
	const versionMap: Record<string, Array<Commit>> = {
		[currentVersion]: [],
	};
	let currentTag = currentVersion;

	commits.forEach((commit) => {
		const tagMatch =
			commit.refNames && commit.refNames.match(/tag: v\d+\.\d+\.\d+/);
		const tagName = tagMatch && tagMatch[0].replace(/^tag: /, "");
		currentTag = tagName || currentTag;
		if (!currentTag) {
			return;
		}
		versionMap[currentTag] = [...(versionMap[currentTag] || []), commit];
	});

	return versionMap;
}

/**
 * Generates changelog markdown from a map of tags
 *
 * @param tagMap - Map of version names to commits within them
 * @returns - Markdown string
 */
function generateMarkdown(tagMap: Record<string, Array<Commit>>): string {
	function renderItems(items: Array<Commit>, title: string, expandable = false) {
		let result = `## ${title}\n\n`;
		if (expandable) {
			result += "<details><summary>Click to expand</summary>\n\n";
		}
		result += items.map((commit) =>
			`- ${commit.subject ? commit.subject.trim() : "_no subject provided_"}`
		).join("\n");
		if (expandable) {
			result += "</details>";
		}
		return items.length > 0 ? result : "";
	}

	const list = Object.keys(tagMap).map((tag) => {
		const commits = tagMap[tag];
		const features = commits.filter(({meta}) => meta?.commitType === "feat");
		const fixes = commits.filter(({meta}) => meta?.commitType === "fix");
		const breaking = commits.filter(({meta}) => meta?.breaking);
		const misc = commits.filter(({meta}) => !meta?.breaking && meta?.custom);
		return dedent`

			## [${tag}](https://github.com/romefrontend/rome/releases/tag/${tag})

${renderItems(features, "Features")}
${renderItems(fixes, "Bug fixes")}
${renderItems(breaking, "BREAKING CHANGES")}
${renderItems(misc, "Miscellaneous", true)}
		`;
	}).join("\n");

	return dedent`
# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.
${list}
<br><br><br><br>
<img alt="Rome, logo of an ancient Greek spartan helmet" src="https://github.com/romefrontend/rome/raw/main/assets/PNG/logomark_transparent.png" width="128px">
	`;
}

/**
 * Get the current package version
 *
 * @returns - Promise resolving to the current version
 */
async function getCurrentVersion(): Promise<string> {
	const path = ROOT.append("package.json");
	return consumeJSON(await readFileTextMeta(path)).get("version").asString();
}

/**
 * Determine if the next release should be a major, minor, or patch release
 * based on the commits since the last release commit
 *
 * @returns - Type of release
 */
function getReleaseType(): string {
	const version = child.spawnSync("git", ["describe", "--tags", "--abbrev=0"]).stdout.toString().trim();

	const newCommits = parseCommitLog(
		GIT_PLACEHOLDERS,
		{
			from: version,
			to: "HEAD",
		},
	);

	const breaking = newCommits.some((commit) => commit.meta?.breaking);
	if (breaking) {
		return ReleaseType.major;
	}

	const feat = newCommits.some((commit) => commit.meta?.commitType === "feat");
	if (feat) {
		return ReleaseType.minor;
	}

	return ReleaseType.patch;
}

/**
 * Check if the current branch is dirty
 *
 * @returns - True if the current branch is dirty
 */
function isDirty(): boolean {
	const diffStatus = child.spawnSync("git", ["diff", "--exit-code"]).status;
	return diffStatus === 1;
}

/**
 * Check if the current branch is the main branch
 *
 * @returns - True if the current branch is the main branch
 */
function isMainBranch(): boolean {
	const branchName = child.spawnSync(
		"git",
		["rev-parse", "--abbrev-ref", "HEAD"],
	).stdout.toString().trim();
	return branchName === "main";
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

/**
 * Parse a raw git log into an array of commit objects
 *
 * @param config - Map of result key names to git pretty placeholders
 * @param opts - Specify a starting and an ending commit
 * @returns - List of commit objects
 */
function parseCommitLog(
	config: Record<string, string>,
	opts?: {
		from: string;
		to: string;
	},
): Array<Commit> {
	const keys = Object.keys(config);

	const cmd = keys.reduce(
		(full, key) => `${full}${full ? `${PROPERTY_DELIM}` : ""}${config[key]}`,
		"",
	);

	const log = child.spawnSync(
		"git",
		[
			"log",
			`--pretty='format:${cmd}${LINE_DELIM}'`,
			opts ? `${opts.from}..${opts.to}` : ".",
		],
	).stdout.toString();

	const commits = log.split(LINE_DELIM).map((rawCommit) => {
		const values = rawCommit.trim().split(PROPERTY_DELIM);
		return keys.reduce<Commit>(
			(commit, key, index) => {
				const newCommit = {
					...commit,
					[key]: values[index],
				};

				if (config[key] === "%B") {
					const ast = parseCommit({
						input: values[index],
						path: "commit",
					});
					newCommit.meta = {
						breaking: ast.breaking,
						commitType: ast.commitType,
						custom: ast.custom,
						scope: ast.scope,
					};
				}

				return newCommit;
			},
			({} as Commit),
		);
	});

	return commits;
}

/**
 * Raise an error using the reporter and exit the process
 *
 * @param message - Error message to report
 * @param keepAlive - Indicates that this error should not be fatal
 */
function raiseError(message: string, keepAlive = false): void {
	reporter.error(markup`${message}`);
	!keepAlive && process.exit(1);
}

/**
 * Update the package version based on a target release type
 *
 * @param releaseType - Target release type
 * @param cwd
 * @returns - New version
 */
function updateVersion(releaseType: string, cwd: AbsoluteFilePath): string {
	return child.spawnSync(
		"npm",
		["--no-git-tag-version", "--force", "version", releaseType],
		{
			cwd: cwd.join(),
		},
	).stdout.toString().trim();
}

export async function main([version]: Array<string>): Promise<void> {
	// Cache the current version for reverting
	const currentVersion = await getCurrentVersion();

	// Ensure the branch is main
	if (!isMainBranch()) {
		raiseError("Change logs must be generated on the main branch.");
	}
	reporter.success(markup`The <emphasis>main</emphasis> branch is being used.`);

	// Ensure the branch is clean
	if (isDirty()) {
		raiseError("Uncommitted changes exist on the main branch.");
	}
	reporter.success(
		markup`The main branch has <emphasis>no uncommitted changes</emphasis>.`,
	);

	// Update the root package version
	const targetReleaseType = version || getReleaseType();
	const newVersion = updateVersion(targetReleaseType, ROOT);
	reporter.success(
		markup`The root package version was updated to <emphasis>${newVersion}</emphasis>.`,
	);

	// Generate changelog
	const commits = await parseCommitLog(GIT_PLACEHOLDERS);
	const tagMap = createTagMap(commits, newVersion);
	const markdown = generateMarkdown(tagMap);
	await writeFile(ROOT.append("CHANGELOG.md"), markdown);
	child.spawnSync("git", ["add", "CHANGELOG.md"]);
	reporter.success(
		markup`The <emphasis>CHANGELOG.md</emphasis> was successfully generated.`,
	);

	// Ensure the version is not yet taken
	if (!(await isNewVersion(newVersion))) {
		raiseError(
			`Version <emphasis>${newVersion}</emphasis> already exists, reverting to <emphasis>${currentVersion}</emphasis>.`,
			true,
		);
	}
	reporter.success(
		markup`The package version <emphasis>${newVersion}</emphasis> is clear to use.`,
	);

	// Update the rome package version
	updateVersion(newVersion, PUBLIC_PACKAGES.append("rome"));
	reporter.success(
		markup`The rome package version was updated to <emphasis>${newVersion}</emphasis>.`,
	);

	// Create a resulting tag
	createTag(newVersion);
	reporter.success(
		markup`A new <emphasis>${newVersion}</emphasis> git tag was created. To publish, run:`,
	);
	reporter.info(markup`./rome run scripts/publish`);
}
