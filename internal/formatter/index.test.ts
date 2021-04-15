import {
	createFixtureTests,
	createMockWorker,
	findFixtureInput,
} from "@internal/test-helpers";
import {test} from "rome";
import {printDiagnosticsToString} from "@internal/cli-diagnostics";
import {formatAST} from "@internal/formatter/index";
import {template} from "@internal/js-ast-utils";
import {createDefaultProjectConfig} from "@internal/project";

test(
	"space indent",
	(t) => {
		const config = createDefaultProjectConfig();

		const formatted = formatAST(
			template.root`if (foo) {bar;}`,
			{
				projectConfig: {
					...config,
					format: {
						...config.format,
						indentStyle: "space",
						indentSize: 2,
					},
				},
			},
		);

		t.inlineSnapshot(formatted.code, "if (foo) {\n  bar;\n}\n");
	},
);

const promise = createFixtureTests(async (fixture, t) => {
	const {options} = fixture;

	const {worker, performFileOperation} = createMockWorker();
	const {input, handler} = findFixtureInput(fixture, undefined);

	const filename = input.relative;
	const format = options.get("format").asStringSetOrVoid(["pretty", "compact"]);
	const content = await input.readAsText();

	const res = await performFileOperation(
		{
			uid: filename.join(),
			sourceText: content,
		},
		async (ref) => {
			return await worker.api.format(ref, {format, forceFormat: true}, {});
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

	snapshot.named("Output", res.formatted);

	snapshot.named(
		"Diagnostics",
		await printDiagnosticsToString({
			diagnostics: res.diagnostics,
			suppressions: res.suppressions,
		}),
		{
			language: undefined,
		},
	);
});

// @ts-ignore allow top level await
await promise;
