import {parseConventionalCommit} from "@internal/commit-parser";
import child = require("child_process");
import {createUIDPath} from "@internal/path";
import { ROOT } from "scripts/_utils";

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
	meta: {
		breaking: boolean;
		commitType: string;
		custom: boolean;
		scope: string;
	};
}

const placeholders = [
	// authorEmail
	"%aE",
	// authorName
	"%aN",
	// body
	"%b",
	// commit
	"%H",
	// date
	"%ad",
	// rawBody
	"%B",
	// refNames
	"%d",
	// subject
	"%s",
];

export function git(args: string[]): string {
	const res = child.spawnSync("git", args, {
		encoding: "utf8",
		cwd: ROOT.join(),
	});
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
	const log = git(
		[
			"log",
			`--pretty='format:${placeholders.join(PROPERTY_DELIM)}${LINE_DELIM}'`,
			`${opts.from}..${opts.to}`,
			"internal",
		],
	);

	const lines = log.split(LINE_DELIM);
	const commits: Commit[] = [];

	for (const line of lines) {
		const values = line.trim().split(PROPERTY_DELIM);
		if (values.length <= 1) {
			continue;
		}

		const rawBody = values[5];

		const ast = parseConventionalCommit({
			input: rawBody,
			path: createUIDPath("commit/body"),
		});

		const meta: Commit["meta"] = {
			breaking: ast.breaking,
			commitType: ast.commitType,
			custom: ast.custom,
			scope: ast.scope,
		};

		const commit: Commit = {
			authorEmail: values[0],
			authorName: values[1],
			body: values[2],
			commit: values[3],
			date: values[4],
			rawBody,
			refNames: values[6],
			subject: values[7],
			meta,
		};

		commits.push(commit);
	}

	return commits;
}
