/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TestHelper} from "rome";
import lint from "../index";
import {parseJS} from "@romejs/js-parser";
import {UnknownFilePath, createUnknownFilePath} from "@romejs/path";
import {createDefaultProjectConfig} from "@romejs/project";
import {ConstProgramSyntax, ConstSourceType} from "@romejs/ast";
import {DiagnosticCategory, DiagnosticsProcessor} from "@romejs/diagnostics";
import {printDiagnosticsToString} from "@romejs/cli-diagnostics";

type TestLintOptions = {
	category: undefined | DiagnosticCategory;
	sourceType?: ConstSourceType;
	syntax?: Array<ConstProgramSyntax>;
	path?: UnknownFilePath | string;
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
		applyFixes: true,
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
	);

	t.namedSnapshot(`${snapshotName}: formatted`, res.src);

	t.clearAdvice();
}
