import {parseCSS} from "@internal/css-parser";
import {createFixtureTests} from "@internal/test-helpers";

const promise = createFixtureTests(async (fixture, t) => {
	const {files} = fixture;
	const inputFile = files.assert("input.css");

	const filename = inputFile.relative;

	const inputContent = inputFile.contentAsText();

	const ast = parseCSS({
		input: inputContent,
		path: filename,
	});

	const outputFile = inputFile.absolute.getParent().append(
		inputFile.absolute.getExtensionlessBasename(),
	).join();

	t.customSnapshot(outputFile).snapshot(ast);
});

// @ts-ignore Doesn't support top-level await
await promise;
