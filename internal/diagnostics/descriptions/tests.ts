import {createDiagnosticsCategory} from "./index";
import {DiagnosticAdvice} from "../types";
import {markup} from "@internal/markup";

export const tests = createDiagnosticsCategory({
	CANCELLED: {
		category: "tests/cancelled",
		message: markup`Test was cancelled`,
	},
	UNDECLARED: {
		message: markup`No tests declared in this file`,
		category: "tests/noneDeclared",
	},
	LOGS: (advice: DiagnosticAdvice) => ({
		message: markup`Test file produced console logs`,
		category: "tests/logs",
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
