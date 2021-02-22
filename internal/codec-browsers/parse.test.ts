import {browserQueryParser, parseBrowserQuery} from "@internal/codec-browsers/parse";
import {createFixtureTests} from "@internal/test-helpers";
import {removeCarriageReturn} from "@internal/string-utils";

const promise = createFixtureTests(async (fixture, t) => {
	const {files} = fixture;
	const inputFile = files.assert("input.txt");

	const filename = inputFile.relative;

	const inputContent = removeCarriageReturn(inputFile.content.toString());

	const parser = browserQueryParser.create({
		input: inputContent,
		path: filename,
	});

	const ast = parseBrowserQuery({
		input: inputContent,
		path: filename,
	});

	const outputFile = inputFile.absolute.getParent().append(
		inputFile.absolute.getExtensionlessBasename(),
	).join();

	t.namedSnapshot("tokens", parser.getAllTokens(), undefined, {filename: outputFile});

	t.namedSnapshot("diagnostics", parser.getDiagnostics(), undefined, {filename: outputFile});

	t.namedSnapshot("ast", ast, undefined, {filename: outputFile});
});

// @ts-ignore Doesn't support top-level await
await promise;
