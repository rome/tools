import {createDiagnosticsCategory} from "./index";
import {DiagnosticAdvice} from "../types";

export const tests = createDiagnosticsCategory({
	CANCELLED: {
		category: "tests/cancelled",
		message: "Test was cancelled",
	},
	UNDECLARED: {
		message: "No tests declared in this file",
		category: "tests/noneDeclared",
	},
	LOGS: (advice: DiagnosticAdvice) => ({
		message: "Test file produced console logs",
		category: "tests/logs",
		advice: [
			...advice,
			{
				type: "log",
				category: "info",
				text: "Only visible when this test file contains failures",
			},
		],
	}),
});
