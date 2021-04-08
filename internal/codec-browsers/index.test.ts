import {
	browserQueryParser,
	parseBrowserQuery,
} from "@internal/codec-browsers/parse";
import {createFixtureTests} from "@internal/test-helpers";
import {resolveTargets} from "@internal/codec-browsers/resolve";
import {getDiagnosticsFromError} from "@internal/diagnostics";
import {printDiagnosticsToString} from "@internal/cli-diagnostics";

const promise = createFixtureTests(async (fixture, t) => {
	const {files} = fixture;
	const inputFile = files.assert("input.txt");

	const filename = inputFile.relative;

	const inputContent = inputFile.contentAsText();

	const parser = browserQueryParser.create({
		input: inputContent,
		path: filename,
	});

	const outputFile = inputFile.absolute.getParent().append(
		inputFile.absolute.getExtensionlessBasename(),
	).join();

	const snapshot = t.customSnapshot(outputFile);

	snapshot.named("tokens", parser.getAllTokens());

	try {
		const parsed = parseBrowserQuery({
			input: inputContent,
			path: filename,
			includeSourceTextInDiagnostics: true,
		});

		const result = Array.from(
			resolveTargets(parsed, {fixedDate: new Date(1_616_056_455_400)}),
		).map((browser) => `${browser.getId()}:${browser.getVersion()}`);

		snapshot.named("targets", parsed);

		snapshot.named("result", result);
	} catch (err) {
		const diagnostics = getDiagnosticsFromError(err);
		if (diagnostics === undefined) {
			snapshot.named("error", err);
		} else {
			const printed = await printDiagnosticsToString({
				diagnostics,
				suppressions: [],
			});
			snapshot.named("diagnostics", printed);
		}
	}
});

// @ts-ignore Doesn't support top-level await
await promise;
