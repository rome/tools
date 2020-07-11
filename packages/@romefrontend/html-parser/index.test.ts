import {parseHTML} from "@romefrontend/html-parser";
import {createFixtureTests} from "@romefrontend/test-helpers";
import {removeCarriageReturn} from "@romefrontend/string-utils";

const promise = createFixtureTests(async (fixture, t) => {
	const {files} = fixture;

	const inputFile = files.get("input.html");
	if (inputFile === undefined) {
		throw new Error(`The fixture ${fixture.dir} did not have an input.html`);
	}

	const filename = inputFile.relative;

	const inputContent = removeCarriageReturn(inputFile.content.toString());

	const ast = parseHTML({
		inlineDiagnosticsSource: true,
		input: inputContent,
		path: filename,
	});

	const outputFile = inputFile.absolute.getParent().append(
		inputFile.absolute.getExtensionlessBasename(),
	).join();
	t.snapshot(ast, undefined, {filename: outputFile});
});

// @ts-ignore Doesn't support top-level await lol
await promise;
