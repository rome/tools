import {
	createFixtureTests,
	createMockWorker,
	findFixtureInput,
} from "@internal/test-helpers";
import {removeCarriageReturn} from "@internal/string-utils";
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
	const content = removeCarriageReturn(input.content.toString());

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
