import {
	createFixtureTests,
	createMockWorker,
	findFixtureInput,
} from "@internal/test-helpers";
import {printDiagnosticsToString} from "@internal/cli-diagnostics";

const promise = createFixtureTests(async (fixture, t) => {
	const {worker, performFileOperation} = createMockWorker();
	const {input, handler} = findFixtureInput(fixture, undefined);

	const filename = input.relative;
	const content = input.contentAsText();

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

	const snapshot = t.customSnapshot(snapshotFile, {language: handler.language});

	snapshot.named("Input", content);

	snapshot.named("Output", res.value.compiledCode);

	snapshot.named(
		"Diagnostics",
		await printDiagnosticsToString({
			diagnostics: res.value.diagnostics,
			suppressions: res.value.suppressions,
		}),
	);
});

// @ts-ignore allow top level await
await promise;
