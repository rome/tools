/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TestHelper} from "rome";
import lint from "../index";
import {parseJS} from "@romefrontend/js-parser";
import {UnknownFilePath, createUnknownFilePath} from "@romefrontend/path";
import {createDefaultProjectConfig} from "@romefrontend/project";
import {ConstJSProgramSyntax, ConstJSSourceType} from "@romefrontend/ast";
import {
	DiagnosticCategory,
	DiagnosticsProcessor,
} from "@romefrontend/diagnostics";
import {printDiagnosticsToString} from "@romefrontend/cli-diagnostics";

type TestLintOptions = {
	category: undefined | DiagnosticCategory;
	sourceType?: ConstJSSourceType;
	syntax?: Array<ConstJSProgramSyntax>;
	path?: UnknownFilePath | string;
	snapshotFilename?: string;
};

type TestLintInput = {
	valid?: Array<string>;
	invalid?: Array<string>;
};

export async function testLint(
	t: TestHelper,
	{invalid = [], valid = []}: TestLintInput,
	opts: TestLintOptions,
) {
	for (const input of invalid) {
		await testLintExpect(t, input, opts, false);
	}
	for (const input of valid) {
		await testLintExpect(t, input, opts, true);
	}
}

async function testLintExpect(
	t: TestHelper,
	input: string,
	{
		syntax = ["jsx", "ts"],
		category,
		sourceType = "module",
		path = createUnknownFilePath("unknown"),
		snapshotFilename,
	}: TestLintOptions,
	expectValid: boolean,
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
		path,
		syntax,
	});

	const res = await lint({
		applyRecommendedFixes: true,
		options: {},
		ast,
		sourceText: input,
		project: {
			folder: undefined,
			config: createDefaultProjectConfig(),
		},
	});

	const processor = new DiagnosticsProcessor();
	processor.normalizer.setInlineSourceText("unknown", input);
	processor.addFilter({
		test: (diag) => diag.description.category === category,
	});
	processor.addDiagnostics(res.diagnostics);

	const diagnostics = processor.getDiagnostics();

	if (expectValid === false) {
		t.true(diagnostics.length > 0, "Expected test to have diagnostics.");
	}

	if (expectValid === true) {
		t.is(diagnostics.length, 0, "Expected test not to have diagnostics.");
	}

	const snapshotName = t.snapshot(
		printDiagnosticsToString({
			diagnostics,
			suppressions: res.suppressions,
		}),
		undefined,
		{filename: snapshotFilename},
	);

	t.namedSnapshot(
		`${snapshotName}: formatted`,
		res.src,
		undefined,
		{filename: snapshotFilename},
	);

	t.clearAdvice();
}
