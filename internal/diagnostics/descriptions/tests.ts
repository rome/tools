import {createDiagnosticsCategory} from "./index";
import {DiagnosticAdvice} from "../types";
import {markup} from "@internal/markup";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";

export const tests = createDiagnosticsCategory({
	CANCELLED: {
		category: DIAGNOSTIC_CATEGORIES["tests/cancelled"],
		message: markup`Test was cancelled`,
	},
	UNDECLARED: {
		message: markup`No tests declared in this file`,
		category: DIAGNOSTIC_CATEGORIES["tests/empty"],
	},
	LOGS: (advice: DiagnosticAdvice[]) => ({
		message: markup`Test file produced console logs`,
		category: DIAGNOSTIC_CATEGORIES["tests/logs"],
		advice: [
			...advice,
			{
				type: "log",
				category: "info",
				text: markup`Only visible when this test file contains failures`,
			},
		],
	}),
});
