import {createFixtureTests} from "@internal/test-helpers";
import {removeCarriageReturn} from "@internal/string-utils";
import {parseCommit} from "./index";

const promise = createFixtureTests(async (fixture, t) => {
	const {files} = fixture;
	const inputFile = files.assert("input.txt");

	const filename = inputFile.relative;

	const inputContent = removeCarriageReturn(inputFile.content.toString());

	const ast = parseCommit({
		input: inputContent,
		path: filename,
	});

	const outputFile = inputFile.absolute.getParent().append(
		inputFile.absolute.getExtensionlessBasename(),
	).join();

	t.snapshot(ast, undefined, {filename: outputFile});
});

// @ts-ignore Doesn't support top-level await
await promise;
