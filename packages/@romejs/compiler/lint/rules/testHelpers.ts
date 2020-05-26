/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TestHelper} from "rome";
import lint from "../index";
import {parseJS} from "@romejs/js-parser";
import {createUnknownFilePath} from "@romejs/path";
import {DEFAULT_PROJECT_CONFIG} from "@romejs/project";
import {ConstProgramSyntax, ConstSourceType} from "@romejs/ast";
import {DiagnosticCategory} from "@romejs/diagnostics";
import {printDiagnosticsToString} from "@romejs/cli-diagnostics";

type TestLintOptions = {
	category: undefined | DiagnosticCategory;
	sourceType?: ConstSourceType;
	syntax?: Array<ConstProgramSyntax>;
};

type TestLintInput = {
	valid?: Array<string>;
	invalid?: Array<string>;
};

export async function testLintMultiple(
	t: TestHelper,
	inputs: Array<string>,
	opts: TestLintOptions,
) {
	for (const input of inputs) {
		await testLint(t, input, opts);
	}
}

export async function testLint(
	t: TestHelper,
	input: string | TestLintInput,
	opts: TestLintOptions,
) {
	if (typeof input === "string") {
		return await testLintExpect(t, input, opts);
	}

	const {invalid, valid} = input;

	if (invalid !== undefined) {
		for (input of invalid) {
			await testLintExpect(t, input, opts, "INVALID");
		}
	}
	if (valid !== undefined) {
		for (input of valid) {
			await testLintExpect(t, input, opts, "VALID");
		}
	}
}

async function testLintExpect(
	t: TestHelper,
	input: string,
	{syntax = ["jsx", "ts"], category, sourceType = "module"}: TestLintOptions,
	expect?: "INVALID" | "VALID",
) {
	t.addToAdvice({
		type: "log",
		category: "info",
		text: "Lint options",
	});

	t.addToAdvice({
		type: "inspect",
		data: {
			category,
			syntax,
			sourceType,
		},
	});

	t.addToAdvice({
		type: "log",
		category: "info",
		text: "Input",
	});

	t.addToAdvice({
		type: "code",
		code: input,
	});

	const ast = parseJS({
		input,
		sourceType,
		path: createUnknownFilePath("unknown"),
		syntax,
	});

	const res = await lint({
		applyFixes: true,
		options: {},
		ast,
		sourceText: input,
		project: {
			folder: undefined,
			config: DEFAULT_PROJECT_CONFIG,
		},
	});

	const diagnostics = res.diagnostics.filter((diag) => {
		return diag.description.category === category;
	}).map((diag) => {
		return {
			...diag,
			location: {
				...diag.location,
				sourceText: input,
			},
		};
	});

	if (expect === "INVALID") {
		t.true(diagnostics.length > 0, "Expected test to have diagnostics.");
	}
	if (expect === "VALID") {
		t.is(diagnostics.length, 0, "Expected test not to have diagnostics.");
	}

	const snapshotId = t.getNextSnapshotId();

	let entryName = String(snapshotId);
	if (expect !== undefined) {
		entryName += ` (${expect})`;
	}

	t.namedSnapshot(
		entryName,
		printDiagnosticsToString({
			diagnostics,
			suppressions: res.suppressions,
		}),
	);

	// const fixable = diagnostics.every((d) => d.fixable);

	let formattedEntryName = `${snapshotId}: formatted`;
	// if (diagnostics.length !== 0) {
	// 	formattedEntryName += fixable ? " and fixed" : " but unfixable";
	// }

	t.namedSnapshot(formattedEntryName, res.src);

	t.clearAdvice();
}
