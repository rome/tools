import {parseCommit} from "@internal/commit-parser";
import child = require("child_process");

const PROPERTY_DELIM = "<--ROME-PROPERTY-->";
const LINE_DELIM = "<--ROME-LINE-->";

export interface Commit {
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

/**
 * Parse a raw git log into an array of commit objects
 *
 * @param config - Map of result key names to git pretty placeholders
 * @param opts - Specify a starting and an ending commit
 * @returns - List of commit objects
 */
export function parseCommitLog(
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

	const commits = log.split(LINE_DELIM).reduce(
		(totalCommits, rawCommit) => {
			const values = rawCommit.trim().split(PROPERTY_DELIM);
			if (values.length <= 1) {
				return totalCommits;
			}

			totalCommits.push(
				keys.reduce<Commit>(
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
				),
			);

			return totalCommits;
		},
		([] as Array<Commit>),
	);

	return commits;
}
