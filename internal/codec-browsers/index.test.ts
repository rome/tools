import {
	browserQueryParser,
	parseBrowserQuery,
} from "@internal/codec-browsers/parse";
import {createFixtureTests} from "@internal/test-helpers";
import {removeCarriageReturn} from "@internal/string-utils";
import {resolveTargets} from "@internal/codec-browsers/resolve";
import {getDiagnosticsFromError} from "@internal/diagnostics";
import {printDiagnosticsToString} from "@internal/cli-diagnostics";
import {decodeUTF8} from "@internal/binary";

const promise = createFixtureTests(async (fixture, t) => {
	const {files} = fixture;
	const inputFile = files.assert("input.txt");

	const filename = inputFile.relative;

	const inputContent = removeCarriageReturn(decodeUTF8(inputFile.content));

	const parser = browserQueryParser.create({
		input: inputContent,
		path: filename,
	});

	const outputFile = inputFile.absolute.getParent().append(
		inputFile.absolute.getExtensionlessBasename(),
	).join();

	t.namedSnapshot(
		"tokens",
		parser.getAllTokens(),
		undefined,
		{filename: outputFile},
	);

	try {
		const parsed = parseBrowserQuery({
			input: inputContent,
			path: filename,
			includeSourceTextInDiagnostics: true,
		});

		const result = Array.from(resolveTargets(parsed)).map((browser) =>
			`${browser.getId()}:${browser.getVersion()}`
		);

		t.namedSnapshot("targets", parsed, undefined, {filename: outputFile});

		t.namedSnapshot("result", result, undefined, {filename: outputFile});
	} catch (err) {
		const diagnostics = getDiagnosticsFromError(err);
		if (diagnostics === undefined) {
			t.namedSnapshot("error", err, undefined, {filename: outputFile});
		} else {
			const printed = await printDiagnosticsToString({
				diagnostics,
				suppressions: [],
			});
			t.namedSnapshot("diagnostics", printed, undefined, {filename: outputFile});
		}
	}
});

// @ts-ignore Doesn't support top-level await
await promise;
