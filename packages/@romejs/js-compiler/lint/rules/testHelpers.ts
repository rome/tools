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
import {ConstProgramSyntax, ConstSourceType} from "@romejs/js-ast";
import {DiagnosticCategory} from "@romejs/diagnostics";
import {printDiagnosticsToString} from "@romejs/cli-diagnostics";

type TestLintOptions = {
	category: undefined | DiagnosticCategory;
	sourceType?: ConstSourceType;
	syntax?: Array<ConstProgramSyntax>;
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
	input: string,
	{syntax = ["jsx", "ts"], category, sourceType = "module"}: TestLintOptions,
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

	const snapshotName = t.snapshot(
		printDiagnosticsToString({
			diagnostics,
			suppressions: res.suppressions,
		}),
	);

	t.namedSnapshot(`${snapshotName}: formatted`, res.src);

	t.clearAdvice();
}
