import {createDiagnosticsCategory} from "./index";
import {markup} from "@internal/markup";

// @internal/codec-js-manifest
export const manifest = createDiagnosticsCategory({
	TOO_MANY_HASH_PARTS: {message: markup`Too many hashes`},
	MISSING_HOSTED_GIT_USER: {message: markup`Missing user`},
	MISSING_HOSTED_GIT_REPO: {message: markup`Missing repo`},
	TOO_MANY_HOSTED_GIT_PARTS: {message: markup`Expected only 2 parts`},
	EMPTY_NPM_PATTERN: {message: markup`Missing rest of npm dependency pattern`},
	TOO_MANY_NPM_PARTS: {message: markup`Too many @ signs`},
	STRING_BIN_WITHOUT_NAME: {
		message: markup`A string bin is only allowed if the manifest has a name property`,
	},
	MISSING_REPO_URL: {message: markup`Missing repo URL`},
	MIXED_EXPORTS_PATHS: {
		message: markup`Cannot mix a root conditional export with relative paths`,
	},
	NAME_EXCEEDS: {message: markup`cannot exceed 214 characters`},
	INVALID_NAME_START: {message: markup`cannot start with a dot or underscore`},
	ORG_WITH_NO_PACKAGE_NAME: {
		message: markup`contains an org but no package name`,
	},
	ORG_TOO_MANY_PARTS: {message: markup`contains too many name separators`},
	REDUNDANT_ORG_NAME_START: {
		message: markup`Redundant <emphasis>@</emphasis> in org name`,
	},
	INVALID_NAME_CHAR: (char: string) => ({
		message: markup`The character <emphasis>${char}</emphasis> isn't allowed`,
	}),
	INCORRECT_CASING: (typoKey: string, correctKey: string) => ({
		message: markup`${typoKey} has incorrect casing, should be ${correctKey}`,
	}),
	INCORRECT_CAMEL_CASING: (typoKey: string, correctKey: string) => ({
		message: markup`${typoKey} isn't correctly camel cased when it should be ${correctKey}`,
	}),
	TYPO: (typoKey: string, correctKey: string) => ({
		message: markup`${typoKey} is a typo of ${correctKey}`,
	}),
	UNSUPPORTED_DEPENDENCY_PATTERN_PREFIX: (prefix: string) => ({
		message: markup`Unsupported dependency pattern prefix <emphasis>${prefix}</emphasis>`,
	}),
});
