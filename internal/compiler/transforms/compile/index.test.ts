import {
	createFixtureTests,
	createMockWorker,
	findFixtureInput,
} from "@internal/test-helpers";
import {removeCarriageReturn} from "@internal/string-utils";
import {printDiagnosticsToString} from "@internal/cli-diagnostics";
import {decodeUTF8} from "@internal/binary";

const promise = createFixtureTests(async (fixture, t) => {
	const {worker, performFileOperation} = createMockWorker();
	const {input, handler} = findFixtureInput(fixture, undefined);

	const filename = input.relative;
	const content = removeCarriageReturn(decodeUTF8(input.content));

	const res = await performFileOperation(
		{
			uid: filename.join(),
			sourceText: content,
		},
		async (ref) => {
			return await worker.api.compile(ref, "compile", {}, {});
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
		res.value.compiledCode,
		undefined,
		{
			filename: snapshotFile,
			language: handler.language,
		},
	);

	t.namedSnapshot(
		"Diagnostics",
		await printDiagnosticsToString({
			diagnostics: res.value.diagnostics,
			suppressions: res.value.suppressions,
		}),
		undefined,
		{
			filename: snapshotFile,
		},
	);
});

// @ts-ignore allow top level await
await promise;
