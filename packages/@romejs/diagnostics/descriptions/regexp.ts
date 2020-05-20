import {createDiagnosticsCategory} from "./index";

// @romejs/codec-js-regexp
export const regexp = createDiagnosticsCategory({
	INVALID_CAPTURE_GROUP_MODIFIER: "Invalid capture group modifier",
	UNCLOSED_GROUP: "Unclosed group",
	UNOPENED_GROUP: "Unopened group",
	INVALID_QUANTIFIER_TARGET: "Invalid target for quantifier",
	UNKNOWN_REGEX_PART: "Unknown regex part",
	REVERSED_CHAR_SET_RANGE: "Range values reversed. Start char code is greater than end char code",
	UNCLOSED_CHAR_SET: "Unclosed character set",
	DUPLICATE_FLAG: "Duplicate regular expression flag",
	INVALID_FLAG: "Invalid regular expression flag",
	REVERSED_QUANTIFIER_RANGE: "Quantifier minimum is greater than maximum",
	NO_TARGET_QUANTIFIER: "Nothing to repeat",
	INVALID_NAMED_CAPTURE: "Invalid named capture referenced",
	UNCLOSED_NAMED_CAPTURE: "Unclosed named capture",
});
