import {
	browserQueryParser,
	parseBrowserQuery,
} from "@internal/codec-browsers/parse";
import {createFixtureTests} from "@internal/test-helpers";
import {removeCarriageReturn} from "@internal/string-utils";
import {resolveTargets} from "@internal/codec-browsers/resolve";

const promise = createFixtureTests(async (fixture, t) => {
	const {files} = fixture;
	const inputFile = files.assert("input.txt");

	const filename = inputFile.relative;

	const inputContent = removeCarriageReturn(inputFile.content.toString());

	const parser = browserQueryParser.create({
		input: inputContent,
		path: filename,
	});

	const outputFile = inputFile.absolute.getParent().append(
		inputFile.absolute.getExtensionlessBasename(),
	).join();

	try {
		const parsed = parseBrowserQuery({
			input: inputContent,
			path: filename,
		});

		const result = Array.from(resolveTargets(parsed)).map((browser) =>
			`${browser.getId()}:${browser.getVersion()}`
		);

		t.namedSnapshot(
			"tokens",
			parser.getAllTokens(),
			undefined,
			{filename: outputFile},
		);

		t.namedSnapshot("targets", parsed, undefined, {filename: outputFile});

		t.namedSnapshot("result", result, undefined, {filename: outputFile});
	} catch (error) {
		t.namedSnapshot("error", error, undefined, {filename: outputFile});
	}
});

// @ts-ignore Doesn't support top-level await
await promise;
