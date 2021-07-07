import {Consumer} from "@internal/consume";
import {descriptions} from "@internal/diagnostics";

export type PathAliasPattern = {
	parts: string[];
	wildcard: boolean;
};

export function consumePathAliasPattern(
	consumer: Consumer,
	alreadyConsumed?: string,
): PathAliasPattern {
	const path = alreadyConsumed || consumer.asString();

	if (path.length === 0) {
		throw consumer.unexpected(descriptions.PROJECT_CONFIG.EMPTY_PATTERN);
	}

	const wildcards = path.match(/\*/g) || [];
	if (wildcards.length > 1) {
		throw consumer.unexpected(
			descriptions.PROJECT_CONFIG.TOO_MANY_WILDCARDS(path),
		);
	}

	return {
		parts: path.split("*"),
		wildcard: wildcards.length === 1,
	};
}

export function buildPathFromAliasPattern(
	path: string,
	pattern: PathAliasPattern,
): string {
	const [prefix, suffix] = pattern.parts;
	if (pattern.wildcard) {
		return prefix + path + suffix;
	}
	return prefix;
}

export function matchAliasPattern(
	path: string,
	pattern: PathAliasPattern,
): string | undefined {
	const [prefix, suffix] = pattern.parts;
	if (pattern.wildcard && path.startsWith(prefix) && path.endsWith(suffix)) {
		return path.slice(prefix.length, path.length - suffix.length);
	}

	if (path === prefix) {
		return path;
	}

	return undefined;
}

export function aliasPatternToString(pattern: PathAliasPattern): string {
	const [prefix, suffix] = pattern.parts;
	if (pattern.wildcard) {
		return `${prefix}*${suffix}`;
	}
	return prefix;
}
