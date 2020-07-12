import {createDiagnosticsCategory} from "./index";
import {markup} from "@romefrontend/string-markup";

// @romefrontend/codec-js-manifest
export const manifest = createDiagnosticsCategory({
	TOO_MANY_HASH_PARTS: "Too many hashes",
	MISSING_HOSTED_GIT_USER: "Missing user",
	MISSING_HOSTED_GIT_REPO: "Missing repo",
	TOO_MANY_HOSTED_GIT_PARTS: "Expected only 2 parts",
	EMPTY_NPM_PATTERN: "Missing rest of npm dependency pattern",
	TOO_MANY_NPM_PARTS: "Too many @ signs",
	STRING_BIN_WITHOUT_NAME: "A string bin is only allowed if the manifest has a name property",
	MISSING_REPO_URL: "Missing repo URL",
	MIXED_EXPORTS_PATHS: "Cannot mix a root conditional export with relative paths",
	NAME_EXCEEDS: "cannot exceed 214 characters",
	INVALID_NAME_START: "cannot start with a dot or underscore",
	ORG_WITH_NO_PACKAGE_NAME: "contains an org but no package name",
	ORG_TOO_MANY_PARTS: "contains too many name separators",
	REDUNDANT_ORG_NAME_START: "Redundant <emphasis>@</emphasis> in org name",
	INVALID_NAME_CHAR: (char: string) => ({
		message: markup`The character <emphasis>${char}</emphasis> isn't allowed`,
	}),
	INCORRECT_CASING: (typoKey: string, correctKey: string) => ({
		message: `${typoKey} has incorrect casing, should be ${correctKey}`,
	}),
	INCORRECT_CAMEL_CASING: (typoKey: string, correctKey: string) => ({
		message: `${typoKey} isn't correctly camel cased when it should be ${correctKey}`,
	}),
	TYPO: (typoKey: string, correctKey: string) => ({
		message: `${typoKey} is a typo of ${correctKey}`,
	}),
});
