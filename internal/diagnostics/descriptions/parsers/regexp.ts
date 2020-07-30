import {createDiagnosticsCategory} from "../index";
import {markup} from "@internal/markup";

// @internal/codec-js-regexp
export const regexp = createDiagnosticsCategory({
	INVALID_CAPTURE_GROUP_MODIFIER: {
		message: markup`Invalid capture group modifier`,
	},
	UNCLOSED_GROUP: {message: markup`Unclosed group`},
	UNOPENED_GROUP: {message: markup`Unopened group`},
	INVALID_QUANTIFIER_TARGET: {message: markup`Invalid target for quantifier`},
	UNKNOWN_REGEX_PART: {message: markup`Unknown regex part`},
	REVERSED_CHAR_SET_RANGE: {
		message: markup`Range values reversed. Start char code is greater than end char code`,
	},
	UNCLOSED_CHAR_SET: {message: markup`Unclosed character set`},
	DUPLICATE_FLAG: {message: markup`Duplicate regular expression flag`},
	INVALID_FLAG: {message: markup`Invalid regular expression flag`},
	REVERSED_QUANTIFIER_RANGE: {
		message: markup`Quantifier minimum is greater than maximum`,
	},
	NO_TARGET_QUANTIFIER: {message: markup`Nothing to repeat`},
	INVALID_NAMED_CAPTURE: {message: markup`Invalid named capture referenced`},
	UNCLOSED_NAMED_CAPTURE: {message: markup`Unclosed named capture`},
});
