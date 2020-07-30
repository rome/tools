import {createDiagnosticsCategory} from "./index";
import {markup} from "@internal/markup";

// @internal/codec-semver
export const semver = createDiagnosticsCategory({
	MISSING_MINOR_VERSION: {
		message: markup`A minor number is required for a version`,
	},
	MISSING_PATCH_VERSION: {
		message: markup`A patch number is required for a version`,
	},
	EXCESSIVE_VERSION_PARTS: {message: markup`Too many parts for version`},
	INVALID_QUANTIFIER_PART: {message: markup`Invalid version qualifier part`},
	WILDCARD_IN_VERSION: {
		message: markup`Wildcard aren't allowed in a hard version`,
	},
	INVALID_VERSION_NUMBER: {
		message: markup`This isn't a valid version part, expected a number`,
	},
	INVALID_RANGE: {
		message: markup`A semver range can only be defined with versions`,
	},
	BARE_PIPE_WITHOUT_LOOSE: {
		message: markup`Bare pipes are only allowed in loose mode`,
	},
	UNEXPECTED_WORD: (word: string) => ({
		message: markup`Unexpected word <emphasis>${word}</emphasis>`,
	}),
	UNKNOWN_START: {message: markup`Unknown start of atom`},
	EXPECTED_VERSION: {message: markup`Unexpected value for version`},
});
