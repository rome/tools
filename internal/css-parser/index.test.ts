import {parseCSS} from "@internal/css-parser";
import {FixtureFile, createFixtureTests} from "@internal/test-helpers";
import {removeCarriageReturn} from "@internal/string-utils";
import {decodeUTF8} from "@internal/binary";
import {TestHelper} from "rome";
import {CSSRoot} from "@internal/ast";

const promise = createFixtureTests(async (fixture, t) => {
	const {files} = fixture;
	if (files.has("input.css")) {
		const inputFile = files.assert("input.css");
		const ast = parseFile(inputFile);
		t.is(ast.diagnostics.length, 0, "Expected test not to have diagnostics.");
		doSnapshot(t, inputFile, ast);
	} else {
		const inputFile = files.assert("invalid.css");
		const ast = parseFile(inputFile);
		t.true(ast.diagnostics.length > 0, "Expected test to have diagnostics.");
		doSnapshot(t, inputFile, ast);
	}
});

function parseFile(inputFile: FixtureFile) {
	const filename = inputFile.relative;

	const inputContent = removeCarriageReturn(decodeUTF8(inputFile.content));

	return parseCSS({
		input: inputContent,
		path: filename,
	});
}

function doSnapshot(t: TestHelper, inputFile: FixtureFile, ast: CSSRoot) {
	const outputFile = inputFile.absolute.getParent().append(
		inputFile.absolute.getExtensionlessBasename(),
	).join();

	t.snapshot(ast, undefined, {filename: outputFile});
}

// @ts-expect-error Doesn't support top-level await
await promise;
