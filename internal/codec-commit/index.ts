export interface CommitMetadata {
	readonly type: undefined | string;
	readonly description: string;
	readonly scope: undefined | string;
	readonly breaking: boolean;
	readonly pullRequest: undefined | number;
	readonly fixesIssues: number[];
}

// TODO: Previously there was usage of ParserCore here, but it ended up having to deserialize the tokens
// and was more complex than this regex approach. If we ever need to perform actual validation, and not
// metadata extraction, we can go back to ParserCore but implement it with token state.

export function parseCommit(input: string): CommitMetadata {
	let line = input.split("\n")[0];

	let type: CommitMetadata["type"];
	let scope: CommitMetadata["scope"];
	let breaking: CommitMetadata["breaking"] = false;
	let pullRequest: CommitMetadata["pullRequest"];
	let fixesIssues: CommitMetadata["fixesIssues"] = [];
	let description: CommitMetadata["description"] = "";

	// Extract the type
	const typeMatch = line.match(/^([a-zA-Z]+)(?:[:(!])/);
	if (typeMatch != null) {
		type = typeMatch[1];
		line = line.slice(type.length);

		// Extract scope
		if (line[0] === "(") {
			const scopeMatch = line.match(/^\((.*?)\)/);
			if (scopeMatch != null) {
				scope = scopeMatch[1];
				line = line.slice(scopeMatch[0].length);
			}
		}

		if (line[0] === "!") {
			breaking = true;
			line = line.slice(1);
		}

		if (line[0] === ":") {
			line = line.slice(1);
		}
	}

	description = line.trim();

	// Extract pull request id from description
	const pullRequestMatch = description.match(/\(#(\d+)\)$/);
	if (pullRequestMatch != null) {
		pullRequest = Number(pullRequestMatch[1]);
		description = description.slice(0, -pullRequestMatch[0].length);
	}

	// TODO fixesIssues

	if (/BREAKING[\-\s]CHANGE:\s\S+/.test(input)) {
		breaking = true;
	}

	return {
		breaking,
		type,
		description,
		scope,
		pullRequest,
		fixesIssues,
	};
}
