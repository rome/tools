import {
	createFixtureTests,
	createMockWorker,
	findFixtureInput,
} from "@internal/test-helpers";
import {printDiagnosticsToString} from "@internal/cli-diagnostics";
import {createDefaultProjectConfig} from "@internal/project";
import {resolveBrowsers} from "@internal/codec-browsers";
import {removeCarriageReturn} from "@internal/string-utils";
import {decodeUTF8} from "@internal/binary";

const promise = createFixtureTests(async (fixture, t) => {
	const {worker, performFileOperation, addProject} = createMockWorker();
	const {input, handler} = findFixtureInput(fixture, undefined);

	const filename = input.relative;
	const content = removeCarriageReturn(decodeUTF8(input.content));

	const res = await performFileOperation(
		{
			uid: filename.join(),
			sourceText: content,
			project: addProject({
				...createDefaultProjectConfig(),
				targets: new Map([
					[
						"default",
						resolveBrowsers(">0%").map((browser) => ({
							name: browser.getId(),
							version: browser.getVersion(),
						})),
					],
				]),
			}),
		},
		async (ref) => {
			return await worker.api.compile(ref, "compile", {target: "default"}, {});
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
