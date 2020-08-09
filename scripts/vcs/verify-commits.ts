import {GIT_PLACEHOLDERS, reporter} from "../_utils";
import {markup} from "@internal/markup";
import {PERMITTED_COMMIT_TYPES} from "./constants";
import {parseCommitLog} from "./utils";

export async function main(): Promise<number> {
	const commits = parseCommitLog(
		GIT_PLACEHOLDERS,
		{
			from: "main",
			to: "HEAD",
		},
	);

	const invalidCommits = commits.filter((commit) => {
		if (!commit.meta) {
			return true;
		}

		return !PERMITTED_COMMIT_TYPES.includes(commit.meta.commitType);
	});

	if (invalidCommits.length === 0) {
		reporter.success(markup`All the commits follow the correct pattern.`);
		return 0;
	}
	reporter.warn(
		markup`One or more commits do not follow the correct pattern, here's the list:`,
	);
	reporter.list(
		invalidCommits.map((commit) => {
			return markup`Commit ${commit.commit}, with message <emphasis>${commit.rawBody}</emphasis>`;
		}),
		{
			ordered: true,
		},
	);

	reporter.info(
		markup`You can check <filelink target="../CONTRIBUTING.md">the internal guidelines (CONTRIBUTING.md)</filelink> and read about how to write a correct commit message.`,
	);
	reporter.info(
		markup`You can run <emphasis>git rebase -i HEAD~${commits.length}</emphasis> and reword the incorrect commits.`,
	);

	return 1;
}
