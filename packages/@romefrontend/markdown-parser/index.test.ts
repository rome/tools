import {parseMarkdown} from "@romefrontend/markdown-parser";
import {createFixtureTests} from "@romefrontend/test-helpers";
import {removeCarriageReturn} from "@romefrontend/string-utils";

const promise = createFixtureTests(async (fixture, t) => {
	const {files} = fixture;

	const inputFile = files.get("input.md");
	if (inputFile === undefined) {
		throw new Error(`The fixture ${fixture.dir} did not have an input.md`);
	}

	const filename = inputFile.relative;

	const inputContent = removeCarriageReturn(inputFile.content.toString());

	const ast = parseMarkdown({
		input: inputContent,
		path: filename,
	});

	const outputFile = inputFile.absolute.getParent().append(
		inputFile.absolute.getExtensionlessBasename(),
	).join();

	// console.log('output', outputFile);

	// throw new Error();
	t.snapshot(ast, undefined, {filename: outputFile});
});

// @ts-ignore Doesn't support top-level await lol
await promise;
