import {createDiagnosticsCategory} from "./index";
import {markup} from "@romejs/string-markup";

// @romejs/codec-semver
export const semver = createDiagnosticsCategory({
	MISSING_MINOR_VERSION: "A minor number is required for a version",
	MISSING_PATCH_VERSION: "A patch number is required for a version",
	EXCESSIVE_VERSION_PARTS: "Too many parts for version",
	INVALID_QUANTIFIER_PART: "Invalid version qualifier part",
	WILDCARD_IN_VERSION: "Wildcard aren't allowed in a hard version",
	INVALID_VERSION_NUMBER: "This isn't a valid version part, expected a number",
	INVALID_RANGE: "A semver range can only be defined with versions",
	BARE_PIPE_WITHOUT_LOOSE: "Bare pipes are only allowed in loose mode",
	UNEXPECTED_WORD: (word: string) => ({
		message: markup`Unexpected word <emphasis>${word}</emphasis>`,
	}),
	UNKNOWN_START: "Unknown start of atom",
	EXPECTED_VERSION: "Unexpected value for version",
});
