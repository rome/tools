import {
	createFixtureTests,
	createMockWorker,
	findFixtureInput,
} from "@internal/test-helpers";
import {removeCarriageReturn} from "@internal/string-utils";

import {printDiagnosticsToString} from "@internal/cli-diagnostics";

const promise = createFixtureTests(async (fixture, t) => {
	const {options} = fixture;

	const {worker, performFileOperation} = createMockWorker();
	const {input, handler} = findFixtureInput(fixture, undefined);

	const filename = input.relative;
	const format = options.get("format").asStringSetOrVoid(["pretty", "compact"]);
	const content = removeCarriageReturn(input.content.toString());

	const res = await performFileOperation(
		{
			uid: filename.join(),
			sourceText: content,
		},
		async (ref) => {
			return await worker.api.format(ref, {format}, {});
		},
	);
	if (res === undefined) {
		throw new Error("No format value returned");
	}

	const snapshotFile = input.absolute.getParent().append(
		input.absolute.getExtensionlessBasename(),
	).join();

	t.namedSnapshot(
		"Input",
		content,
		undefined,
		{
			filename: snapshotFile,
			language: handler.language,
		},
	);

	t.namedSnapshot(
		"Output",
		res.formatted,
		undefined,
		{
			filename: snapshotFile,
			language: handler.language,
		},
	);

	t.namedSnapshot(
		"Diagnostics",
		await printDiagnosticsToString({
			diagnostics: res.diagnostics,
			suppressions: res.suppressions,
		}),
		undefined,
		{
			filename: snapshotFile,
		},
	);
});

// @ts-ignore allow top level await
await promise;
