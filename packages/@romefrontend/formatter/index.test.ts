import {
	createFixtureTests,
	createIntegrationWorker,
	findFixtureInput,
} from "@romefrontend/test-helpers";
import {removeCarriageReturn} from "@romefrontend/string-utils";

import {printDiagnosticsToString} from "@romefrontend/cli-diagnostics";

const promise = createFixtureTests(async (fixture, t) => {
	const {options} = fixture;

	const {worker, createFileReference} = createIntegrationWorker();
	const {input, handler} = findFixtureInput(fixture, undefined);

	const filename = input.relative;
	const format = options.get("format").asStringSetOrVoid(["pretty", "compact"]);
	const content = removeCarriageReturn(input.content.toString());

	const {ref, teardown} = createFileReference({
		uid: filename.join(),
		sourceText: content,
	});
	const res = await worker.api.format(ref, {format}, {});
	teardown();
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
		printDiagnosticsToString({
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
