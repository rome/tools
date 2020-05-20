import {createDiagnosticsCategory} from "./index";
import {markup} from "@romejs/string-markup";

// @romejs/core/master/commands/lint.ts
export const lintCommand = createDiagnosticsCategory({
	INVALID_DECISION_ACTION: (action: string) => ({
		message: markup`<emphasis>${action}</emphasis> is not a valid decision action`,
	}),
	INVALID_DECISION_PART_COUNT: (i: number) => ({
		message: `Segment ${i} contains an invalid number of decision parts`,
	}),
});
