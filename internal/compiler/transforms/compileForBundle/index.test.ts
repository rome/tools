import {
	createFixtureTests,
	createMockWorker,
	findFixtureInput,
} from "@internal/test-helpers";
import {printDiagnosticsToString} from "@internal/cli-diagnostics";
import {createDefaultProjectConfig} from "@internal/project";
import {resolveBrowsers} from "@internal/codec-browsers";
import {createUIDPath, UIDPathMap} from "@internal/path";

const promise = createFixtureTests(async (fixture, t) => {
	const {worker, performFileOperation, addProject} = createMockWorker();
	const {input, handler} = findFixtureInput(fixture, undefined);

	const filename = input.relative;
	const content = input.contentAsText();

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
			return await worker.api.compile(ref, "compileForBundle", {target: "default", bundle: {
					assetPath: input.relative,
					__filename: input.relative,
					moduleAll: true,
					moduleId: createUIDPath(input.relative.join()),
					relativeSourcesToModuleId: new Map(),
					resolvedImports: new UIDPathMap()
				}}, {
				sourceTypeJS: "module"
			});
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

// @ts-expect-error allow top level await
await promise;
