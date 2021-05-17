/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TestHelper} from "rome";
import {
	DIAGNOSTIC_CATEGORIES,
	DiagnosticLintCategory,
	DiagnosticsProcessor,
	equalCategoryNames,
	joinCategoryLintName,
} from "@internal/diagnostics";
import {printDiagnosticsToString} from "@internal/cli-diagnostics";
import {IntegrationWorker, createMockWorker} from "@internal/test-helpers";
import {Path, createUIDPath} from "@internal/path";
import {LintRuleName} from "@internal/compiler";

type TestLintOptions = {
	category: DiagnosticLintCategory;
	path: Path;
	snapshotFilename?: string;
	valid?: string[];
	invalid?: string[];
};

export async function testLint(t: TestHelper, opts: TestLintOptions) {
	const int = createMockWorker();
	const {valid = [], invalid = []} = opts;

	let i = 0;

	for (const input of invalid) {
		await testLintExpect(t, int, input, opts, ++i, false);
	}

	for (const input of valid) {
		await testLintExpect(t, int, input, opts, ++i, true);
	}
}

async function testLintExpect(
	t: TestHelper,
	{worker, performFileOperation}: IntegrationWorker,
	input: string,
	{
		category,
		path,
		snapshotFilename,
	}: TestLintOptions,
	index: number,
	expectValid: boolean,
) {
	t.addToAdvice({
		type: "inspect",
		data: {
			filename: path.join(),
			expectValid,
		},
	});

	t.addToAdvice({
		type: "log",
		category: "info",
		text: "Input",
	});

	t.addToAdvice({
		type: "code",
		sourceText: input,
	});

	const uid = createUIDPath(
		`${joinCategoryLintName(category)}/${expectValid ? "pass" : "reject"}/${index}/${path.join()}`,
	);

	const res = await performFileOperation(
		{
			uid,
			sourceText: input,
		},
		async (ref) => {
			return await worker.api.lint(
				ref,
				{
					applySafeFixes: true,
					prefetchedModuleSignatures: {},
					save: true,
					applyLintCategories: [category.slice(1).join("/") as LintRuleName],
				},
				{
					//allowCorrupt: true,
				},
			);
		},
	);

	t.addToAdvice({
		type: "log",
		category: "info",
		text: "Response",
	});

	t.addToAdvice({
		type: "inspect",
		data: res,
	});

	const processor = new DiagnosticsProcessor();
	processor.normalizer.setInlineSourceText(uid, input);
	processor.addEliminationFilter({
		test: (diag) =>
			equalCategoryNames(diag.description.category, category) ||
			equalCategoryNames(diag.description.category, DIAGNOSTIC_CATEGORIES.parse)
		,
	});
	processor.addDiagnostics(res.diagnostics);

	const diagnostics = processor.getDiagnostics();

	if (expectValid) {
		t.is(diagnostics.length, 0, "Expected test not to have diagnostics.");
	} else {
		t.true(diagnostics.length > 0, "Expected test to have diagnostics.");
	}

	const snapshotName = t.snapshot(
		await printDiagnosticsToString({
			diagnostics,
			suppressions: res.suppressions,
			printerOptions: {
				fileHandlers: [
					{
						async exists() {
							return true;
						},
					},
				],
			},
		}),
		undefined,
		{filename: snapshotFilename},
	);

	t.namedSnapshot(
		`${snapshotName}: formatted`,
		res.save?.content,
		undefined,
		{filename: snapshotFilename, language: path.getExtensions().slice(1)},
	);

	t.clearAdvice();
}
