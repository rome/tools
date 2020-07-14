import {createDiagnosticsCategory} from "./index";
import stringDiff from "@romefrontend/string-diff";

export const snapshots = createDiagnosticsCategory({
	MISSING_NEWLINE_AFTER_CODE_BLOCK: "Newline required after code block",
	MISSING_NEWLINE_BEFORE_CODE_BLOCK: "Newline required before code block end",
	UNCLOSED_CODE_BLOCK: "Unclosed code block",
	EXPECTED_CODE_BLOCK_AFTER_HEADING: "Expected a code block after this heading",
	REDUNDANT: {
		category: "tests/snapshots/redundant",
		message: "Snapshot should not exist",
	},
	MISSING: {
		category: "tests/snapshots/missing",
		message: "Snapshot does not exist",
	},
	INCORRECT: (expected: string, got: string) => ({
		category: "tests/snapshots/incorrect",
		message: "Snapshots do not match",
		advice: [
			{
				type: "diff",
				language: "unknown",
				diff: stringDiff(expected, got),
			},
		],
	}),
	INLINE_COLLISION: {
		category: "tests/snapshots/inlineCollision",
		message: "Trying to update this inline snapshot multiple times",
		advice: [
			{
				type: "log",
				category: "info",
				text: "<emphasis>t.inlineSnapshot</emphasis> can only be called once. Did you call it in a loop?",
			},
		],
	},
	INLINE_MISSING_RECEIVED: {
		category: "tests/snapshots/inlineMissingReceived",
		message: "This inline snapshot call does not have a received argument",
	},
	INLINE_FROZEN: {
		category: "tests/snapshots/frozen",
		message: "Inline snapshot cannot be updated as snapshots are frozen",
	},
	FROZEN: {
		category: "tests/snapshots/frozen",
		message: "Snapshot cannot be updated as snapshots are frozen",
	},
	INLINE_BAD_MATCH: {
		category: "tests/snapshots/incorrect",
		message: "Inline snapshots do not match",
	},
});
